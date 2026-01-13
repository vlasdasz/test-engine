use std::net::Ipv4Addr;

use anyhow::Result;
use inspect::{AppCommand, InspectorCommand, PORT_RANGE};
use tokio::sync::OnceCell;

pub type Client = netrun::Client<AppCommand, InspectorCommand>;

static CLIENT: OnceCell<Client> = OnceCell::const_new();

pub(crate) async fn client() -> Result<&'static Client> {
    CLIENT
        .get_or_try_init(|| async { Ok(Client::connect((Ipv4Addr::LOCALHOST, PORT_RANGE.start)).await?) })
        .await
}
