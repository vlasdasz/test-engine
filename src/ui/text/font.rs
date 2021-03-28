use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::ptr::null_mut;
use freetype_sys::{FT_Face, FT_Library, FT_Get_Char_Index, FT_Load_Glyph, FT_LOAD_RENDER, FT_Init_FreeType, FT_New_Memory_Face, FT_Set_Pixel_Sizes, FT_Done_FreeType, FT_Glyph, FT_Get_Glyph, FT_BitmapGlyph, FT_Done_Glyph};
use crate::gm::{Size, Point};
use crate::image::Image;
use std::ffi::c_void;
use crate::ui::Glyph;

unsafe fn render_glyph(face: FT_Face, symbol: char, ft_lib: FT_Library) -> Glyph {
    let index = FT_Get_Char_Index(face, symbol as u64);

    FT_Load_Glyph(face, index, FT_LOAD_RENDER as i32);

    let mut ft_glyph: FT_Glyph = null_mut();
    FT_Get_Glyph((*face).glyph, &mut ft_glyph);

    let bitmap_glyph = ft_glyph as FT_BitmapGlyph;

    let size = Size {
        width:  (*bitmap_glyph).bitmap.width as f32,
        height: (*bitmap_glyph).bitmap.rows as f32
    };

    let metrics = (*(*face).glyph).metrics;

    let bearing = Point {
        x:  metrics.horiBearingX as f32 / 64.0,
        y: metrics.horiBearingY as f32 / 64.0,
    };

    let image = Image::from((*bitmap_glyph).bitmap.buffer as *const c_void, size, 1);

    let glyph = Glyph::new(symbol, image, (metrics.horiAdvance as f32 / 64.0) as u32, bearing);

    FT_Done_Glyph(ft_glyph);

    glyph
}

// auto glyph = new Glyph(symbol,
// new Image(bitmap_glyph->bitmap.buffer, size.width, size.height, 1),
// static_cast<int>(face->glyph->metrics.horiAdvance / 64),
// bearing);
//
// FT_Done_Glyph(ft_glyph);
//
// return glyph;
// }

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