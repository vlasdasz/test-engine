#![allow(incomplete_features)]
#![feature(trait_upcasting)]
#![feature(stmt_expr_attributes)]
#![feature(const_trait_impl)]
#![feature(specialization)]
#![feature(arbitrary_self_types)]

use std::process::ExitCode;

mod view_tests;

fn main() -> ExitCode {
    ExitCode::SUCCESS
}
