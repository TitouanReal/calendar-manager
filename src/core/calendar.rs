use std::cell::RefCell;

use adw::{prelude::*, subclass::prelude::*};
use gtk::{
    gio,
    glib::{self, Object},
};
use tracing::{info, warn};
use tsparql::{prelude::*, SparqlConnection};

mod imp {
    use super::*;

    #[derive(Debug, Default, glib::Properties)]
    #[properties(wrapper_type = super::Calendar)]
    pub struct Calendar {
        #[property(get, set)]
        collection_uri: RefCell<String>,
        #[property(get, set)]
        name: RefCell<String>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Calendar {
        const NAME: &'static str = "Calendar";
        type Type = super::Calendar;
        type ParentType = Object;
    }

    #[glib::derived_properties]
    impl ObjectImpl for Calendar {}
}

glib::wrapper! {
    pub struct Calendar(ObjectSubclass<imp::Calendar>);
}

impl Calendar {
    pub fn new(collection_uri: &str, name: &str) -> Self {
        glib::Object::builder()
            .property("collection-uri", collection_uri)
            .property("name", name)
            .build()
    }

    /// Retrieves a calendar resource from a URI.
    ///
    /// # Panics
    ///
    /// This function may panic if the given URI is invalid or does not point to a calendar resource.
    pub fn from_uri(read_connection: &SparqlConnection, uri: &str) -> Result<Self, ()> {
        let cursor = read_connection
            .query(
                &format!(
                    "SELECT ?calendar_name ?collection
                    FROM ccm:Calendar
                    WHERE {{
                        \"{}\" rdfs:label ?calendar_name ;
                            ccm:collection ?collection .
                    }}",
                    uri
                ),
                None::<&gio::Cancellable>,
            )
            .unwrap();

        match cursor.next(None::<&gio::Cancellable>) {
            Err(e) => {
                warn!("Encountered glib error: {}", e);
                Err(())
            }
            Ok(false) => {
                warn!("Resource {uri} was created but is not found in database");
                Err(())
            }
            Ok(true) => {
                let calendar_name = cursor.string(0).unwrap();
                let collection_uri = cursor.string(1).unwrap();
                let calendar = Calendar::new(&collection_uri, &calendar_name);

                info!("Calendar {calendar_name} created with uri {uri}");
                Ok(calendar)
            }
        }
    }
}
