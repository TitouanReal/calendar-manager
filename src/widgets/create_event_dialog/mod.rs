use adw::subclass::prelude::*;
use gtk::glib;

mod calendar_combo_row;

use calendar_combo_row::CalendarComboRow;

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/io/gitlab/TitouanReal/CalendarManager/create_event_dialog.ui")]
    pub struct CreateEventDialog {
        #[template_child]
        calendar_choice: TemplateChild<CalendarComboRow>,
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

    impl ObjectImpl for CreateEventDialog {}

    impl WidgetImpl for CreateEventDialog {}
    impl AdwDialogImpl for CreateEventDialog {}

    #[gtk::template_callbacks]
    impl CreateEventDialog {}
}

glib::wrapper! {
    pub struct CreateEventDialog(ObjectSubclass<imp::CreateEventDialog>)
        @extends gtk::Widget, adw::Dialog;
}

impl CreateEventDialog {
    pub fn new() -> Self {
        glib::Object::new()
    }
}

impl Default for CreateEventDialog {
    fn default() -> Self {
        Self::new()
    }
}
