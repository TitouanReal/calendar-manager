use std::cell::{OnceCell, RefCell};

use adw::{prelude::*, subclass::prelude::*};
use gtk::{
    gio::ListStore,
    glib::{self, Object},
};
use tsparql::SparqlConnection;

use crate::core::Calendar;

mod imp {
    use super::*;

    #[derive(Debug, Default, glib::Properties)]
    #[properties(wrapper_type = super::Collection)]
    pub struct Collection {
        #[property(get, set)]
        name: RefCell<String>,
        #[property(get)]
        calendars: OnceCell<ListStore>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Collection {
        const NAME: &'static str = "Collection";
        type Type = super::Collection;
        type ParentType = Object;
    }

    #[glib::derived_properties]
    impl ObjectImpl for Collection {
        fn constructed(&self) {
            self.parent_constructed();

            self.calendars.get_or_init(ListStore::new::<Calendar>);
        }
    }

    impl Collection {
        pub fn calendars(&self) -> &ListStore {
            self.calendars
                .get()
                .expect("providers should be initialized")
        }
    }
}

glib::wrapper! {
    pub struct Collection(ObjectSubclass<imp::Collection>);
}

impl Collection {
    pub fn new(name: &str) -> Self {
        glib::Object::builder().property("name", name).build()
    }

    /// Retrieves a collection resource from a URI.
    ///
    /// # Panics
    ///
    /// This function may panic if the given URI is invalid or does not point to a collection resource.
    pub fn from_uri(_read_connection: &SparqlConnection, _uri: &str) -> Result<Self, ()> {
        todo!()
    }

    pub fn add_calendar(&self, calendar: &Calendar) {
        self.imp().calendars().append(calendar);
    }
}
