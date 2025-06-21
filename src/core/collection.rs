use std::{cell::RefCell, fmt};

use adw::{prelude::*, subclass::prelude::*};
use gtk::glib::{self, Object};

mod imp {
    use super::*;

    #[derive(Debug, Default, glib::Properties)]
    #[properties(wrapper_type = super::Collection)]
    pub struct Collection {
        #[property(get, set)]
        name: RefCell<String>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Collection {
        const NAME: &'static str = "Collection";
        type Type = super::Collection;
        type ParentType = Object;
    }

    #[glib::derived_properties]
    impl ObjectImpl for Collection {}
}

glib::wrapper! {
    pub struct Collection(ObjectSubclass<imp::Collection>);
}

impl Collection {
    pub fn new(name: &str) -> Self {
        glib::Object::builder().property("name", name).build()
    }
}

impl fmt::Display for Collection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}
