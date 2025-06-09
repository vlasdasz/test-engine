use anyhow::Result;
use log::info;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{
        TcpStream,
        tcp::{OwnedReadHalf, OwnedWriteHalf},
    },
    spawn,
    sync::Mutex,
};

use crate::message::DebugMessage;

pub struct Connection {
    callback: Mutex<Option<Box<dyn FnMut(DebugMessage) + Send>>>,
    stream:   Mutex<Option<TcpStream>>,
    write:    Mutex<Option<OwnedWriteHalf>>,
}

impl Connection {
    pub fn new(stream: TcpStream) -> Self {
        Self {
            callback: Mutex::const_new(None),
            stream:   Mutex::new(Some(stream)),
            write:    Mutex::new(None),
        }
    }

    pub async fn start(&'static self) {
        let mut stream = self.stream.lock().await;

        if stream.is_none() {
            return;
        }

        let (read, write) = stream.take().unwrap().into_split();

        let mut wr = self.write.lock().await;

        if wr.is_some() {
            panic!("Writer already exits");
        }

        wr.replace(write);

        spawn(async move { self.handle_read(read).await.unwrap() });
    }

    pub async fn handle_read(&self, mut reader: OwnedReadHalf) -> Result<()> {
        loop {
            let mut buf = vec![0u8; 4096];
            let n = reader.read(&mut buf).await?;

            if n == 0 {
                continue;
            }

            let json_str = std::str::from_utf8(&buf[..n])?;
            let msg: DebugMessage = serde_json::from_str(json_str)?;
            info!("Received: {:?}", msg);
            self.callback.lock().await.as_mut().unwrap()(msg)
        }
    }

    pub async fn on_receive(
        &'static self,
        action: impl FnMut(DebugMessage) + Send + 'static,
    ) -> &'static Self {
        let mut callback = self.callback.lock().await;

        if callback.is_some() {
            panic!("Already has callback");
        }

        callback.replace(Box::new(action));

        self
    }

    pub async fn send(&'static self, msg: impl Into<DebugMessage>) -> Result<()> {
        let msg = msg.into();

        let json = serde_json::to_string(&msg)?;

        let mut writer = self.write.lock().await;
        let writer = writer.as_mut().expect("No writer. Did you start the connection?");

        writer.write_all(json.as_bytes()).await?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use tokio::sync::OnceCell;

    use crate::connection::Connection;

    static _CONNECTION: OnceCell<Connection> = OnceCell::const_new();
}
