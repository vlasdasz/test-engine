#![allow(incomplete_features)]
#![feature(trait_upcasting)]
#![feature(stmt_expr_attributes)]
#![feature(const_trait_impl)]
#![feature(specialization)]
#![feature(arbitrary_self_types)]

use anyhow::Result;
use ui::Container;

use crate::views::{
    button::test_button, drop_down::test_drop_down, int_view::test_int_view, layout::test_layout,
    switch::test_switch,
};

mod view_tests;
mod views;

fn main() -> Result<()> {
    test_engine::ViewApp::<Container>::start_with_actor(async {
        test_button().await?;
        test_switch().await?;
        test_layout().await?;
        test_int_view().await?;
        test_drop_down().await?;

        Ok(())
    })
}
