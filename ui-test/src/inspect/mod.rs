mod parsing;

use anyhow::Result;

use crate::inspect::parsing::test_inspect_parsing;

pub(crate) async fn test_inspect() -> Result<()> {
    test_inspect_parsing().await?;

    Ok(())
}
