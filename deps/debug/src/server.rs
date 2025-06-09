use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use anyhow::Result;
use tokio::{
    net::TcpListener,
    spawn,
    sync::{Mutex, OnceCell},
};

use crate::{MyMessage, connection::Connection};

pub struct DebugServer {
    listener:   TcpListener,
    connection: OnceCell<Connection>,
    started:    Mutex<bool>,
    callback:   Mutex<Option<Box<dyn FnMut(MyMessage) + Send>>>,
}

impl DebugServer {
    pub async fn new(port: u16) -> Result<Self> {
        let listener =
            TcpListener::bind(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), port)).await?;

        Ok(Self {
            listener,
            connection: OnceCell::default(),
            started: Mutex::new(false),
            callback: Mutex::new(None),
        })
    }

    pub async fn start(&'static self) {
        let mut started = self.started.lock().await;

        if *started {
            return;
        }

        spawn(async {
            loop {
                let (stream, addr) = self.listener.accept().await.unwrap();
                println!("Client connected: {addr}");

                if self.connection.get().is_some() {
                    panic!("Connection already exists");
                }

                self.connection
                    .get_or_init(|| async { Connection::new(stream) })
                    .await
                    .on_receive(self.callback.lock().await.take().expect("No callback set"))
                    .await
                    .start()
                    .await;
            }
        });

        *started = true;
    }

    pub async fn on_receive(&'static self, action: impl FnMut(MyMessage) + Send + 'static) {
        let mut callback = self.callback.lock().await;

        if callback.is_some() {
            panic!("Already has callback");
        }

        callback.replace(Box::new(action));
    }

    pub async fn send(&'static self, msg: MyMessage) -> Result<()> {
        let Some(connection) = self.connection.get() else {
            dbg!("No connection");
            return Ok(());
        };

        connection.send(msg).await
    }
}

#[cfg(test)]
mod test {
    use std::{
        net::{IpAddr, Ipv4Addr, SocketAddr},
        time::Duration,
    };

    use anyhow::Result;
    use tokio::{sync::OnceCell, time::sleep};

    use crate::{MyMessage, client::Client, server::DebugServer};

    const PORT: u16 = 57056;

    static SERVER: OnceCell<DebugServer> = OnceCell::const_new();

    async fn server() -> &'static DebugServer {
        SERVER.get_or_init(|| async { DebugServer::new(PORT).await.unwrap() }).await
    }

    static CLIENT: OnceCell<Client> = OnceCell::const_new();

    async fn client() -> &'static Client {
        CLIENT
            .get_or_init(|| async {
                Client::new(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), PORT))
                    .await
                    .unwrap()
            })
            .await
    }

    #[tokio::test]
    async fn test_debug_server() -> Result<()> {
        let server = server().await;

        server
            .on_receive(|msg| {
                dbg!("Server received:");
                dbg!(&msg);
            })
            .await;

        server.start().await;

        server
            .send(MyMessage {
                id:      0,
                content: "bydyn".to_string(),
            })
            .await?;

        let client = client().await;

        client.start().await;

        client
            .on_receive(|msg| {
                dbg!("Client received:");
                dbg!(&msg);
            })
            .await;

        sleep(Duration::from_millis(100)).await;

        server
            .send(MyMessage {
                id:      0,
                content: "to_client".to_string(),
            })
            .await?;

        client
            .send(MyMessage {
                id:      0,
                content: "to_server".to_string(),
            })
            .await?;

        sleep(Duration::from_millis(500)).await;

        Ok(())
    }
}
