use std::{cell::Cell, sync::LazyLock};

use adw::{prelude::*, subclass::prelude::*};
use gtk::glib::{self, closure_local, subclass::Signal};

use super::NarrowYearViewMonthCell;

pub(crate) mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate, glib::Properties)]
    #[template(resource = "/io/gitlab/TitouanReal/CalendarManager/narrow_year_view_year_row.ui")]
    #[properties(wrapper_type = super::NarrowYearViewYearRow)]
    pub struct NarrowYearViewYearRow {
        #[property(get, set)]
        year: Cell<i32>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for NarrowYearViewYearRow {
        const NAME: &'static str = "NarrowYearViewYearRow";
        type Type = super::NarrowYearViewYearRow;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_callbacks();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    #[glib::derived_properties]
    impl ObjectImpl for NarrowYearViewYearRow {
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

    impl WidgetImpl for NarrowYearViewYearRow {}
    impl BoxImpl for NarrowYearViewYearRow {}

    #[gtk::template_callbacks]
    impl NarrowYearViewYearRow {
        #[template_callback]
        fn get_year_label_narrow(&self) -> String {
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
    pub struct NarrowYearViewYearRow(ObjectSubclass<imp::NarrowYearViewYearRow>)
        @extends gtk::Widget, gtk::Box,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl NarrowYearViewYearRow {
    pub fn new(year: i32) -> Self {
        glib::Object::builder().property("year", year).build()
    }

    pub fn connect_month_clicked<F: Fn(&Self, i32, i32) + 'static>(
        &self,
        f: F,
    ) -> glib::SignalHandlerId {
        self.connect_closure(
            "month-clicked",
            true,
            closure_local!(move |obj: Self, year: i32, month: i32| {
                f(&obj, year, month);
            }),
        )
    }
}
