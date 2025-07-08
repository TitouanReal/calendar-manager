use adw::{prelude::*, subclass::prelude::*};
use gtk::{gdk, gio, glib};

use crate::widgets::{CalendarManagerDialog, CreateEventDialog};

pub(crate) mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/io/gitlab/TitouanReal/CalendarManager/window.ui")]
    pub struct CalendarManagerWindow {}

    #[glib::object_subclass]
    impl ObjectSubclass for CalendarManagerWindow {
        const NAME: &'static str = "CalendarManagerWindow";
        type Type = super::CalendarManagerWindow;
        type ParentType = adw::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_callbacks();

            klass.install_action("win.manage-calendars", None, |obj, _, _| {
                obj.imp().manage_calendars();
            });

            klass.add_binding_action(
                gdk::Key::M,
                gdk::ModifierType::CONTROL_MASK | gdk::ModifierType::ALT_MASK,
                "win.manage-calendars",
            );

            klass.install_action("win.create-event", None, |obj, _, _| {
                obj.imp().create_event();
            });

            klass.add_binding_action(
                gdk::Key::N,
                gdk::ModifierType::CONTROL_MASK,
                "win.create-event",
            );
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for CalendarManagerWindow {
        fn constructed(&self) {
            self.parent_constructed();

            // self.manager.get_or_init(Manager::new);
        }
    }

    impl WidgetImpl for CalendarManagerWindow {}
    impl WindowImpl for CalendarManagerWindow {}
    impl ApplicationWindowImpl for CalendarManagerWindow {}
    impl AdwApplicationWindowImpl for CalendarManagerWindow {}

    #[gtk::template_callbacks]
    impl CalendarManagerWindow {
        // fn manager(&self) -> &Manager {
        //     self.manager.get().expect("manager should be initialized")
        // }

        #[template_callback]
        fn manage_calendars(&self) {
            let dialog = CalendarManagerDialog::new();
            dialog.present(Some(&*self.obj()));
        }

        #[template_callback]
        fn create_event(&self) {
            let dialog = CreateEventDialog::new();
            dialog.present(Some(&*self.obj()));
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
