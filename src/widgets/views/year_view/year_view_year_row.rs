use std::{
    cell::{Cell, OnceCell},
    sync::LazyLock,
};

use adw::{prelude::*, subclass::prelude::*};
use gtk::glib::{self, clone, closure_local, subclass::Signal};

use super::YearViewMonthCell;

#[derive(Debug, Default, Hash, Eq, PartialEq, Clone, Copy, glib::Enum)]
#[enum_type(name = "GridLayout")]
pub enum GridLayout {
    #[enum_value(name = "Rows 6 Columns 2", nick = "rows6columns2")]
    Rows6Columns2,
    #[default]
    #[enum_value(name = "Rows 4 Columns 3", nick = "rows4columns3")]
    Rows4Columns3,
    #[enum_value(name = "Rows 3 Columns 4", nick = "rows3columns4")]
    Rows3Columns4,
    #[enum_value(name = "Rows 2 Columns 6", nick = "rows2columns6")]
    Rows2Columns6,
}

pub(crate) mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate, glib::Properties)]
    #[template(resource = "/io/gitlab/TitouanReal/CalendarManager/year_view_year_row.ui")]
    #[properties(wrapper_type = super::YearViewYearRow)]
    pub struct YearViewYearRow {
        #[property(get, set)]
        year: Cell<i32>,
        #[property(get, set, builder(GridLayout::default()))]
        grid_layout: Cell<GridLayout>,
        #[template_child]
        month_grid: TemplateChild<gtk::Grid>,
        month_cells: OnceCell<[YearViewMonthCell; 12]>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for YearViewYearRow {
        const NAME: &'static str = "YearViewYearRow";
        type Type = super::YearViewYearRow;
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
    impl ObjectImpl for YearViewYearRow {
        fn constructed(&self) {
            self.parent_constructed();

            let obj = self.obj();

            let year = self.year.get();

            let mut cells = Vec::new();
            for month in 1..=12 {
                let cell = YearViewMonthCell::new(year, month);
                obj.bind_property("year", &cell, "year").build();
                cell.connect_clicked(clone!(
                    #[weak(rename_to = imp)]
                    self,
                    move |cell| {
                        imp.obj()
                            .emit_by_name::<()>("month-clicked", &[&cell.year(), &cell.month()]);
                    }
                ));
                cells.push(cell);
            }

            self.month_cells
                .set(cells.try_into().expect("There should be 12 month cells"))
                .expect("Month cells should not already be initialized");

            obj.connect_grid_layout_notify(clone!(
                #[weak(rename_to = imp)]
                self,
                move |_| {
                    imp.update_grid();
                }
            ));
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

    impl WidgetImpl for YearViewYearRow {}
    impl BoxImpl for YearViewYearRow {}

    #[gtk::template_callbacks]
    impl YearViewYearRow {
        fn update_grid(&self) {
            for i in 0..6 {
                for j in 0..6 {
                    if let Some(widget) = self.month_grid.child_at(i, j) {
                        self.month_grid.remove(&widget);
                    }
                }
            }

            match self.grid_layout.get() {
                GridLayout::Rows2Columns6 => {
                    for i in 0..12 {
                        let month_cell = self
                            .month_cells
                            .get()
                            .expect("Month cells should be initialized")
                            .get(i)
                            .expect("There should be 12 month cells");
                        let row = i / 6;
                        let column = i % 6;
                        self.month_grid
                            .attach(month_cell, column as i32, row as i32, 1, 1);
                    }
                }
                GridLayout::Rows3Columns4 => {
                    for i in 0..12 {
                        let month_cell = self
                            .month_cells
                            .get()
                            .expect("Month cells should be initialized")
                            .get(i)
                            .expect("There should be 12 month cells");
                        let row = i / 4;
                        let column = i % 4;
                        self.month_grid
                            .attach(month_cell, column as i32, row as i32, 1, 1);
                    }
                }
                GridLayout::Rows4Columns3 => {
                    for i in 0..12 {
                        let month_cell = self
                            .month_cells
                            .get()
                            .expect("Month cells should be initialized")
                            .get(i)
                            .expect("There should be 12 month cells");
                        let row = i / 3;
                        let column = i % 3;
                        self.month_grid
                            .attach(month_cell, column as i32, row as i32, 1, 1);
                    }
                }
                GridLayout::Rows6Columns2 => {
                    for i in 0..12 {
                        let month_cell = self
                            .month_cells
                            .get()
                            .expect("Month cells should be initialized")
                            .get(i)
                            .expect("There should be 12 month cells");
                        let row = i / 2;
                        let column = i % 2;
                        self.month_grid
                            .attach(month_cell, column as i32, row as i32, 1, 1);
                    }
                }
            }
        }

        #[template_callback]
        fn get_year_label_narrow(&self) -> String {
            self.obj().year().to_string()
        }

        #[template_callback]
        fn month_cell_clicked(&self, cell: YearViewMonthCell) {
            self.obj()
                .emit_by_name::<()>("month-clicked", &[&cell.year(), &cell.month()]);
        }
    }
}

glib::wrapper! {
    pub struct YearViewYearRow(ObjectSubclass<imp::YearViewYearRow>)
        @extends gtk::Widget, gtk::Box,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl YearViewYearRow {
    pub fn new(year: i32, grid_layout: GridLayout) -> Self {
        glib::Object::builder()
            .property("year", year)
            .property("grid-layout", grid_layout)
            .build()
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
