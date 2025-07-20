use std::{cell::Cell, sync::LazyLock};

use adw::{prelude::*, subclass::prelude::*};
use gtk::glib::{self, subclass::Signal};

mod narrow_year_view_month_cell;

use self::narrow_year_view_month_cell::NarrowYearViewMonthCell;

pub(crate) mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate, glib::Properties)]
    #[template(resource = "/io/gitlab/TitouanReal/CalendarManager/narrow_year_view.ui")]
    #[properties(wrapper_type = super::NarrowYearView)]
    pub struct NarrowYearView {
        #[property(get, set)]
        year: Cell<i32>,
        #[template_child]
        scrolled_window: TemplateChild<gtk::ScrolledWindow>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for NarrowYearView {
        const NAME: &'static str = "NarrowYearView";
        type Type = super::NarrowYearView;
        type ParentType = gtk::Widget;

        fn class_init(klass: &mut Self::Class) {
            NarrowYearViewMonthCell::ensure_type();

            klass.bind_template();
            klass.bind_template_callbacks();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    #[glib::derived_properties]
    impl ObjectImpl for NarrowYearView {
        fn constructed(&self) {
            self.parent_constructed();
            self.obj().set_year(2025);
        }

        fn signals() -> &'static [Signal] {
            static SIGNALS: LazyLock<Vec<Signal>> = LazyLock::new(|| {
                vec![
                    Signal::builder("month-clicked")
                        // Year, Month
                        // TODO: Should these be something else than i32?
                        .param_types([i32::static_type(), i32::static_type()])
                        .build(),
                ]
            });
            SIGNALS.as_ref()
        }
    }

    impl WidgetImpl for NarrowYearView {
        fn size_allocate(&self, width: i32, height: i32, baseline: i32) {
            let (_header_height, ..) = self
                .scrolled_window
                .measure(gtk::Orientation::Vertical, width);
            self.scrolled_window.allocate(width, height, baseline, None);
        }
    }

    #[gtk::template_callbacks]
    impl NarrowYearView {
        #[template_callback]
        fn go_to_previous_year(&self) {
            let year = self.obj().year();
            self.obj().set_year(year - 1);
        }

        #[template_callback]
        fn go_to_next_year(&self) {
            let year = self.obj().year();
            self.obj().set_year(year + 1);
        }

        #[template_callback]
        fn get_year_label(&self) -> String {
            self.obj().year().to_string()
        }

        #[template_callback]
        fn month_cell_clicked(&self, cell: NarrowYearViewMonthCell) {
            self.obj()
                .emit_by_name::<()>("month-clicked", &[&cell.year(), &cell.month()]);
        }
    }
}

glib::wrapper! {
    pub struct NarrowYearView(ObjectSubclass<imp::NarrowYearView>)
        @extends gtk::Widget;
}
