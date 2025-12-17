#![cfg(not_wasm)]

use inspect::{AppCommand, InspectorCommand, PORT_RANGE};
use log::{debug, error};
use tokio::{spawn, sync::OnceCell};

// static SERVER: OnceCell<Server> = OnceCell::const_new();

type Server = netrun::Server<InspectorCommand, AppCommand>;

static SERVER: OnceCell<Server> = OnceCell::const_new();

async fn server() -> &'static Server {
    SERVER
        .get_or_init(|| async {
            let server = Server::new(PORT_RANGE.start).await.unwrap();

            debug!("Inspect server listening on port: {}", PORT_RANGE.start);

            server
        })
        .await
}

pub struct InspectServer {}

impl InspectServer {
    pub fn start_listening(action: impl FnMut(InspectorCommand) + Send + 'static) {
        spawn(async {
            server().await.start().await;
            server().await.on_receive(action).await;
        });
    }

    pub fn send(command: AppCommand) {
        spawn(async {
            if let Err(err) = server().await.send(command).await {
                error!("Failed to send app command: {err}");
            }
        });
    }
}
