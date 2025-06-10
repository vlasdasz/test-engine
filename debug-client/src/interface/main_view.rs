use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use anyhow::Result;
use debug::{Client, DEFAULT_PORT, DebugMessage};
use test_engine::{
    refs::Weak,
    ui::{Button, HasText, Setup, ViewData, async_link_button, view},
};
use tokio::sync::OnceCell;

static CLIENT: OnceCell<Client> = OnceCell::const_new();

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

#[view]
pub struct MainView {
    #[init]
    connect: Button,
    send:    Button,
}

impl MainView {
    async fn connect_pressed(self: Weak<Self>) -> Result<()> {
        client()
            .await
            .on_receive(|msg| {
                dbg!("Racovko");
                dbg!(&msg);
            })
            .await
            .start()
            .await;
        Ok(())
    }

    async fn send_pressed(self: Weak<Self>) -> Result<()> {
        client()
            .await
            .send(DebugMessage {
                id:      111,
                msg:     "Plottiiii".to_string(),
                command: Default::default(),
            })
            .await
    }
}

impl Setup for MainView {
    fn setup(mut self: Weak<Self>) {
        self.place().all_ver();

        self.connect.set_text("Connect");
        async_link_button!(self.connect, connect_pressed);

        self.send.set_text("Send");
        async_link_button!(self.send, send_pressed);
    }
}
