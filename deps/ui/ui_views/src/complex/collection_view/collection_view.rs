use gm::flat::Size;
use refs::{ToWeak, Weak};
use ui::{view, SubView, View, ViewCallbacks, ViewFrame, ViewSetup, ViewSubviews, ViewTouch};

use crate::{debug_view::DebugView, CollectionData, ScrollView};

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
    pub data_source: Weak<dyn CollectionData>,
    pub layout:      CollectionLayout,
    cells:           Vec<Weak<dyn View>>,
    scroll:          SubView<ScrollView>,
}

impl ViewSetup for CollectionView {
    fn setup(mut self: Weak<Self>) {
        self.scroll.content_size = (1000, 1500).into();
        self.scroll.place.back();
    }
}

impl CollectionView {
    pub fn set_priority(&mut self, p: usize) {
        self.priority = p;
        self.scroll.priority = p;
    }

    pub fn reload_data(&mut self) {
        for cell in &mut self.cells {
            cell.remove_from_superview();
        }
        self.cells.clear();
        for i in 0..self.data_source.number_of_cells() {
            let mut cell = self.data_source.make_cell();
            self.data_source.setup_cell_for_index(cell.as_any_mut(), i);
            let cell = self.scroll.add_subview(cell);
            cell.enable_touch_low_priority();
            let mut this = self.weak();
            cell.on_touch_began.sub(move || this.data_source.cell_selected(i));
            self.cells.push(cell);
        }
        if self.layout.is_table() {
            self.scroll.content_size.height =
                self.data_source.number_of_cells() as f32 * self.data_source.size_for_index(0).height;
            self.scroll.content_size.width = self.width();
        }
    }

    fn layout(&mut self) {
        match self.layout {
            CollectionLayout::Table => self.table_layout(),
            CollectionLayout::Cards => self.cards_layout(),
        }
    }

    fn table_layout(&mut self) {
        self.scroll.content_size.width = self.width();
        self.scroll.content_size.height =
            self.data_source.number_of_cells() as f32 * self.data_source.size_for_index(0).height;

        let mut last_y: f32 = 0.0;
        let width = self.width() - 60.0;

        DebugView::current().set_custom("Table size", self.frame().size.to_string());

        for (index, cell) in self.cells.iter_mut().enumerate() {
            let height = self.data_source.size_for_index(index).height;
            cell.set_frame((0, last_y, width, height));
            last_y += height;
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

impl ViewCallbacks for CollectionView {
    fn update(&mut self) {
        self.layout()
    }
}
