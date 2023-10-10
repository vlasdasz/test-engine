#![allow(incomplete_features)]
#![feature(trait_upcasting)]
#![feature(stmt_expr_attributes)]
#![feature(const_trait_impl)]
#![feature(specialization)]
#![feature(let_chains)]
#![feature(arbitrary_self_types)]

use crate::view_tests::button_test::test_button_view;

mod ui_test;
mod view_tests;

fn main() {
    test_button_view();
}
