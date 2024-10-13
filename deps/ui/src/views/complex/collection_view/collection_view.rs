use gm::{flat::Size, LossyConvert};
use refs::{weak_from_ref, Weak};
use ui_proc::view;

use crate::{
    view::{ViewData, ViewFrame, ViewSubviews, ViewTouch},
    Setup, View, WeakView,
};

mod test_engine {
    pub(crate) use educe;
    pub(crate) use refs;

    pub(crate) use crate as ui;
}

use crate::{CollectionData, ScrollView};

#[derive(Default, Debug)]
pub enum CollectionLayout {
    #[default]
    Table,
    Cards,
}

impl CollectionLayout {
    pub fn is_table(&self) -> bool {
        matches!(self, Self::Table)
    }

    pub fn is_cards(&self) -> bool {
        matches!(self, Self::Cards)
    }
}

#[view]
pub struct CollectionView {
    pub layout: CollectionLayout,

    data_source: Weak<dyn CollectionData>,
    cells:       Vec<WeakView>,

    #[init]
    pub(crate) scroll: ScrollView,
}

impl Setup for CollectionView {
    fn setup(mut self: Weak<Self>) {
        self.scroll.set_content_size((1000, 1500));
        self.scroll.place().back();
        self.size_changed().sub(move || {
            self.reload_data();
        });
    }
}

impl CollectionView {
    pub fn set_data_source(mut self: Weak<Self>, data: &(impl CollectionData + 'static)) -> Weak<Self> {
        self.data_source = weak_from_ref(data);
        self
    }

    pub fn reload_data(&mut self) {
        self.layout();
    }

    fn layout(&mut self) {
        for cell in &mut self.cells {
            cell.remove_from_superview();
        }
        self.cells.clear();

        assert!(
            self.data_source.is_ok(),
            "Set data source for: {} before using",
            self.label()
        );

        match self.layout {
            CollectionLayout::Table => self.table_layout(),
            CollectionLayout::Cards => self.cards_layout(),
        }
    }

    fn table_layout(&mut self) {
        let number_of_cells = self.data_source.number_of_cells();

        if number_of_cells == 0 {
            return;
        }

        let cell_height = self.data_source.size_for_index(0).height;
        let table_height = number_of_cells.lossy_convert() * cell_height;

        let width = self.width();
        self.scroll.set_content_size((width, table_height));
        let width = self.width();

        let mut content_start = -self.scroll.base_view().content_offset;
        let content_end = content_start + table_height;

        if content_start < 0.0 {
            content_start = 0.0;
        }

        let content_height = content_end - content_start;

        if content_height <= 0.0 {
            return;
        }

        let first_cell_index: usize = (content_start / cell_height).floor().lossy_convert();
        let number_of_cells_fit: usize = (content_height / cell_height).ceil().lossy_convert();

        let mut last_cell_index = first_cell_index + number_of_cells_fit;

        if last_cell_index + 1 > number_of_cells {
            last_cell_index = number_of_cells - 1;
        }

        for index in first_cell_index..=last_cell_index {
            let cell = self.data_source.make_cell();

            let mut cell = self.scroll.add_subview(cell);

            self.data_source.setup_cell_for_index(cell.as_any_mut(), index);
            cell.set_frame((0, index.lossy_convert() * cell_height, width, cell_height));
            cell.enable_touch_low_priority();
            let mut this = weak_from_ref(self);
            cell.touch().began.sub(move || this.data_source.cell_selected(index));
            self.cells.push(cell);
        }
    }

    fn cards_layout(&mut self) {
        for i in 0..self.data_source.number_of_cells() {
            let mut cell = self.data_source.make_cell();
            self.data_source.setup_cell_for_index(cell.as_any_mut(), i);
            let mut cell = self.scroll.add_subview(cell);
            cell.base_view_mut().view_label = format!("Table cell: {}", cell.label());
            cell.enable_touch_low_priority();
            let mut this = weak_from_ref(self);
            cell.touch().up_inside.sub(move || this.data_source.cell_selected(i));
            self.cells.push(cell);
        }

        let size = self.size();
        self.scroll.set_content_size(size);

        let area_width = self.width();

        let sizes: Vec<Size> = (0..self.data_source.number_of_cells())
            .map(|i| self.data_source.size_for_index(i))
            .collect();

        let mut rectangles = Vec::new();
        let mut x = 0.0;
        let mut y = 0.0;
        for size in &sizes {
            let rectangle = (x, y, size.width, size.height);
            rectangles.push(rectangle);
            x += size.width;
            if x + size.width > area_width {
                x = 0.0;
                y += size.height;
            }
        }

        for (i, cell) in self.cells.iter_mut().enumerate() {
            cell.set_frame(rectangles[i]);
        }
    }
}
