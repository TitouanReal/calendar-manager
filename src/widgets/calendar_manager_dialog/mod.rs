use std::cell::OnceCell;

use adw::subclass::prelude::*;
use gtk::{glib, prelude::*};

mod calendar_row;
mod collection_row;
mod collections_list;

use self::collections_list::CollectionsList;
use crate::core::Manager;

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate, glib::Properties)]
    #[template(
        resource = "/io/gitlab/TitouanReal/CalendarManager/widgets/calendar_manager_dialog/mod.ui"
    )]
    #[properties(wrapper_type = super::CalendarManagerDialog)]
    pub struct CalendarManagerDialog {
        #[property(get, set, construct_only)]
        manager: OnceCell<Manager>,
        #[template_child]
        collections_list: TemplateChild<CollectionsList>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for CalendarManagerDialog {
        const NAME: &'static str = "CalendarManagerDialog";
        type Type = super::CalendarManagerDialog;
        type ParentType = adw::Dialog;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    #[glib::derived_properties]
    impl ObjectImpl for CalendarManagerDialog {
        fn constructed(&self) {
            self.parent_constructed();

            let manager = self.manager();
            self.collections_list.set_model(manager.collections());
        }
    }
    impl WidgetImpl for CalendarManagerDialog {}
    impl AdwDialogImpl for CalendarManagerDialog {}

    impl CalendarManagerDialog {
        fn manager(&self) -> &Manager {
            self.manager.get().expect("manager should be initialized")
        }
    }
}

glib::wrapper! {
    pub struct CalendarManagerDialog(ObjectSubclass<imp::CalendarManagerDialog>)
        @extends gtk::Widget, adw::Dialog;
}

impl CalendarManagerDialog {
    pub fn new(manager: &Manager) -> Self {
        glib::Object::builder().property("manager", manager).build()
    }
}
