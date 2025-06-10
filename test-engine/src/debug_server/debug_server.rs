use anyhow::Result;
use debug::{DebugMessage, DebugServer};
use tokio::sync::OnceCell;

static SERVER: OnceCell<DebugServer> = OnceCell::const_new();

async fn server() -> &'static DebugServer {
    SERVER
        .get_or_init(|| async { DebugServer::new(debug::DEFAULT_PORT).await.unwrap() })
        .await
}

pub async fn start_listtening_for_debug_client() {
    server().await.start().await;
}

pub async fn on_debug_client_message(action: impl FnMut(DebugMessage) + Send + 'static) {
    server().await.on_receive(action).await;
}

pub async fn send_to_debug_client(msg: impl Into<DebugMessage>) -> Result<()> {
    server().await.send(msg).await
}
