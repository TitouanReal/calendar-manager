use std::cell::OnceCell;

use adw::{prelude::*, subclass::prelude::*};
use gtk::glib;

use crate::core::Calendar;

mod imp {
    use super::*;

    #[derive(Default, gtk::CompositeTemplate, glib::Properties)]
    #[template(resource = "/io/gitlab/TitouanReal/CalendarManager/widgets/calendar_row.ui")]
    #[properties(wrapper_type = super::CalendarRow)]
    pub struct CalendarRow {
        #[property(get, set, construct_only)]
        pub calendar: OnceCell<Calendar>,
        #[template_child]
        pub name_label: TemplateChild<gtk::Label>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for CalendarRow {
        const NAME: &'static str = "CalendarRow";
        type Type = super::CalendarRow;
        type ParentType = adw::ActionRow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    #[glib::derived_properties]
    impl ObjectImpl for CalendarRow {
        fn constructed(&self) {
            self.parent_constructed();

            self.obj().setup_widget();
        }
    }
    impl WidgetImpl for CalendarRow {}
    impl ListBoxRowImpl for CalendarRow {}
    impl PreferencesRowImpl for CalendarRow {}
    impl ActionRowImpl for CalendarRow {}
}

glib::wrapper! {
    pub struct CalendarRow(ObjectSubclass<imp::CalendarRow>)
    @extends gtk::Widget, gtk::ListBoxRow, adw::PreferencesRow, adw::ActionRow;
}

impl CalendarRow {
    pub fn new(calendar: &Calendar) -> Self {
        glib::Object::builder()
            .property("calendar", calendar)
            .build()
    }

    fn setup_widget(&self) {
        let imp = self.imp();
        let calendar = self.calendar();

        calendar
            .bind_property("name", &*imp.name_label, "label")
            .sync_create()
            .build();
    }
}
