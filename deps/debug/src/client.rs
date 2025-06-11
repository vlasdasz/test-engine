use std::ops::Deref;

use anyhow::Result;
use tokio::net::{TcpStream, ToSocketAddrs};

use crate::connection::Connection;

pub struct Client {
    connection: Connection,
}

impl Client {
    pub async fn new(address: impl ToSocketAddrs) -> Result<Self> {
        Ok(Self {
            connection: Connection::new(TcpStream::connect(address).await?),
        })
    }
}

impl Deref for Client {
    type Target = Connection;

    fn deref(&self) -> &Self::Target {
        &self.connection
    }
}
