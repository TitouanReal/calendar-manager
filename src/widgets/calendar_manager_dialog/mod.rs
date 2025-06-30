use std::cell::OnceCell;

use adw::subclass::prelude::*;
use gtk::{glib, prelude::*};
use tracing::error;

mod calendar_details_page;
mod calendar_row;
mod collection_row;
mod collections_list;

use self::{calendar_details_page::CalendarDetailsPage, collections_list::CollectionsList};
use crate::core::{Manager, Resource};

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate, glib::Properties)]
    #[template(resource = "/io/gitlab/TitouanReal/CalendarManager/calendar_manager_dialog.ui")]
    #[properties(wrapper_type = super::CalendarManagerDialog)]
    pub struct CalendarManagerDialog {
        #[property(get, set, construct_only)]
        manager: OnceCell<Manager>,
        #[template_child]
        navigation_view: TemplateChild<adw::NavigationView>,
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

            klass.install_action(
                "calendar-manager.show-calendar-subpage",
                Some(&String::static_variant_type()),
                |obj, _, param| {
                    let resource = match param
                        .and_then(glib::Variant::get::<String>)
                        .and_then(|uri| obj.manager().find_resource(&uri))
                    {
                        Some(resource) => resource,
                        None => {
                            error!("Invalid resource URI");
                            return;
                        }
                    };

                    let Resource::Calendar(calendar) = resource else {
                        error!("Invalid resource type");
                        return;
                    };

                    obj.imp()
                        .navigation_view
                        .push(&CalendarDetailsPage::new(&calendar));
                },
            );
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
