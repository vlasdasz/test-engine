use test_engine::gm::flat::Size;
use ui::{
    refs::{Own, ToWeak, Weak},
    view, Container, SubView, View, ViewSetup,
};
use ui_views::{collection_data, BackButton, CollectionData, CollectionLayout, CollectionView};

#[view]
pub struct CollectionTestView {
    back:            SubView<BackButton>,
    collection_view: SubView<CollectionView>,
}

impl ViewSetup for CollectionTestView {
    fn setup(mut self: Weak<Self>) {
        self.back.place.size(50, 50).br(10);

        self.collection_view.place.background();
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

    fn cell_for_index(&self, _index: usize) -> Own<dyn View> {
        Own::<Container>::default()
    }

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
    test_engine::ViewApp::<CollectionTestView>::start()
}
