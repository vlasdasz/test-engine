#![allow(incomplete_features)]
#![feature(stmt_expr_attributes)]
#![feature(const_trait_impl)]
#![feature(specialization)]
#![feature(arbitrary_self_types)]

mod app;
mod benchmark_view;

use test_engine::register_app;

use crate::app::BenchmarkApp;
pub use crate::benchmark_view::BenchmarkView;

register_app!(BenchmarkApp);
