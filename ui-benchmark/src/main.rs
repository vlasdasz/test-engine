#![allow(incomplete_features)]
#![feature(stmt_expr_attributes)]
#![feature(const_trait_impl)]
#![feature(specialization)]
#![feature(arbitrary_self_types)]

mod benchmark_view;

use test_engine::test_engine_start_app;
pub use ui_benchmark::test_engine_create_app;

fn main() {
    test_engine_start_app();
}
