use cfg_if::cfg_if;

cfg_if! {if #[cfg(not(any(target_os="ios", target_os="android")))] {
    use crate::check_gl_error;
    use soil2::{SOIL_free_image_data, SOIL_load_image};
    use std::os::raw::c_int;
    use std::ffi::CString;
}}

use crate::gl_wrapper::{GLWrapper, TextureLoader};
use crate::gm::Size;
use image::GenericImageView;
use std::ffi::c_void;
use std::path::PathBuf;

#[derive(Debug, Copy, Clone)]
pub struct Image {
    pub size: Size,
    pub channels: u32,
    gl_handle: u32,
}

impl Image {
    pub fn new() -> Image {
        Image {
            size: Size::new(),
            channels: 0,
            gl_handle: u32::MAX,
        }
    }

    pub fn invalid(&self) -> bool {
        self.gl_handle == u32::MAX
    }

    pub fn load(path: &PathBuf) -> Image {
        cfg_if::cfg_if! {
            if #[cfg(any(target_os="ios", target_os="android"))] {
                Image { size: Size::new(), channels: 0, gl_handle: 0 }
            }
            else {
                Image::load_with_image(path)
            }
        }
    }

    pub fn load_with_image(path: &PathBuf) -> Image {
        let image = image::open(path).expect(&format!("Failed to open image {:?}", path));

        let dimensions = image.dimensions();

        let data = image.as_bytes();

        let channels = image.color().channel_count();

        let size = Size {
            width: dimensions.0 as f32,
            height: dimensions.1 as f32,
        };

        Image::from(data.as_ptr() as *const c_void, size, channels as u32)
    }

    #[cfg(not(any(target_os = "ios", target_os = "android")))]
    pub fn load_with_soil(path: &PathBuf) -> Image {
        unsafe {
            let mut width: c_int = -1;
            let mut height: c_int = -1;
            let mut channels: c_int = -1;

            let c_path = CString::new(path.to_str().unwrap()).expect("CString::new failed");

            cfg_if::cfg_if! {
                if #[cfg(all(target_os = "linux", any(target_arch = "arm", target_arch = "aarch64")))] {
                    type CPath = *const u8;
                }
                else {
                    type CPath = *const i8;
                }
            };

            let data = SOIL_load_image(
                c_path.as_ptr() as CPath,
                &mut width,
                &mut height,
                &mut channels,
                4, //SOIL_LOAD_RGBA
            );

            check_gl_error!();

            if data.is_null() || width == -1 || height == -1 {
                panic!("Failed to load image: {:?}", path);
            }

            let image = Image::from(
                data as *const c_void,
                Size {
                    width: width as f32,
                    height: height as f32,
                },
                channels as u32,
            );

            SOIL_free_image_data(data);

            check_gl_error!();

            image
        }
    }

    pub fn from(data: *const c_void, size: Size, channels: u32) -> Image {
        let gl_handle = TextureLoader::load(data, size, channels);
        Image {
            size,
            channels,
            gl_handle,
        }
    }

    pub fn is_monochrome(&self) -> bool {
        self.channels == 1
    }

    pub fn bind(&self) {
        GLWrapper::bind_image(self.gl_handle)
    }
}
