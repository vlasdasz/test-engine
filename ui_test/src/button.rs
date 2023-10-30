#![allow(incomplete_features)]
#![feature(trait_upcasting)]
#![feature(stmt_expr_attributes)]
#![feature(const_trait_impl)]
#![feature(specialization)]
#![feature(arbitrary_self_types)]
#![feature(process_exitcode_internals)]

use std::process::ExitCode;

use crate::view_tests::button_test::test_button_view;

mod ui_test;
mod view_tests;

fn main() -> ExitCode {
    test_button_view()
}
