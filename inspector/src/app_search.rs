use std::net::Ipv4Addr;

use inspect::{AppCommand, InspectorCommand, PORT_RANGE};
use tokio::sync::OnceCell;

pub type Client = netrun::Client<AppCommand, InspectorCommand>;

static CLIENT: OnceCell<Client> = OnceCell::const_new();

pub(crate) async fn client() -> &'static Client {
    CLIENT
        .get_or_init(|| async { Client::new((Ipv4Addr::LOCALHOST, PORT_RANGE.start)).await.unwrap() })
        .await
}

#[cfg(test)]
mod test {
    use anyhow::Result;
    use test_engine::dispatch::sleep;

    use super::*;

    #[ignore]
    #[tokio::test]
    async fn test_find_app() -> Result<()> {
        client().await.start().await;
        client().await.send(InspectorCommand::Ping).await?;
        client()
            .await
            .on_receive(|command| {
                dbg!(&command);
            })
            .await;

        sleep(20.0).await;

        // let app = Search::find_app().await?;
        //
        // app.send(InspectorCommand::Ping).await?;

        Ok(())
    }
}
