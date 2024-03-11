use gm::{flat::Size, LossyConvert};
use refs::{weak_from_ref, Weak};
use ui::{view, Sub, ViewData, ViewFrame, ViewSetup, ViewSubviews, ViewTouch, WeakView};

mod test_engine {
    pub(crate) use refs;
    pub(crate) use ui;
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
    scroll:      Sub<ScrollView>,
}

impl ViewSetup for CollectionView {
    fn setup(mut self: Weak<Self>) {
        self.scroll.content_size = (1000, 1500).into();
        self.scroll.place().back();
        self.size_changed().sub(move || {
            self.reload_data();
        });
    }
}

impl CollectionView {
    pub fn set_data_source(mut self: Weak<Self>, data: &(impl CollectionData + 'static)) {
        self.data_source = weak_from_ref(data);
    }

    pub fn reload_data(&mut self) {
        if self.layout.is_table() {
            self.layout();
            return;
        }

        for cell in &mut self.cells {
            cell.remove_from_superview();
        }
        self.cells.clear();

        for i in 0..self.data_source.number_of_cells() {
            let mut cell = self.data_source.make_cell();
            self.data_source.setup_cell_for_index(cell.as_any_mut(), i);
            let mut cell = self.scroll.add_subview(cell);
            cell.base_mut().label = format!("Table cell: {}", cell.label());
            cell.enable_touch_low_priority();
            let mut this = weak_from_ref(self);
            cell.touch().up_inside.sub(move || this.data_source.cell_selected(i));
            self.cells.push(cell);
        }
    }

    fn layout(&mut self) {
        match self.layout {
            CollectionLayout::Table => self.table_layout(),
            CollectionLayout::Cards => self.cards_layout(),
        }
    }

    fn table_layout(&mut self) {
        for cell in &mut self.cells {
            cell.remove_from_superview();
        }
        self.cells.clear();

        assert!(
            self.data_source.is_ok(),
            "Set data source for CollectionView before using"
        );

        let number_of_cells = self.data_source.number_of_cells();

        if number_of_cells == 0 {
            return;
        }

        let cell_height = self.data_source.size_for_index(0).height;

        self.scroll.content_size.width = self.width();
        self.scroll.content_size.height = number_of_cells as f32 * cell_height;

        let width = self.width();

        let mut content_start = -self.scroll.content_offset.y;
        let content_end = content_start + self.scroll.height();

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
            cell.set_frame((0, index as f32 * cell_height, width, cell_height));
            cell.enable_touch_low_priority();
            let mut this = weak_from_ref(self);
            cell.touch().began.sub(move || this.data_source.cell_selected(index));
            self.cells.push(cell);
        }
    }

    fn cards_layout(&mut self) {
        self.scroll.content_size = self.size();

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
