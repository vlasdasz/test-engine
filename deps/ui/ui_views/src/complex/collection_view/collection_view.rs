use refs::Weak;
use ui::{view, ViewSetup};

#[view]
pub struct CollectionView {}

impl ViewSetup for CollectionView {
    fn setup(self: Weak<Self>) {
        //self.add_subview(view)
    }
}
