mod server;

use anyhow::Result;
use log::info;
use serde::{Deserialize, Serialize};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    spawn,
    sync::{Mutex, MutexGuard, mpsc::channel},
};
use vents::Event;

static EVENT: Mutex<Event<MyMessage>> = Mutex::const_new(Event::const_default());

#[derive(Serialize, Deserialize, Debug)]
pub struct MyMessage {
    id:      u32,
    content: String,
}

pub async fn new_message_event() -> MutexGuard<'static, Event<MyMessage>> {
    EVENT.lock().await
}

pub fn start_listening() {
    spawn(async { start_listening_internal().await.unwrap() });
}

async fn start_listening_internal() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:4000").await?;
    info!("Server listening on 127.0.0.1:4000");

    let (se, mut re) = channel::<MyMessage>(1);

    spawn(async move {
        loop {
            let Some(message) = re.recv().await else {
                dbg!("nothing");
                continue;
            };
            dbg!("Mossaggee!!");
            dbg!(&message);
            EVENT.lock().await.trigger(message);
        }
    });

    loop {
        let (mut socket, _) = listener.accept().await?;
        let mut buf = vec![0u8; 4096];
        let n = socket.read(&mut buf).await?;

        // socket.split()

        if n == 0 {
            continue;
        }

        let json_str = std::str::from_utf8(&buf[..n])?;
        let msg: MyMessage = serde_json::from_str(json_str)?;
        info!("Received: {:?}", msg);
        se.send(msg).await?;
    }
}

pub async fn send_message(my_message: MyMessage) -> Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:4000").await?;

    let json = serde_json::to_string(&my_message)?;

    stream.write_all(json.as_bytes()).await?;
    Ok(())
}

#[cfg(test)]
mod test {
    use std::time::Duration;

    use anyhow::Result;
    use tokio::time::sleep;

    use crate::{MyMessage, new_message_event, send_message, start_listening};

    #[tokio::test]
    async fn test_debug_channel() -> Result<()> {
        start_listening();

        sleep(Duration::from_millis(100)).await;

        new_message_event().await.val(|event| {
            dbg!("OO ivabntockooo!!");
            dbg!(&event);
        });

        send_message(MyMessage {
            id:      0,
            content: "oskolok".to_string(),
        })
        .await?;

        send_message(MyMessage {
            id:      0,
            content: "oskolok2".to_string(),
        })
        .await?;

        sleep(Duration::from_millis(100)).await;

        Ok(())
    }
}
