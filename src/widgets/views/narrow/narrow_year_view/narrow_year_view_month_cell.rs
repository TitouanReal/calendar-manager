use std::cell::Cell;

use adw::{prelude::*, subclass::prelude::*};
use ccm::jiff;
use gtk::glib;

pub(crate) mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate, glib::Properties)]
    #[template(resource = "/io/gitlab/TitouanReal/CalendarManager/narrow_year_view_month_cell.ui")]
    #[properties(wrapper_type = super::NarrowYearViewMonthCell)]
    pub struct NarrowYearViewMonthCell {
        #[property(get, set = Self::set_year)]
        year: Cell<i32>,
        #[property(get, set = Self::set_month)]
        month: Cell<i32>,
        #[template_child]
        days_grid: TemplateChild<gtk::Grid>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for NarrowYearViewMonthCell {
        const NAME: &'static str = "NarrowYearViewMonthCell";
        type Type = super::NarrowYearViewMonthCell;
        type ParentType = gtk::Button;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_callbacks();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    #[glib::derived_properties]
    impl ObjectImpl for NarrowYearViewMonthCell {}
    impl WidgetImpl for NarrowYearViewMonthCell {}
    impl ButtonImpl for NarrowYearViewMonthCell {}

    #[gtk::template_callbacks]
    impl NarrowYearViewMonthCell {
        fn set_year(&self, year: i32) {
            self.year.set(year);
            self.set_days_grid();
        }

        fn set_month(&self, month: i32) {
            self.month.set(month);
            self.set_days_grid();
        }

        fn set_days_grid(&self) {
            let year = self.year.get();
            let month = self.month.get();
            let first_day = jiff::civil::date(year as i16, month as i8, 1);
            let days_in_month = first_day.days_in_month() as usize;
            let weekday_of_first_day = first_day.weekday() as usize - 1;

            for cell in 0..42 {
                if let Some(label) = self.days_grid.child_at(cell % 7, cell / 7) {
                    self.days_grid.remove(&label);
                }
            }

            let mut cells = 0..42;

            for cell in cells.by_ref().take(weekday_of_first_day) {
                let label = gtk::Label::new(None);
                label.add_css_class("narrow_year_view_month_cell_month_label");
                self.days_grid.attach(&label, cell % 7, cell / 7, 1, 1);
            }

            for (day_number, cell) in cells.by_ref().take(days_in_month).enumerate() {
                let label = gtk::Label::new(Some(&format!("{}", day_number + 1)));
                label.add_css_class("narrow_year_view_month_cell_day_label");
                self.days_grid.attach(&label, cell % 7, cell / 7, 1, 1);
            }

            for cell in cells {
                let label = gtk::Label::new(None);
                label.add_css_class("narrow_year_view_month_cell_day_label");
                self.days_grid.attach(&label, cell % 7, cell / 7, 1, 1);
            }
        }

        #[template_callback]
        fn get_month_name(&self) -> &str {
            let month = self.obj().month();
            match month {
                1 => "January",
                2 => "February",
                3 => "March",
                4 => "April",
                5 => "May",
                6 => "June",
                7 => "July",
                8 => "August",
                9 => "September",
                10 => "October",
                11 => "November",
                12 => "December",
                _ => "",
            }
        }
    }
}

glib::wrapper! {
    pub struct NarrowYearViewMonthCell(ObjectSubclass<imp::NarrowYearViewMonthCell>)
        @extends gtk::Widget, gtk::Button,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}
