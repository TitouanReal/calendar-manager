use std::{
    cell::{OnceCell, RefCell},
    collections::HashMap,
    sync::{Mutex, MutexGuard},
};

use adw::subclass::prelude::*;
use gtk::{
    gio::{self, BusType, DBusProxy, DBusProxyFlags, ListStore},
    glib::{self, clone, Object},
};
use tracing::{debug, info, warn};
use tsparql::{prelude::SparqlCursorExtManual, Notifier, NotifierEvent, SparqlConnection};

use crate::{
    core::{Calendar, Collection, Resource},
    spawn,
    tsparql_utils::NotifierUtils,
};

mod imp {
    use super::*;

    #[derive(Debug, Default)]
    pub struct Manager {
        read_connection: OnceCell<SparqlConnection>,
        write_connection: OnceCell<DBusProxy>,
        notifier: OnceCell<tsparql::Notifier>,
        resource_pool: OnceCell<Mutex<HashMap<String, Resource>>>,
        collections: OnceCell<ListStore>,
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

            self.collections.get_or_init(ListStore::new::<Collection>);

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

        pub fn collections(&self) -> &ListStore {
            self.collections
                .get()
                .expect("providers should be initialized")
        }

        pub fn resource_pool(&self) -> MutexGuard<'_, HashMap<String, Resource>> {
            self.resource_pool
                .get()
                .expect("resource pool should be initialized")
                .lock()
                .unwrap()
        }

        fn refresh_resources(&self) {
            let collection_cursor = self
                .read_connection()
                .query(
                    "SELECT ?collection ?collection_name
                    FROM ccm:Calendar
                    WHERE {
                        ?collection a ccm:Collection;
                            rdfs:label ?collection_name.
                    }",
                    None::<&gio::Cancellable>,
                )
                .unwrap();

            while let Ok(true) = collection_cursor.next(None::<&gio::Cancellable>) {
                let collection_uri = collection_cursor.string(0).unwrap();
                let collection_name = collection_cursor.string(1).unwrap();
                let collection = Collection::new(&collection_name);

                if let Some(_old_ressource) = self.resource_pool().insert(
                    collection_uri.to_string(),
                    Resource::Collection(collection.clone()),
                ) {
                    warn!("Resource with URI \"{collection_uri}\" existed but has been replaced by the collection \"{collection_name}\"");
                }
                self.collections().insert(0, &collection);

                info!(
                    "Found URI \"{collection_uri}\" associated with collection \"{collection_name}\""
                );

                let calendar_cursor = self
                    .read_connection()
                    .query(
                        &format!(
                            "SELECT ?calendar ?calendar_name
                            FROM ccm:Calendar
                            WHERE {{
                                ?calendar a ccm:Calendar;
                                    rdfs:label ?calendar_name;
                                    ccm:collection {collection_uri}.
                            }}"
                        ),
                        None::<&gio::Cancellable>,
                    )
                    .unwrap();

                while let Ok(true) = calendar_cursor.next(None::<&gio::Cancellable>) {
                    let calendar_uri = calendar_cursor.string(0).unwrap();
                    let calendar_name = calendar_cursor.string(1).unwrap();
                    let calendar = Calendar::new(&calendar_name);
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
                            Resource::Provider(_provider) => {
                                // TODO: Update provider properties
                                // info!("Provider {} updated", provider.name());
                            }
                            Resource::Collection(_collection) => {
                                // TODO: Update collection properties
                                // info!("Collection {} updated", collection.name());
                            }
                            Resource::Calendar(_calendar) => {
                                // TODO: Update calendar properties
                                // info!("Calendar {} updated", calendar.name());
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
                                Resource::Collection(_collection) => {
                                    // TODO
                                }
                                Resource::Calendar(_calendar) => {
                                    // TODO
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

    pub fn collections(&self) -> ListStore {
        self.imp().collections().clone()
    }
}

impl Default for Manager {
    fn default() -> Self {
        Self::new()
    }
}
