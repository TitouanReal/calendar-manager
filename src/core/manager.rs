use std::{
    cell::{OnceCell, RefCell},
    collections::HashMap,
    sync::{Mutex, MutexGuard},
};

use adw::subclass::prelude::*;
use gtk::{
    gio::{self, BusType, DBusProxy, DBusProxyFlags, ListStore},
    glib::{self, Object, clone},
};
use tracing::{debug, error, info, warn};
use tsparql::{Notifier, NotifierEvent, NotifierEventType, SparqlConnection, prelude::*};

use crate::{
    core::{Calendar, Collection, Event, Provider, Resource, pre_resource::PreResource},
    spawn,
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
        events_handler: RefCell<Option<glib::SignalHandlerId>>,
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

            self.events_handler
                .replace(Some(self.notifier().connect_events(clone!(
                    #[weak(rename_to = imp)]
                    self,
                    move |_notifier: &tsparql::Notifier,
                          _service: Option<&str>,
                          _graph: Option<&str>,
                          events: Vec<NotifierEvent>| {
                        imp.handle_notifier_events(events);
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

        pub(super) fn collections(&self) -> &ListStore {
            self.collections
                .get()
                .expect("providers should be initialized")
        }

        pub(super) fn resource_pool(&self) -> MutexGuard<'_, HashMap<String, Resource>> {
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
                    warn!(
                        "Resource with URI \"{collection_uri}\" existed but has been replaced by the collection \"{collection_name}\""
                    );
                }
                self.collections().insert(0, &collection);

                info!(
                    "Found URI \"{collection_uri}\" associated with collection \"{collection_name}\""
                );

                let calendar_cursor = self
                    .read_connection()
                    .query(
                        &format!(
                            "SELECT ?calendar ?calendar_color ?calendar_name
                            FROM ccm:Calendar
                            WHERE {{
                                ?calendar a ccm:Calendar ;
                                    rdfs:label ?calendar_name ;
                                    ccm:color ?calendar_color ;
                                    ccm:collection {collection_uri}.
                            }}"
                        ),
                        None::<&gio::Cancellable>,
                    )
                    .unwrap();

                while let Ok(true) = calendar_cursor.next(None::<&gio::Cancellable>) {
                    let calendar_uri = calendar_cursor.string(0).unwrap();
                    let calendar_color = match calendar_cursor.string(1).unwrap().parse() {
                        Ok(color) => color,
                        Err(e) => {
                            warn!("Failed to parse calendar color: {}", e);
                            continue;
                        }
                    };
                    let calendar_name = calendar_cursor.string(2).unwrap();

                    let calendar = Calendar::new(&calendar_name, calendar_color);
                    self.resource_pool().insert(
                        calendar_uri.to_string(),
                        Resource::Calendar(calendar.clone()),
                    );
                    collection.add_calendar(&calendar);

                    info!(
                        "Found URI \"{calendar_uri}\" associated with calendar \"{calendar_name}\""
                    );
                }
            }
        }

        fn handle_notifier_events(&self, events: Vec<NotifierEvent>) {
            debug!("Received {} events", events.len());

            let mut resource_pool = self.resource_pool();

            let mut create_events = Vec::new();
            let mut update_events = Vec::new();
            let mut delete_events = Vec::new();

            for mut event in events {
                match event.event_type() {
                    NotifierEventType::Create => {
                        create_events.push(event.urn().unwrap());
                    }
                    NotifierEventType::Update => {
                        update_events.push(event.urn().unwrap());
                    }
                    NotifierEventType::Delete => {
                        delete_events.push(event.urn().unwrap());
                    }
                    _ => {
                        error!("Unknown event type: {:?}", event.event_type());
                    }
                }
            }

            let created_resources = create_events
                .into_iter()
                .map(|uri| PreResource::from_uri(self.read_connection(), &uri).unwrap())
                .collect::<Vec<_>>();

            // Create providers
            for pre_provider in created_resources.iter().filter_map(|pre_resource| {
                if let PreResource::Provider(pre_provider) = pre_resource {
                    Some(pre_provider)
                } else {
                    None
                }
            }) {
                let provider = Provider::new(&pre_provider.name);
                let provider_uri = pre_provider.uri.clone();
                resource_pool.insert(provider_uri, Resource::Provider(provider));

                info!("Created provider {}", pre_provider.uri);
            }

            // Create collections
            for pre_collection in created_resources.iter().filter_map(|pre_resource| {
                if let PreResource::Collection(pre_collection) = pre_resource {
                    Some(pre_collection)
                } else {
                    None
                }
            }) {
                let collection_uri = pre_collection.uri.clone();
                let provider_uri = pre_collection.provider_uri.clone();

                if let Some(Resource::Provider(provider)) = resource_pool.get(&provider_uri) {
                    let collection = Collection::new(&pre_collection.name);
                    provider.add_collection(&collection);
                    resource_pool.insert(collection_uri, Resource::Collection(collection));

                    info!("Created collection {}", pre_collection.uri);
                } else {
                    error!(
                        "Collection {collection_uri} has provider {provider_uri} but it does not exist"
                    );
                }
            }

            // Create calendars
            for pre_calendar in created_resources.iter().filter_map(|pre_resource| {
                if let PreResource::Calendar(pre_calendar) = pre_resource {
                    Some(pre_calendar)
                } else {
                    None
                }
            }) {
                let calendar_uri = pre_calendar.uri.clone();
                let collection_uri = pre_calendar.collection_uri.clone();

                if let Some(Resource::Collection(collection)) = resource_pool.get(&collection_uri) {
                    let calendar = Calendar::new(&pre_calendar.name, pre_calendar.color);
                    collection.add_calendar(&calendar);
                    resource_pool.insert(calendar_uri, Resource::Calendar(calendar));

                    info!("Created calendar {}", pre_calendar.uri);
                } else {
                    error!(
                        "Calendar {calendar_uri} has collection {collection_uri} but it does not exist"
                    );
                }
            }

            // Create events
            for pre_event in created_resources.iter().filter_map(|pre_resource| {
                if let PreResource::Event(pre_event) = pre_resource {
                    Some(pre_event)
                } else {
                    None
                }
            }) {
                let event_uri = pre_event.uri.to_string();
                let calendar_uri = pre_event.calendar_uri.clone();

                if let Some(Resource::Calendar(calendar)) = resource_pool.get(&calendar_uri) {
                    let event = Event::new(&pre_event.name);
                    calendar.add_event(&event);
                    resource_pool.insert(event_uri, Resource::Event(event));

                    info!("Created event {}", pre_event.uri);
                } else {
                    error!("Event {event_uri} has calendar {calendar_uri} but it does not exist");
                }
            }

            let _update_events = update_events
                .into_iter()
                .map(|uri| {
                    let old = self.resource_pool().get(uri.as_str()).unwrap().to_owned();
                    let new = PreResource::from_uri(self.read_connection(), &uri).unwrap();
                    (uri, old, new)
                })
                .collect::<Vec<_>>();
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
