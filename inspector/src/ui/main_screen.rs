use std::net::IpAddr;

use anyhow::{Result, anyhow, bail};
use hreads::log_spawn;
use inspect::{AppCommand, InspectorCommand, SystemResponse, UIRequest, UIResponse};
use log::{debug, info};
use test_engine::{
    dispatch::on_main,
    refs::Weak,
    ui::{
        Alert, AlertErr,
        Anchor::{Right, Top},
        Button, DropDown, HasText, Setup, Spinner, ViewData, async_link_button, view,
    },
};

use crate::ui::{common::ValueView, inspect::UIRepresentView};

type Client = netrun::Client<AppCommand, InspectorCommand>;

#[view]
pub struct MainScreen {
    current_client: Option<Client>,

    #[init]
    scan:    Button,
    clients: DropDown<IpAddr>,

    play_sound:     Button,
    get_ui:         Button,
    ui_scale_value: ValueView,

    ui_represent: UIRepresentView,
}

impl Setup for MainScreen {
    fn setup(mut self: Weak<Self>) {
        self.scan.set_text("Scan").place().tl(10).size(100, 50);
        async_link_button!(self.scan, scan_tapped);

        self.clients.place().at_right(self.scan, 10);

        self.play_sound.set_text("Play Sound").place().size(200, 50).tr(10);
        async_link_button!(self.play_sound, play_sound_tapped);

        self.get_ui.set_text("Get UI");
        self.get_ui.place().below(self.play_sound, 10);
        async_link_button!(self.get_ui, get_ui_tapped);

        self.ui_scale_value
            .set_title("UI scale")
            .place()
            .r(10)
            .anchor(Top, self.get_ui, 10)
            .size(100, 100);

        self.ui_scale_value.on_change.val_async(move |val| async move {
            {
                self.scale_changed(val).await.alert_err();
            }
        });

        self.ui_represent
            .place()
            .l(20)
            .anchor(Top, self.scan, 20)
            .anchor(Right, self.play_sound, 20)
            .b(20);

        log_spawn(self.initial_scan());
    }
}

impl MainScreen {
    async fn initial_scan(mut self: Weak<Self>) -> Result<()> {
        let spin = Spinner::lock();

        let clients = netrun::scan_for_port(inspect::PORT_RANGE.start).await?;

        let Some((ip, _)) = clients.into_iter().next() else {
            return Ok(());
        };

        let client = Client::connect((ip, inspect::PORT_RANGE.start)).await?;

        client.send(UIRequest::GetUI).await?;

        on_main(move || {
            self.clients.set_values(vec![ip]);
            self.current_client = Some(client);

            drop(spin);

            log_spawn::<anyhow::Error>(async move {
                loop {
                    let client = self.current_client.as_ref().ok_or(anyhow!("No client"))?;
                    let command = client.receive().await?;
                    self.process_command(command).await?;
                }
            });
        });

        Ok(())
    }

    async fn scan_tapped(mut self: Weak<Self>) -> Result<()> {
        let spin = Spinner::lock();

        let clients = netrun::scan_for_port(inspect::PORT_RANGE.start).await?;

        if clients.is_empty() {
            spin.stop();
            Alert::show("No clients found");
            return Ok(());
        }

        info!("Found: {} clients", clients.len());

        let mut available_clients = vec![];

        for (client_ip, _) in clients {
            debug!("Checking: {client_ip}");
            let client = Client::connect((client_ip, inspect::PORT_RANGE.start)).await?;

            client.send(InspectorCommand::GetSystemInfo).await?;

            let AppCommand::System(SystemResponse::Info(system)) = client.receive().await? else {
                bail!("Invalid response from the client");
            };

            if available_clients.iter().find(|(id, _)| id == &system.app_id).is_none() {
                available_clients.push((system.app_id, client_ip));
            }
        }

        dbg!(&available_clients);

        let (_, ip) = available_clients[0];

        let client = Client::connect((ip, inspect::PORT_RANGE.start)).await?;

        on_main(move || {
            self.clients
                .set_values(available_clients.iter().map(|(_app, ip)| *ip).collect());
            self.current_client = Some(client);

            log_spawn::<anyhow::Error>(async move {
                loop {
                    let client = self.current_client.as_ref().ok_or(anyhow!("No client"))?;
                    let command = client.receive().await?;
                    self.process_command(command).await?;
                }
            });
        });

        Ok(())
    }

    async fn play_sound_tapped(self: Weak<Self>) -> Result<()> {
        self.current_client
            .as_ref()
            .ok_or(anyhow!("No client"))?
            .send(InspectorCommand::PlaySound)
            .await
    }

    async fn get_ui_tapped(self: Weak<Self>) -> Result<()> {
        self.current_client
            .as_ref()
            .ok_or(anyhow!("No client"))?
            .send(UIRequest::GetUI)
            .await
    }

    async fn scale_changed(self: Weak<Self>, scale: f32) -> Result<()> {
        self.current_client
            .as_ref()
            .ok_or(anyhow!("No client"))?
            .send(UIRequest::SetScale(scale))
            .await
    }

    async fn process_command(self: Weak<Self>, command: AppCommand) -> Result<()> {
        match command {
            AppCommand::UI(ui) => {
                self.process_ui_command(ui).await?;
            }
            AppCommand::System(info) => {
                dbg!(&info);
            }
            AppCommand::Ok => {
                info!("Ok received");
            }
        };

        Ok(())
    }

    async fn process_ui_command(self: Weak<Self>, command: UIResponse) -> Result<()> {
        match command {
            UIResponse::Scale(scale) => {
                on_main(move || {
                    self.ui_scale_value.set_value(scale);
                });
            }
            UIResponse::SendUI { scale, root } => on_main(move || {
                self.ui_represent.set_root(scale, root);
            }),
        };

        Ok(())
    }
}
