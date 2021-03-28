use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::ffi::c_void;
use freetype::freetype::{FT_New_Memory_Face, FT_Init_FreeType, FT_Library, FT_Done_FreeType, FT_Face, FT_Set_Pixel_Sizes};
use std::ptr::{null, null_mut};

pub struct Font {

}

impl Font {
    pub fn new(path: &PathBuf, size: u32) -> Option<Font> {

        guard!(let Ok(file) = File::open(path) else {
            log!(format!("Font {:?} not found.", path));
            return None
        });

        let mut data: Vec<u8> = vec![];

        for byte in file.bytes() {
            data.push(byte.unwrap());
        }

        unsafe {

            let mut ft_lib: FT_Library = null_mut();

            FT_Init_FreeType(&mut ft_lib);

            assert_null!(ft_lib);

            let mut face: FT_Face = null_mut();

            let len = data.len();

            FT_New_Memory_Face(ft_lib,
                               data.as_ptr(),
                               len as i64,
                               0,
                               &mut face);

            assert_null!(face);

            FT_Set_Pixel_Sizes(face, 0, size);


            FT_Done_FreeType(ft_lib);
        }

        Some(Font {})
    }
}