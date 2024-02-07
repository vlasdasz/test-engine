#![allow(incomplete_features)]
#![feature(stmt_expr_attributes)]
#![feature(const_trait_impl)]
#![feature(specialization)]
#![feature(arbitrary_self_types)]

mod benchmark_view;

use anyhow::Result;

use crate::benchmark_view::BenchmarkView;

fn main() -> Result<()> {
    old_engine::ViewApp::<BenchmarkView>::start()
}
