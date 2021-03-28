
pub struct Platform;

impl Platform {
    pub const MAC: bool = cfg!(target_os = "macos");
    pub const WIN: bool = cfg!(target_os = "windows");
}