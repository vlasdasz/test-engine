#![allow(incomplete_features)]
#![feature(trait_upcasting)]
#![feature(stmt_expr_attributes)]
#![feature(const_trait_impl)]
#![feature(specialization)]
#![feature(arbitrary_self_types)]

use anyhow::Result;
use ui::Container;

use crate::{button::test_button, layout::test_layout, switch::test_switch};

mod button;
mod layout;
mod switch;
mod view_tests;

fn main() -> Result<()> {
    test_engine::ViewApp::<Container>::start_with_actor(async {
        test_button().await?;
        test_switch().await?;
        test_layout().await?;

        Ok(())
    })
}
