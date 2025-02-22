#![allow(incomplete_features)]
#![feature(stmt_expr_attributes)]
#![feature(const_trait_impl)]
#![feature(specialization)]
#![feature(arbitrary_self_types)]

mod app;
mod benchmark_view;

use test_engine::{register_app, test_engine_start_app};

use crate::app::BenchmarkApp;

register_app!(BenchmarkApp);

fn main() {
    test_engine_start_app();
}
