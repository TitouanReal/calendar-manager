use std::cell::OnceCell;

use adw::{prelude::*, subclass::prelude::*};
use gtk::{gio, glib};

use crate::{core::Manager, widgets::CollectionsList};

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/io/gitlab/TitouanReal/CalendarManager/widgets/window.ui")]
    pub struct CalendarManagerWindow {
        manager: OnceCell<Manager>,
        #[template_child]
        collections_list: TemplateChild<CollectionsList>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for CalendarManagerWindow {
        const NAME: &'static str = "CalendarManagerWindow";
        type Type = super::CalendarManagerWindow;
        type ParentType = adw::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for CalendarManagerWindow {
        fn constructed(&self) {
            self.parent_constructed();

            self.manager.get_or_init(Manager::new);

            let manager = self.manager();
            self.collections_list.set_model(manager.collections());
        }
    }
    impl WidgetImpl for CalendarManagerWindow {}
    impl WindowImpl for CalendarManagerWindow {}
    impl ApplicationWindowImpl for CalendarManagerWindow {}
    impl AdwApplicationWindowImpl for CalendarManagerWindow {}

    impl CalendarManagerWindow {
        fn manager(&self) -> &Manager {
            self.manager.get().expect("manager should be initialized")
        }
    }
}

glib::wrapper! {
    pub struct CalendarManagerWindow(ObjectSubclass<imp::CalendarManagerWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, adw::ApplicationWindow,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl CalendarManagerWindow {
    pub fn new<P: IsA<gtk::Application>>(application: &P) -> Self {
        glib::Object::builder()
            .property("application", application)
            .build()
    }
}
