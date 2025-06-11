use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use anyhow::Result;
use debug::{Client, Command, DEFAULT_PORT, DebugMessage, LevelCommand};
use test_engine::{
    dispatch::on_main,
    refs::Weak,
    ui::{Button, HasText, Setup, ViewData, async_link_button, view},
};
use tokio::sync::OnceCell;

use crate::interface::level_view::LevelView;

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
    connect:         Button,
    ping:            Button,
    get_level_scale: Button,
    level:           LevelView,
}

impl MainView {
    async fn connect_pressed(self: Weak<Self>) -> Result<()> {
        client()
            .await
            .on_receive(move |msg| on_main(move || self.on_receive(msg)))
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
                command: Command::Ping,
            })
            .await
    }

    async fn get_level_scale_pressed(self: Weak<Self>) -> Result<()> {
        client().await.send(LevelCommand::GetScale).await
    }

    fn on_receive(mut self: Weak<Self>, msg: DebugMessage) {
        dbg!("Racovko");
        dbg!(&msg);

        let Command::Level(level) = msg.command else {
            return;
        };

        match level {
            LevelCommand::SendScale(scale) => {
                self.level.label.set_text(scale);
            }
            _ => unimplemented!(),
        }
    }
}

impl Setup for MainView {
    fn setup(mut self: Weak<Self>) {
        self.place().all_ver();

        self.connect.set_text("Connect");
        async_link_button!(self.connect, connect_pressed);

        self.ping.set_text("Ping");
        async_link_button!(self.ping, send_pressed);

        self.get_level_scale.set_text("Get Level Scale");
        async_link_button!(self.get_level_scale, get_level_scale_pressed);
    }
}
