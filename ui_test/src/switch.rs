#![allow(incomplete_features)]
#![feature(trait_upcasting)]
#![feature(stmt_expr_attributes)]
#![feature(const_trait_impl)]
#![feature(specialization)]
#![feature(arbitrary_self_types)]
#![feature(process_exitcode_internals)]

use std::process::ExitCode;

use crate::view_tests::switch_test::test_switch_view;

mod view_tests;

fn main() -> ExitCode {
    test_switch_view()
}
