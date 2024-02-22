#![allow(incomplete_features)]
#![feature(stmt_expr_attributes)]
#![feature(const_trait_impl)]
#![feature(specialization)]
#![feature(arbitrary_self_types)]

mod benchmark_view;

use anyhow::Result;
use test_engine::{ui::ViewSetup, App};

use crate::benchmark_view::BenchmarkView;

#[tokio::main]
async fn main() -> Result<()> {
    App::start(BenchmarkView::new()).await
}
