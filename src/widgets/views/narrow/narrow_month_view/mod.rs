use std::{cell::Cell, sync::LazyLock};

use adw::{prelude::*, subclass::prelude::*};
use gtk::glib::{self, subclass::Signal};

pub(crate) mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate, glib::Properties)]
    #[template(resource = "/io/gitlab/TitouanReal/CalendarManager/narrow_month_view.ui")]
    #[properties(wrapper_type = super::NarrowMonthView)]
    pub struct NarrowMonthView {
        #[property(get, set)]
        year: Cell<i32>,
        #[property(get, set)]
        month: Cell<i32>,
        #[template_child]
        button: TemplateChild<gtk::Button>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for NarrowMonthView {
        const NAME: &'static str = "NarrowMonthView";
        type Type = super::NarrowMonthView;
        type ParentType = gtk::Widget;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_callbacks();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    #[glib::derived_properties]
    impl ObjectImpl for NarrowMonthView {
        // fn constructed(&self) {
        //     self.parent_constructed();
        //     self.obj().set_year(2025);
        // }

        fn signals() -> &'static [Signal] {
            static SIGNALS: LazyLock<Vec<Signal>> =
                LazyLock::new(|| vec![Signal::builder("day-clicked").build()]);
            SIGNALS.as_ref()
        }
    }

    impl WidgetImpl for NarrowMonthView {
        fn size_allocate(&self, width: i32, height: i32, baseline: i32) {
            let (_header_height, ..) = self.button.measure(gtk::Orientation::Vertical, width);
            self.button.allocate(width, height, baseline, None);
        }
    }

    #[gtk::template_callbacks]
    impl NarrowMonthView {
        #[template_callback]
        fn get_month_label(&self) -> String {
            let year = self.obj().year();
            let month = self.obj().month();
            format!("{year} {month}")
        }

        #[template_callback]
        fn day_cell_clicked(&self) {
            self.obj().emit_by_name::<()>("day-clicked", &[]);
        }
    }
}

glib::wrapper! {
    pub struct NarrowMonthView(ObjectSubclass<imp::NarrowMonthView>)
        @extends gtk::Widget;
}
