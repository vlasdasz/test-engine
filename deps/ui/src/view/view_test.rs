use gm::flat::IntSize;
use refs::Weak;

use crate::View;

pub trait ViewTest {
    fn test_setup(self: Weak<Self>);
    fn test_size() -> IntSize
    where Self: Sized;
}

impl<T: ?Sized + View> ViewTest for T {
    default fn test_setup(self: Weak<Self>) {}
    default fn test_size() -> IntSize
    where Self: Sized {
        (1000, 1000).into()
    }
}
