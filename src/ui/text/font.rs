use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::ptr::null_mut;
use freetype_sys::{FT_Face, FT_Library, FT_Get_Char_Index, FT_Load_Glyph, FT_LOAD_RENDER, FT_Init_FreeType, FT_New_Memory_Face, FT_Set_Pixel_Sizes, FT_Done_FreeType, FT_Glyph, FT_Get_Glyph, FT_BitmapGlyph, FT_Done_Glyph};
use crate::gm::{Size, Point};
use crate::image::Image;
use std::ffi::c_void;
use crate::ui::Glyph;
use std::ops::Range;

cfg_if::cfg_if! {
    if #[cfg(target_os = "windows")] {
        pub type FtSymbol = u32;
        pub type FtLen = i32;
    } else {
        pub type FtSymbol = i64;
        pub type FtLen = u32;
    }
}

unsafe fn render_glyph(face: FT_Face, symbol: char, ft_lib: FT_Library) -> Glyph {

    let index = FT_Get_Char_Index(face, symbol as FtSymbol);

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

pub struct Font {

    pub size: u32,
    pub height: f32,
    pub baseline_shift: f32,

    glyphs: Vec<Glyph>
}

impl Font {
    pub fn new(path: &PathBuf, size: u32) -> Option<Font> {

        guard!(let Ok(file) = File::open(path) else {
            log!(format!("Font {:?} not found.", path));
            return None
        });

        let mut data: Vec<u8> = vec![];

        for byte in file.bytes() {
            data.push(byte.unwrap())
        }

        unsafe {

            let mut ft_lib: FT_Library = null_mut();

            FT_Init_FreeType(&mut ft_lib);

            assert_null!(ft_lib);

            let mut face: FT_Face = null_mut();

            let len = data.len();

            FT_New_Memory_Face(ft_lib,
                               data.as_ptr(),
                               len as FtLen,
                               0,
                               &mut face);

            assert_null!(face);

            FT_Set_Pixel_Sizes(face, 0, size);

            let mut glyphs = Vec::<Glyph>::with_capacity(128);

            let mut y_max: f32 = f32::MIN;
            let mut y_min: f32 = f32::MAX;

            for symbol in (Range { start: 0 as char, end: 127 as char }) {
                let glyph = render_glyph(face, symbol, ft_lib);

                if y_max < glyph.y_max() {
                    y_max = glyph.y_max()
                }

                if y_min > glyph.y_min() {
                    y_min = glyph.y_min()
                }

                log!(&glyph.image);

                glyphs.push(glyph);
            }

            let height = y_max - y_min;
            let baseline_position = y_min.abs();
            let baseline_shift = height / 2.0 - baseline_position;

            FT_Done_FreeType(ft_lib);

            Some(Font {
                size,
                height,
                baseline_shift,
                glyphs
            })
        }
    }
}

impl Font {

    pub fn glyph_for_char(&self, ch: char) -> &Glyph {
        &self.glyphs[ch as usize]
    }

}