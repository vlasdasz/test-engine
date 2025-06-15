use anyhow::Result;
use debug::{DebugMessage, DebugServer};
use tokio::sync::OnceCell;

static SERVER: OnceCell<DebugServer> = OnceCell::const_new();

async fn server() -> Result<&'static DebugServer> {
    SERVER
        .get_or_try_init(|| async { DebugServer::new(debug::DEFAULT_PORT).await })
        .await
}

pub async fn start_listtening_for_debug_client() -> Result<()> {
    server().await?.start().await;
    Ok(())
}

pub async fn on_debug_client_message(action: impl FnMut(DebugMessage) + Send + 'static) -> Result<()> {
    server().await?.on_receive(action).await;
    Ok(())
}

pub fn send_to_debug_client(msg: impl Into<DebugMessage> + Send + 'static) {
    tokio::spawn(async { server().await.unwrap().send(msg).await.unwrap() });
}
