use ui::{refs::Weak, view, SubView, ViewSetup};
use ui_views::CollectionView;

#[view]
pub struct CollectionTestView {
    collection_view: SubView<CollectionView>,
}

impl ViewSetup for CollectionTestView {
    fn setup(self: Weak<Self>) {
        self.collection_view.place.background();
    }
}
