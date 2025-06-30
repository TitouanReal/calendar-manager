use std::cell::RefCell;

use adw::{prelude::*, subclass::prelude::*};
use gtk::glib::{self};

use crate::core::Calendar;

mod imp {
    use super::*;

    #[derive(Default, gtk::CompositeTemplate, glib::Properties)]
    #[template(resource = "/io/gitlab/TitouanReal/CalendarManager/calendar_details_page.ui")]
    #[properties(wrapper_type = super::CalendarDetailsPage)]
    pub struct CalendarDetailsPage {
        #[property(get, set, construct_only)]
        pub calendar: RefCell<Option<Calendar>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for CalendarDetailsPage {
        const NAME: &'static str = "CalendarDetailsPage";
        type Type = super::CalendarDetailsPage;
        type ParentType = adw::NavigationPage;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_callbacks();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    #[glib::derived_properties]
    impl ObjectImpl for CalendarDetailsPage {
        fn constructed(&self) {
            self.parent_constructed();
        }
    }
    impl WidgetImpl for CalendarDetailsPage {}
    impl NavigationPageImpl for CalendarDetailsPage {}

    #[gtk::template_callbacks]
    impl CalendarDetailsPage {}
}

glib::wrapper! {
    pub struct CalendarDetailsPage(ObjectSubclass<imp::CalendarDetailsPage>)
    @extends gtk::Widget, adw::NavigationPage;
}

impl CalendarDetailsPage {
    pub fn new(calendar: &Calendar) -> Self {
        glib::Object::builder()
            .property("calendar", calendar)
            .build()
    }
}
