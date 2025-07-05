use std::cell::OnceCell;

use adw::{prelude::*, subclass::prelude::*};
use ccm::{Collection, Manager};
use gtk::glib;

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate, glib::Properties)]
    #[template(resource = "/io/gitlab/TitouanReal/CalendarManager/create_event_dialog.ui")]
    #[properties(wrapper_type = super::CreateEventDialog)]
    pub struct CreateEventDialog {
        #[property(get, set, construct_only)]
        manager: OnceCell<Manager>,
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

            // let imp = self.imp();

            let model = self.manager().collections();
            self.calendar_choice.set_model(Some(&model));
        }
    }
    impl WidgetImpl for CreateEventDialog {}
    impl AdwDialogImpl for CreateEventDialog {}

    #[gtk::template_callbacks]
    impl CreateEventDialog {
        fn manager(&self) -> &Manager {
            self.manager.get().expect("manager should be initialized")
        }

        #[template_callback]
        fn calendar_item_setup(_factory: gtk::SignalListItemFactory, _item: gtk::ListItem) {}

        #[template_callback]
        fn calendar_item_bind(_factory: gtk::SignalListItemFactory, item: gtk::ListItem) {
            let collection = item.item().unwrap().downcast::<Collection>().unwrap();
            item.set_child(Some(&gtk::Label::new(Some(&collection.name()))));
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
            _factory: gtk::SignalListItemFactory,
            _header: gtk::ListHeader,
        ) {
            // let collection = header.item().unwrap().downcast::<Collection>().unwrap();
            // let start = header.start();
            // let end = header.end();
            // let child = gtk::Label::new(Some(&format!(
            //     "{} - {} - {}",
            //     collection.name(),
            //     start,
            //     end
            // )));
            // child.set_visible(start == 0);
            // header.set_child(Some(&child));
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
