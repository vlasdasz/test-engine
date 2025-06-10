mod client;
mod command;
mod connection;
mod message;
mod server;

pub use client::Client;
pub use command::*;
pub use message::DebugMessage;
pub use server::DebugServer;

pub const DEFAULT_PORT: u16 = 57056;

type Callback = Box<dyn FnMut(DebugMessage) + Send>;
