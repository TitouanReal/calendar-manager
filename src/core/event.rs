use std::cell::RefCell;

use adw::{prelude::*, subclass::prelude::*};
use gtk::glib::{self, Object};
use tsparql::SparqlConnection;

mod imp {
    use super::*;

    #[derive(Debug, Default, glib::Properties)]
    #[properties(wrapper_type = super::Event)]
    pub struct Event {
        #[property(get, set)]
        name: RefCell<String>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Event {
        const NAME: &'static str = "Event";
        type Type = super::Event;
        type ParentType = Object;
    }

    #[glib::derived_properties]
    impl ObjectImpl for Event {}
}

glib::wrapper! {
    pub struct Event(ObjectSubclass<imp::Event>);
}

impl Event {
    pub fn new(name: &str) -> Self {
        glib::Object::builder().property("name", name).build()
    }

    /// Retrieves an event resource from a URI.
    ///
    /// # Panics
    ///
    /// This function may panic if the given URI is invalid or does not point to an event resource.
    pub fn from_uri(_read_connection: &SparqlConnection, _uri: &str) -> Result<Self, ()> {
        todo!()
    }
}
