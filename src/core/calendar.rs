use std::cell::RefCell;

use adw::{prelude::*, subclass::prelude::*};
use gtk::glib::{self, Object};

mod imp {
    use super::*;

    #[derive(Debug, Default, glib::Properties)]
    #[properties(wrapper_type = super::Calendar)]
    pub struct Calendar {
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
    pub fn new(name: &str) -> Self {
        glib::Object::builder().property("name", name).build()
    }
}
