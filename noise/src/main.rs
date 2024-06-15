#![allow(incomplete_features)]
#![feature(stmt_expr_attributes)]
#![feature(const_trait_impl)]
#![feature(specialization)]
#![feature(arbitrary_self_types)]

mod noise_view;

use anyhow::Result;
use test_engine::{ui::ViewSetup, App};

use crate::noise_view::NoiseView;

#[tokio::main]
async fn main() -> Result<()> {
    App::start(NoiseView::new()).await?;
    Ok(())
}
