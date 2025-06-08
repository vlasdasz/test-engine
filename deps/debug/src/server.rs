use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    sync::atomic::AtomicBool,
};

use anyhow::Result;
use log::warn;
use tokio::{
    io::AsyncWriteExt,
    net::{
        TcpListener, TcpStream,
        tcp::{OwnedReadHalf, OwnedWriteHalf},
    },
    spawn,
    sync::Mutex,
};

use crate::MyMessage;

pub struct DebugServer {
    listener: TcpListener,
    write:    Mutex<Option<OwnedWriteHalf>>,
    started:  Mutex<bool>,
}

impl DebugServer {
    pub async fn new(port: u16) -> Result<Self> {
        let listener =
            TcpListener::bind(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), port)).await?;

        Ok(Self {
            listener,
            write: Mutex::default(),
            started: Mutex::new(false),
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
                spawn(self.handle_client(stream));
            }
        });

        *started = true;
    }

    pub async fn handle_client(&'static self, stream: TcpStream) -> Result<()> {
        let (read, write) = stream.into_split();

        let mut wr = self.write.lock().await;

        if wr.is_some() {
            panic!("Writer already exits");
        }

        wr.replace(write);

        spawn(async move { self.handle_read(read).await.unwrap() });

        Ok(())
    }

    pub async fn handle_read(&self, read: OwnedReadHalf) -> Result<()> {
        Ok(())
    }

    pub async fn send(&self, message: MyMessage) -> Result<()> {
        let mut write = self.write.lock().await;

        let Some(ref mut write) = *write else {
            dbg!("No write");
            return Ok(());
        };

        let json = serde_json::to_string(&message)?;

        write.write_all(json.as_bytes()).await?;

        Ok(())
    }
}

#[cfg(test)]
mod test {

    use anyhow::Result;
    use tokio::sync::OnceCell;

    use crate::{MyMessage, server::DebugServer};

    static SERVER: OnceCell<DebugServer> = OnceCell::const_new();

    async fn server() -> &'static DebugServer {
        SERVER.get_or_init(|| async { DebugServer::new(4000).await.unwrap() }).await
    }

    #[tokio::test]
    async fn test_debug_server() -> Result<()> {
        let server = server().await;

        server.start().await;

        server
            .send(MyMessage {
                id:      0,
                content: "bydyn".to_string(),
            })
            .await?;

        Ok(())
    }
}
