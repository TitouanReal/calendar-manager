use std::cell::OnceCell;

use adw::{prelude::*, subclass::prelude::*};
use ccm::{Calendar, Collection};
use gtk::{FlattenListModel, glib};

mod calendar_combo_row_header;
mod calendar_combo_row_item;

use crate::application::CalendarManagerApplication;

use self::{
    calendar_combo_row_header::CalendarComboRowHeader,
    calendar_combo_row_item::CalendarComboRowItem,
};

mod imp {
    use super::*;

    #[derive(Default, gtk::CompositeTemplate)]
    #[template(resource = "/io/gitlab/TitouanReal/CalendarManager/calendar_combo_row.ui")]
    pub struct CalendarComboRow {
        flattened_collections_model: OnceCell<FlattenListModel>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for CalendarComboRow {
        const NAME: &'static str = "CalendarComboRow";
        type Type = super::CalendarComboRow;
        type ParentType = adw::ComboRow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_callbacks();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for CalendarComboRow {
        fn constructed(&self) {
            self.parent_constructed();

            let manager = CalendarManagerApplication::default().manager();

            self.flattened_collections_model.get_or_init(|| {
                let model = manager.collections_model();
                FlattenListModel::new(Some(model))
            });

            self.obj()
                .set_model(Some(self.flattened_collections_model()));
        }
    }

    impl WidgetImpl for CalendarComboRow {}
    impl ListBoxRowImpl for CalendarComboRow {}
    impl PreferencesRowImpl for CalendarComboRow {}
    impl ActionRowImpl for CalendarComboRow {}
    impl ComboRowImpl for CalendarComboRow {}

    #[gtk::template_callbacks]
    impl CalendarComboRow {
        // fn manager(&self) -> &Manager {
        //     self.manager.get().expect("manager should be initialized")
        // }

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
    pub struct CalendarComboRow(ObjectSubclass<imp::CalendarComboRow>)
    @extends gtk::Widget, gtk::ListBoxRow, adw::PreferencesRow, adw::ActionRow, adw::ComboRow;
}

impl CalendarComboRow {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }
}

impl Default for CalendarComboRow {
    fn default() -> Self {
        Self::new()
    }
}
