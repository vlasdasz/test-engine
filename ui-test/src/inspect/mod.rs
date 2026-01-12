mod anchor_view;
mod parsing;

use anyhow::Result;

use crate::inspect::{anchor_view::test_anchor_view, parsing::test_inspect_parsing};

pub(crate) async fn test_inspect() -> Result<()> {
    test_anchor_view().await?;
    test_inspect_parsing().await?;

    Ok(())
}
