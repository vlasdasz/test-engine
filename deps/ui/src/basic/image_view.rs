use crate::{impl_view, view, View, ViewBase};

#[view]
#[derive(Default, Debug)]
pub struct ImageView {}

impl_view!(ImageView);
