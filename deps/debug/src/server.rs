use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use anyhow::Result;
use tokio::{
    net::{
        TcpListener, TcpStream,
        tcp::{OwnedReadHalf, OwnedWriteHalf},
    },
    spawn,
    sync::Mutex,
};

pub struct DebugServer {
    listener: TcpListener,
    write:    Mutex<Option<OwnedWriteHalf>>,
}

impl DebugServer {
    pub async fn new(port: u16) -> Result<Self> {
        let listener =
            TcpListener::bind(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), port)).await?;

        Ok(Self {
            listener,
            write: Mutex::default(),
        })
    }

    pub async fn start(&'static self) -> Result<()> {
        loop {
            let (stream, addr) = self.listener.accept().await?;
            println!("Client connected: {addr}");
            spawn(self.handle_client(stream));
        }
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

    pub async fn handle_write(&self, read: OwnedWriteHalf) -> Result<()> {
        Ok(())
    }
}
