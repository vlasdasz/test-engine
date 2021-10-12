use gm::Size;
use tools::math::mm_to_inch;

#[derive(Debug)]
pub struct Monitor {
    pub name: String,

    pub ppi:          u32,
    pub scale:        f32,
    pub refresh_rate: u32,

    pub resolution:    Size,
    pub physical_size: Size,
}

#[cfg(not(any(target_os = "ios", target_os = "android")))]
impl From<&glfw::Monitor> for Monitor {
    fn from(monitor: &glfw::Monitor) -> Self {
        let name = monitor.get_name().unwrap();

        let mode = monitor.get_video_mode().unwrap();

        let resolution: Size = (mode.width, mode.height).into();
        let refresh_rate = mode.refresh_rate;
        let scale = monitor.get_content_scale().0;

        let size = monitor.get_physical_size();
        let physical_size: Size = (size.0, size.1).into();

        let ppi = (resolution.height / mm_to_inch(physical_size.height)) as u32;

        Self {
            name,
            ppi,
            scale,
            refresh_rate,
            resolution,
            physical_size,
        }
    }
}
