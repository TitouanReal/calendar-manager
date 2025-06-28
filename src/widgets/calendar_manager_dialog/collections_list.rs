use adw::prelude::*;
use gtk::{gio::ListStore, glib, subclass::prelude::*};

use super::collection_row::CollectionRow;
use crate::core::Collection;

mod imp {
    use super::*;

    #[derive(Default, gtk::CompositeTemplate)]
    #[template(resource = "/io/gitlab/TitouanReal/CalendarManager/collections_list.ui")]
    pub struct CollectionsList {
        #[template_child]
        pub providers_list: TemplateChild<gtk::ListBox>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for CollectionsList {
        const NAME: &'static str = "CollectionsList";
        type Type = super::CollectionsList;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for CollectionsList {
        fn constructed(&self) {
            self.parent_constructed();
        }
    }
    impl WidgetImpl for CollectionsList {}
    impl BoxImpl for CollectionsList {}
}

glib::wrapper! {
    pub struct CollectionsList(ObjectSubclass<imp::CollectionsList>)
        @extends gtk::Widget, gtk::Box;
}

impl CollectionsList {
    pub fn set_model(&self, model: ListStore) {
        let imp = self.imp();

        imp.providers_list.bind_model(Some(&model), move |obj| {
            let collection = obj.downcast_ref::<Collection>().unwrap();

            let row = CollectionRow::new(collection);
            // let row = ListBoxRow::new();
            // let child = gtk::Label::new(Some(&provider.name()));
            // row.set_child(Some(&child));
            // row.set_title(&provider.name());
            // let mut subtitle = String::new();
            // row.set_subtitle(&subtitle);

            row.upcast::<gtk::Widget>()
        });
    }
}
