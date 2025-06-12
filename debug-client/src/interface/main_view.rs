use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use anyhow::Result;
use debug::{Client, Command, DEFAULT_PORT, DebugMessage, LevelCommand};
use test_engine::{
    dispatch::on_main,
    refs::Weak,
    ui::{Button, HasText, Setup, ViewData, async_link_button, view},
};
use tokio::{spawn, sync::OnceCell};

use crate::interface::{level_view::LevelView, ui_view::UIView};

static CLIENT: OnceCell<Client> = OnceCell::const_new();

pub(crate) async fn client() -> &'static Client {
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
    ping:  Button,
    level: LevelView,
    ui:    UIView,
}

impl MainView {
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

    fn on_receive(mut self: Weak<Self>, msg: DebugMessage) {
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

        self.ping.set_text("Ping");
        async_link_button!(self.ping, send_pressed);

        spawn(async move {
            client()
                .await
                .on_receive(move |msg| on_main(move || self.on_receive(msg)))
                .await
                .start()
                .await;
        });
    }
}
