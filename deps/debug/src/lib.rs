use anyhow::Result;
use log::info;
use serde::{Deserialize, Serialize};
use tokio::{io::AsyncReadExt, net::TcpListener, spawn, sync::mpsc::channel};

#[derive(Serialize, Deserialize, Debug)]
struct MyMessage {
    id:      u32,
    content: String,
}

pub fn start_listening() {
    spawn(async { start_listening_internal().await.unwrap() });
}

async fn start_listening_internal() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:4000").await?;
    info!("Server listening on 127.0.0.1:4000");

    let (se, re) = channel::<MyMessage>(1);

    loop {
        let (mut socket, _) = listener.accept().await?;
        let mut buf = vec![0u8; 4096];
        let n = socket.read(&mut buf).await?;

        if n == 0 {
            continue;
        }

        let json_str = std::str::from_utf8(&buf[..n])?;
        let msg: MyMessage = serde_json::from_str(json_str)?;
        info!("Received: {:?}", msg);
    }
}
