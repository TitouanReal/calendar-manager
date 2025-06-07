/* calendar-manager.rs
 *
 * Copyright 2025 Titouan Real
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use std::{
    cell::{OnceCell, RefCell},
    collections::HashMap,
};

use adw::subclass::prelude::*;
use gtk::{
    gio::{self, ListStore},
    glib::{self, clone, Object},
};
use tracing::{error, info};
use tsparql::{prelude::SparqlCursorExtManual, Notifier, NotifierEvent, SparqlConnection};

use crate::{
    core::{Calendar, Resource},
    spawn,
    tsparql_utils::NotifierUtils,
};

mod imp {
    use std::sync::{Mutex, MutexGuard};

    use gtk::gio::{BusType, DBusProxy, DBusProxyFlags};
    use tracing::{debug, warn};

    use crate::core::Provider;

    use super::*;

    #[derive(Debug, Default)]
    pub struct Manager {
        read_connection: OnceCell<SparqlConnection>,
        write_connection: OnceCell<DBusProxy>,
        notifier: OnceCell<tsparql::Notifier>,
        resource_pool: OnceCell<Mutex<HashMap<String, Resource>>>,
        list_store: OnceCell<ListStore>,
        notifier_handler: RefCell<Option<glib::SignalHandlerId>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Manager {
        const NAME: &'static str = "Manager";
        type Type = super::Manager;
        type ParentType = Object;
    }

    impl ObjectImpl for Manager {
        fn constructed(&self) {
            self.parent_constructed();

            self.read_connection.get_or_init(|| {
                SparqlConnection::bus_new("io.gitlab.TitouanReal.CcmRead", None, None).unwrap()
            });

            self.write_connection.get_or_init(|| {
                DBusProxy::for_bus_sync(
                    BusType::Session,
                    DBusProxyFlags::NONE,
                    None,
                    "io.gitlab.TitouanReal.CcmWrite",
                    "/io/gitlab/TitouanReal/CcmWrite/Provider",
                    "io.gitlab.TitouanReal.CcmWrite.Provider",
                    None::<&gio::Cancellable>,
                )
                .unwrap()
            });

            self.notifier
                .get_or_init(|| SparqlConnection::create_notifier(self.read_connection()).unwrap());

            self.resource_pool
                .get_or_init(|| Mutex::new(HashMap::new()));

            self.list_store.get_or_init(ListStore::new::<Calendar>);

            spawn!(clone!(
                #[weak(rename_to = imp)]
                self,
                async move {
                    imp.refresh_resources();
                }
            ));

            self.notifier_handler
                .replace(Some(self.notifier().connect_events(clone!(
                    #[weak(rename_to = imp)]
                    self,
                    move |_notifier: &tsparql::Notifier,
                          _service: Option<&str>,
                          _graph: Option<&str>,
                          events: Vec<NotifierEvent>| {
                        imp.handle_events(events);
                    },
                ))));
        }
    }

    #[gtk::template_callbacks]
    impl Manager {
        fn read_connection(&self) -> &SparqlConnection {
            self.read_connection
                .get()
                .expect("read connection should be initialized")
        }

        fn _write_connection(&self) -> &DBusProxy {
            self.write_connection
                .get()
                .expect("write connection should be initialized")
        }

        fn notifier(&self) -> &Notifier {
            self.notifier.get().expect("notifier should be initialized")
        }

        pub fn list_store(&self) -> &ListStore {
            self.list_store
                .get()
                .expect("list store should be initialized")
        }

        pub fn resource_pool(&self) -> MutexGuard<'_, HashMap<String, Resource>> {
            self.resource_pool
                .get()
                .expect("resource pool should be initialized")
                .lock()
                .unwrap()
        }

        fn refresh_resources(&self) {
            let provider_cursor = self
                .read_connection()
                .query(
                    "SELECT ?provider ?provider_name
                    FROM ccm:Calendar
                    WHERE {
                        ?provider a ccm:Provider;
                            rdfs:label ?provider_name.
                    }",
                    None::<&gio::Cancellable>,
                )
                .unwrap();

            while let Ok(true) = provider_cursor.next(None::<&gio::Cancellable>) {
                let provider_uri = provider_cursor.string(0).unwrap();
                let provider_name = provider_cursor.string(1).unwrap();
                let provider = Provider::new(&provider_name);
                if let Some(_old_ressource) = self.resource_pool().insert(
                    provider_uri.to_string(),
                    Resource::Provider(provider.clone()),
                ) {
                    warn!("Resource with URI \"{provider_uri}\" existed but has been replaced by the provider \"{provider_name}\"");
                }
                info!("Found URI \"{provider_uri}\" associated with provider \"{provider_name}\"");

                let calendar_cursor = self
                    .read_connection()
                    .query(
                        &format!(
                            "SELECT ?calendar ?calendar_name
                            FROM ccm:Calendar
                            WHERE {{
                                ?calendar a ccm:Calendar;
                                    rdfs:label ?calendar_name;
                                    ccm:provider {provider_uri}.
                            }}"
                        ),
                        None::<&gio::Cancellable>,
                    )
                    .unwrap();

                while let Ok(true) = calendar_cursor.next(None::<&gio::Cancellable>) {
                    let calendar_uri = calendar_cursor.string(0).unwrap();
                    let calendar_name = calendar_cursor.string(1).unwrap();
                    let calendar = Calendar::new(&calendar_name);
                    self.list_store().insert(0, &calendar);
                    self.resource_pool().insert(
                        calendar_uri.to_string(),
                        Resource::Calendar(calendar.clone()),
                    );

                    info!(
                        "Found URI \"{calendar_uri}\" associated with calendar \"{calendar_name}\""
                    );
                }
            }
        }

        fn handle_events(&self, events: Vec<NotifierEvent>) {
            debug!("Received {} events", events.len());
            for mut event in events {
                info!("Received event {:?}", event.event_type());
                match event.event_type() {
                    tsparql::NotifierEventType::Create => {
                        let urn = event.urn().unwrap();
                        // TODO: get resource type - don't assume it is a calendar
                        // TODO: Be safer than injecting a string
                        let cursor = self
                            .read_connection()
                            .query(
                                &format!(
                                    "SELECT ?calendar_name
                                    FROM ccm:Calendar
                                    WHERE {{
                                        \"{}\" rdfs:label ?calendar_name.
                                    }}",
                                    urn.as_str()
                                ),
                                None::<&gio::Cancellable>,
                            )
                            .unwrap();

                        // TODO: Handle errors
                        // Maybe trigger a full refresh?
                        match cursor.next(None::<&gio::Cancellable>) {
                            Err(e) => {
                                warn!("Encountered glib error: {}", e);
                            }
                            Ok(false) => {
                                warn!("Resource {urn} was created but is not found in database");
                            }
                            Ok(true) => {
                                let calendar_name = cursor.string(0).unwrap();
                                let calendar = Calendar::new(&calendar_name);
                                self.list_store().insert(0, &calendar);
                                self.resource_pool()
                                    .insert(urn.to_string(), Resource::Calendar(calendar));
                                info!("Calendar {calendar_name} created with uri {urn}");
                            }
                        }
                    }
                    tsparql::NotifierEventType::Update => {
                        let urn = event.urn().unwrap();
                        let resource_pool = self.resource_pool();
                        let resource = match resource_pool.get(urn.as_str()) {
                            Some(resource) => resource,
                            None => {
                                warn!("Resource {urn} was updated but is not found in database");
                                return;
                            }
                        };
                        match resource {
                            Resource::Provider(provider) => {
                                // TODO: Update provider properties
                                info!("Provider {} updated", provider.name());
                            }
                            Resource::Calendar(calendar) => {
                                // TODO: Update calendar properties
                                info!("Calendar {} updated", calendar.name());
                            }
                        }
                    }
                    tsparql::NotifierEventType::Delete => {
                        let urn = event.urn().unwrap();
                        let mut resource_pool = self.resource_pool();
                        match resource_pool.remove(urn.as_str()) {
                            Some(resource) => match resource {
                                Resource::Provider(_provider) => {
                                    // TODO
                                }
                                Resource::Calendar(calendar) => {
                                    match self.list_store().find(&calendar) {
                                        Some(index) => {
                                            self.list_store().remove(index);
                                            info!("Calendar {} removed", calendar.name());
                                        }
                                        None => {
                                            error!("Calendar {} was deleted but is not found in list store", calendar.name());
                                        }
                                    }
                                }
                            },
                            None => {
                                warn!("Resource {urn} was deleted but is not found in database");
                                return;
                            }
                        };
                    }
                    _ => {
                        warn!("Unknown event type: {:?}", event.event_type());
                    }
                }
            }
        }
    }
}

glib::wrapper! {
    pub struct Manager(ObjectSubclass<imp::Manager>);
}

impl Manager {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }

    pub fn model(&self) -> &ListStore {
        self.imp().list_store()
    }
}

impl Default for Manager {
    fn default() -> Self {
        Self::new()
    }
}
