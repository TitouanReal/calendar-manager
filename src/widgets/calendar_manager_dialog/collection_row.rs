use std::cell::RefCell;

use adw::prelude::*;
use gtk::{glib, subclass::prelude::*};

use super::calendar_row::CalendarRow;
use crate::core::Collection;

mod imp {
    use super::*;

    #[derive(Default, gtk::CompositeTemplate, glib::Properties)]
    #[template(resource = "/io/gitlab/TitouanReal/CalendarManager/collection_row.ui")]
    #[properties(wrapper_type = super::CollectionRow)]
    pub struct CollectionRow {
        #[property(get, set, construct_only)]
        pub collection: RefCell<Option<Collection>>,
        // #[template_child]
        // pub preferences_group: TemplateChild<adw::PreferencesGroup>,
        #[template_child]
        pub name_label: TemplateChild<gtk::Label>,
        #[template_child]
        pub calendars_list: TemplateChild<gtk::ListBox>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for CollectionRow {
        const NAME: &'static str = "CollectionRow";
        type Type = super::CollectionRow;
        type ParentType = gtk::ListBoxRow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_callbacks();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    #[glib::derived_properties]
    impl ObjectImpl for CollectionRow {
        fn constructed(&self) {
            self.parent_constructed();
            self.obj().setup_widget();
        }
    }
    impl WidgetImpl for CollectionRow {}
    impl ListBoxRowImpl for CollectionRow {}

    #[gtk::template_callbacks]
    impl CollectionRow {
        #[template_callback]
        fn create_calendar(&self) {
            dbg!("todo");
        }
    }
}

glib::wrapper! {
    pub struct CollectionRow(ObjectSubclass<imp::CollectionRow>)
        @extends gtk::Widget, gtk::ListBoxRow;
}

impl CollectionRow {
    pub fn new(collection: &Collection) -> Self {
        glib::Object::builder()
            .property("collection", collection)
            .build()
    }

    fn setup_widget(&self) {
        let imp = self.imp();
        let collection = self.collection().expect("collection should be initialized");

        imp.calendars_list
            .bind_model(Some(&collection.calendars()), |calendar| {
                CalendarRow::new(calendar.downcast_ref().unwrap()).upcast()
            });
    }
}
