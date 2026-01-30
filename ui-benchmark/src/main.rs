#![allow(incomplete_features)]
#![feature(stmt_expr_attributes)]
#![feature(specialization)]
#![feature(arbitrary_self_types)]

mod benchmark_view;

use anyhow::Result;
use test_engine::{AppRunner, ui::UIDrawer};

use crate::benchmark_view::BenchmarkView;

fn main() -> Result<()> {
    AppRunner::start_with_actor(async {
        UIDrawer::init_test_view::<BenchmarkView>();

        Ok(())
    })
}
