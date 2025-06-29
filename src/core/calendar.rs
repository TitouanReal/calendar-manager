use std::cell::{OnceCell, RefCell};

use adw::{prelude::*, subclass::prelude::*};
use gtk::{
    gdk::{self, RGBA},
    gio::ListStore,
    glib::{self, Object},
};

use crate::core::Event;

mod imp {
    use super::*;

    #[derive(Debug, Default, glib::Properties)]
    #[properties(wrapper_type = super::Calendar)]
    pub struct Calendar {
        #[property(get, set)]
        name: RefCell<String>,
        // TODO: Remove the Option
        #[property(get, set)]
        color: RefCell<Option<RGBA>>,
        #[property(get)]
        events: OnceCell<ListStore>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Calendar {
        const NAME: &'static str = "Calendar";
        type Type = super::Calendar;
        type ParentType = Object;
    }

    #[glib::derived_properties]
    impl ObjectImpl for Calendar {
        fn constructed(&self) {
            self.parent_constructed();

            self.events.get_or_init(ListStore::new::<Event>);
        }
    }

    impl Calendar {
        pub fn events(&self) -> &ListStore {
            self.events.get().expect("events should be initialized")
        }
    }
}

glib::wrapper! {
    pub struct Calendar(ObjectSubclass<imp::Calendar>);
}

impl Calendar {
    pub fn new(name: &str, color: gdk::RGBA) -> Self {
        glib::Object::builder()
            .property("name", name)
            .property("color", Some(color))
            .build()
    }

    pub fn add_event(&self, event: &Event) {
        self.imp().events().append(event);
    }
}
