use std::any::Any;

use test_engine::gm::flat::Size;
use ui::{
    refs::{Own, Weak},
    view, Container, SubView, View, ViewData, ViewSetup,
};
use ui_views::{collection_data, CollectionData, CollectionLayout, CollectionView};

#[view]
pub struct CollectionTestView {
    collection_view: SubView<CollectionView>,
}

impl ViewSetup for CollectionTestView {
    fn setup(mut self: Weak<Self>) {
        self.collection_view.place().back();
        self.collection_view.layout = CollectionLayout::Cards;
        self.collection_view.data_source = collection_data!(self);
        self.collection_view.reload_data();
    }
}

static RECTANGLE_SIZES: [Size; 9] = [
    Size {
        width:  100.0,
        height: 100.0,
    },
    Size {
        width:  100.0,
        height: 100.0,
    },
    Size {
        width:  100.0,
        height: 100.0,
    },
    Size {
        width:  100.0,
        height: 100.0,
    },
    Size {
        width:  100.0,
        height: 100.0,
    },
    Size {
        width:  100.0,
        height: 100.0,
    },
    Size {
        width:  100.0,
        height: 100.0,
    },
    Size {
        width:  100.0,
        height: 100.0,
    },
    Size {
        width:  100.0,
        height: 100.0,
    },
];

impl CollectionData for CollectionTestView {
    fn number_of_cells(&self) -> usize {
        RECTANGLE_SIZES.len()
    }

    fn make_cell(&self) -> Own<dyn View> {
        Own::<Container>::default()
    }

    fn setup_cell_for_index(&self, _cell: &mut dyn Any, _index: usize) {}

    fn size_for_index(&self, index: usize) -> Size {
        RECTANGLE_SIZES[index]
    }

    fn cell_selected(&mut self, index: usize) {
        dbg!(index);
    }
}

#[ignore]
#[test]
fn test() {
    test_engine::ViewApp::<CollectionTestView>::start().unwrap()
}
