use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use anyhow::Result;
use log::info;
use tokio::{
    net::TcpListener,
    spawn,
    sync::{Mutex, OnceCell},
};

use crate::{Callback, connection::Connection, message::DebugMessage};

pub struct DebugServer {
    listener:   TcpListener,
    connection: OnceCell<Connection>,
    started:    Mutex<bool>,
    callback:   Mutex<Option<Callback>>,
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
                info!("Client connected: {addr}");

                assert!(self.connection.get().is_none(), "Connection already exists");

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

    pub async fn on_receive(&'static self, action: impl FnMut(DebugMessage) + Send + 'static) {
        let mut callback = self.callback.lock().await;

        assert!(callback.is_none(), "Already has callback");

        callback.replace(Box::new(action));
    }

    pub async fn send(&'static self, msg: impl Into<DebugMessage>) -> Result<()> {
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
        ops::Deref,
        time::Duration,
    };

    use anyhow::Result;
    use tokio::{
        spawn,
        sync::{Mutex, OnceCell},
        time::sleep,
    };

    use crate::{
        DEFAULT_PORT,
        client::Client,
        command::Command,
        server::{DebugMessage, DebugServer},
    };

    static SERVER: OnceCell<DebugServer> = OnceCell::const_new();

    async fn server() -> &'static DebugServer {
        SERVER
            .get_or_init(|| async { DebugServer::new(DEFAULT_PORT).await.unwrap() })
            .await
    }

    static CLIENT: OnceCell<Client> = OnceCell::const_new();

    static DATA: Mutex<Vec<DebugMessage>> = Mutex::const_new(Vec::new());

    async fn client() -> &'static Client {
        CLIENT
            .get_or_init(|| async {
                Client::new(SocketAddr::new(
                    IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
                    DEFAULT_PORT,
                ))
                .await
                .unwrap()
            })
            .await
    }

    #[tokio::test]
    async fn test_debug_server() -> Result<()> {
        let sv = server().await;

        sv.on_receive(|msg| {
            spawn(async { DATA.lock().await.push(msg) });
        })
        .await;

        sv.start().await;

        server()
            .await
            .send(DebugMessage {
                id:      0,
                msg:     "bydyn".to_string(),
                command: Command::Ping,
            })
            .await?;

        let cl = client().await;

        cl.start().await;

        cl.on_receive(|msg| {
            spawn(async { DATA.lock().await.push(msg) });
        })
        .await;

        sleep(Duration::from_millis(100)).await;

        server()
            .await
            .send(DebugMessage {
                id:      35,
                msg:     "to_client".to_string(),
                command: Command::Ping,
            })
            .await?;

        client()
            .await
            .send(DebugMessage {
                id:      89,
                msg:     "to_server".to_string(),
                command: Command::Ping,
            })
            .await?;

        sleep(Duration::from_millis(10)).await;

        client().await.send(Command::Ping).await?;

        sleep(Duration::from_millis(200)).await;

        assert_eq!(
            DATA.lock().await.deref(),
            &vec![
                DebugMessage {
                    id:      35,
                    msg:     "to_client".to_string(),
                    command: Command::Ping,
                },
                DebugMessage {
                    id:      89,
                    msg:     "to_server".to_string(),
                    command: Command::Ping,
                },
                Command::Ping.into()
            ]
        );

        Ok(())
    }
}
