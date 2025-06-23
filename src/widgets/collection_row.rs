use std::cell::OnceCell;

use adw::prelude::*;
use gtk::{glib, subclass::prelude::*, Label, ListBoxRow};

use crate::core::{Calendar, Collection};

mod imp {
    use super::*;

    #[derive(Default, gtk::CompositeTemplate, glib::Properties)]
    #[template(resource = "/io/gitlab/TitouanReal/CalendarManager/widgets/collection_row.ui")]
    #[properties(wrapper_type = super::CollectionRow)]
    pub struct CollectionRow {
        #[property(get, set, construct_only)]
        pub collection: OnceCell<Collection>,
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
        let collection = self.collection();

        collection
            .bind_property("name", &*imp.name_label, "label")
            .sync_create()
            .build();

        imp.calendars_list
            .bind_model(Some(&collection.calendars()), |calendar| {
                let calendar = calendar.downcast_ref::<Calendar>().unwrap();
                let calendar_row = ListBoxRow::new();
                let label = Label::new(Some(&calendar.name()));
                calendar_row.set_child(Some(&label));
                calendar_row.upcast()
            });
    }
}
