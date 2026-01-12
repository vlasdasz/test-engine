mod parsing;
mod anchor_view;

use anyhow::Result;
use crate::inspect::anchor_view::test_anchor_view;
use crate::inspect::parsing::test_inspect_parsing;

pub(crate) async fn test_inspect() -> Result<()> {
    test_anchor_view().await?;
    test_inspect_parsing().await?;

    Ok(())
}
