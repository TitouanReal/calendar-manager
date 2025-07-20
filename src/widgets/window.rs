use adw::{prelude::*, subclass::prelude::*};
use gtk::{gdk, gio, glib};
use tracing::info;

use crate::widgets::{
    CalendarManagerDialog, CreateEventDialog, SearchDialog, views::NarrowYearView,
};

pub(crate) mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/io/gitlab/TitouanReal/CalendarManager/window.ui")]
    pub struct CalendarManagerWindow {
        #[template_child]
        narrow_stack: TemplateChild<gtk::Stack>,
        #[template_child]
        narrow_year_view: TemplateChild<NarrowYearView>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for CalendarManagerWindow {
        const NAME: &'static str = "CalendarManagerWindow";
        type Type = super::CalendarManagerWindow;
        type ParentType = adw::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_callbacks();

            klass.install_action("win.search-events", None, |obj, _, _| {
                obj.imp().search_events();
            });

            klass.add_binding_action(
                gdk::Key::F,
                gdk::ModifierType::CONTROL_MASK,
                "win.search-events",
            );

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

    impl ObjectImpl for CalendarManagerWindow {}
    impl WidgetImpl for CalendarManagerWindow {}
    impl WindowImpl for CalendarManagerWindow {}
    impl ApplicationWindowImpl for CalendarManagerWindow {}
    impl AdwApplicationWindowImpl for CalendarManagerWindow {}

    #[gtk::template_callbacks]
    impl CalendarManagerWindow {
        #[template_callback]
        fn search_events(&self) {
            let dialog = SearchDialog::new();
            dialog.present(Some(&*self.obj()));
        }

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

        #[template_callback]
        fn get_year_label(&self, year: i32) -> String {
            year.to_string()
        }

        #[template_callback]
        fn open_narrow_month_view(&self, year: i32, month: i32) {
            info!(
                "Opening narrow month view for year {} and month {}",
                year, month
            );
            self.narrow_stack.set_visible_child_name("month");
        }

        #[template_callback]
        fn go_back_to_narrow_year_view(&self) {
            self.narrow_stack.set_visible_child_name("year");
        }

        #[template_callback]
        fn open_narrow_days_view(&self) {
            self.narrow_stack.set_visible_child_name("days");
        }

        #[template_callback]
        fn go_back_to_narrow_month_view(&self) {
            self.narrow_stack.set_visible_child_name("month");
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
