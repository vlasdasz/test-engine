use gm::flat::Size;
use refs::Weak;

use crate::View;

pub trait ViewTest {
    fn test_setup(self: Weak<Self>);
    fn test_size() -> Size<u32>
    where Self: Sized;
}

impl<T: ?Sized + View> ViewTest for T {
    default fn test_setup(self: Weak<Self>) {}
    default fn test_size() -> Size<u32>
    where Self: Sized {
        (600, 600).into()
    }
}
