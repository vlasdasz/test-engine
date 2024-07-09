#![feature(arbitrary_self_types)]

use ui::Button;
use ui_proc::view;

mod test_engine {
    pub(crate) use educe;
    pub(crate) use refs;
    pub(crate) use ui;
}

#[view]
struct _ProcView {
    _button:      Button,
    #[init]
    _weak_button: Button,
}

fn main() {}
