mod alert_err;
mod alert_ios;
mod alert_not_ios;
mod back_button;
mod collection_view;
mod consent;
mod dpad_view;
mod drawing_view;
mod drop_down;
mod form_view;
mod highlight_view;
mod input_view;
mod keyboard_view;
mod labeled;
mod number_view;
mod point_view;
mod question;
mod spinner;
mod stick_view;
mod table_view;

pub mod alert {
    #[cfg(target_os = "ios")]
    pub use super::alert_ios::*;
    #[cfg(not(target_os = "ios"))]
    pub use super::alert_not_ios::*;
}

pub use alert::*;
pub use alert_err::*;
pub use back_button::*;
pub use collection_view::*;
pub use consent::*;
pub use dpad_view::DPadView;
pub use drawing_view::DrawingView;
pub use drop_down::*;
pub use form_view::FormView;
pub use highlight_view::*;
pub use input_view::*;
pub use keyboard_view::*;
pub use labeled::Labeled;
pub use number_view::NumberView;
pub use point_view::PointView;
pub use question::Question;
pub use spinner::*;
pub use stick_view::StickView;
pub use table_view::*;
