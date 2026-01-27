mod anchor_view;
mod parsing;
mod placer_view;

use anyhow::Result;

use crate::inspect::{anchor_view::test_anchor_view, parsing::test_inspect_parsing, placer_view::test_placer_view};

pub(crate) async fn test_inspect() -> Result<()> {
    test_placer_view().await?;
    test_anchor_view().await?;
    test_inspect_parsing().await?;

    Ok(())
}
