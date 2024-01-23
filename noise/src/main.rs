#![allow(incomplete_features)]
#![feature(stmt_expr_attributes)]
#![feature(const_trait_impl)]
#![feature(specialization)]
#![feature(arbitrary_self_types)]

mod noise_view;

use anyhow::Result;

use crate::noise_view::NoiseView;

fn main() -> Result<()> {
    test_engine::ViewApp::<NoiseView>::start()
}
