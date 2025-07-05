use std::cell::OnceCell;

use adw::{prelude::*, subclass::prelude::*};
use ccm::{Calendar, Collection, Manager};
use gtk::{FlattenListModel, glib};

mod calendar_combo_row_header;
mod calendar_combo_row_item;

use crate::widgets::create_event_dialog::{
    calendar_combo_row_header::CalendarComboRowHeader,
    calendar_combo_row_item::CalendarComboRowItem,
};

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate, glib::Properties)]
    #[template(resource = "/io/gitlab/TitouanReal/CalendarManager/create_event_dialog.ui")]
    #[properties(wrapper_type = super::CreateEventDialog)]
    pub struct CreateEventDialog {
        #[property(get, set, construct_only)]
        manager: OnceCell<Manager>,
        flattened_collections_model: OnceCell<FlattenListModel>,
        #[template_child]
        pub calendar_choice: TemplateChild<adw::ComboRow>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for CreateEventDialog {
        const NAME: &'static str = "CreateEventDialog";
        type Type = super::CreateEventDialog;
        type ParentType = adw::Dialog;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_callbacks();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    #[glib::derived_properties]
    impl ObjectImpl for CreateEventDialog {
        fn constructed(&self) {
            self.parent_constructed();

            self.flattened_collections_model.get_or_init(|| {
                let model = self.manager().collections_model();
                FlattenListModel::new(Some(model))
            });

            self.calendar_choice
                .set_model(Some(self.flattened_collections_model()));
        }
    }
    impl WidgetImpl for CreateEventDialog {}
    impl AdwDialogImpl for CreateEventDialog {}

    #[gtk::template_callbacks]
    impl CreateEventDialog {
        fn manager(&self) -> &Manager {
            self.manager.get().expect("manager should be initialized")
        }

        fn flattened_collections_model(&self) -> &FlattenListModel {
            self.flattened_collections_model
                .get()
                .expect("flattened_collections_model should be initialized")
        }

        #[template_callback]
        fn calendar_item_setup(_factory: gtk::SignalListItemFactory, _item: gtk::ListItem) {}

        #[template_callback]
        fn calendar_item_bind(_factory: gtk::SignalListItemFactory, item: gtk::ListItem) {
            let calendar = item
                .item()
                .expect("item should be bound")
                .downcast::<Calendar>()
                .expect("item should be a Calendar");
            let calendar_combo_row_item = CalendarComboRowItem::new(&calendar);
            item.set_child(Some(&calendar_combo_row_item));
        }

        #[template_callback]
        fn calendar_item_unbind(_factory: gtk::SignalListItemFactory, _item: gtk::ListItem) {}

        #[template_callback]
        fn calendar_item_header_setup(
            _factory: gtk::SignalListItemFactory,
            _header: gtk::ListHeader,
        ) {
        }

        #[template_callback]
        fn calendar_item_header_bind(
            &self,
            header: gtk::ListHeader,
            _factory: gtk::SignalListItemFactory,
        ) {
            let start = header.start();
            let flatten_model = self.flattened_collections_model();
            let collection = flatten_model
                .model_for_item(start)
                .expect("item should exist at this position")
                .downcast::<Collection>()
                .expect("item should be a Collection");
            let calendar_combo_row_header = CalendarComboRowHeader::new(&collection);
            header.set_child(Some(&calendar_combo_row_header));
        }

        #[template_callback]
        fn calendar_item_header_unbind(
            _factory: gtk::SignalListItemFactory,
            _item: gtk::ListHeader,
        ) {
        }
    }
}

glib::wrapper! {
    pub struct CreateEventDialog(ObjectSubclass<imp::CreateEventDialog>)
        @extends gtk::Widget, adw::Dialog;
}

impl CreateEventDialog {
    pub fn new(manager: &Manager) -> Self {
        glib::Object::builder().property("manager", manager).build()
    }
}
