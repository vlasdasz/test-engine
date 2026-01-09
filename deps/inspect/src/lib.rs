mod app_command;
mod inspector_command;

pub use app_command::*;
pub use inspector_command::*;

pub const PORT_RANGE: std::ops::Range<u16> = 55400..55410;
