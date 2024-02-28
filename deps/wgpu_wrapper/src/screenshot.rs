use gm::{
    flat::{Point, Size},
    Color, LossyConvert, U8Color,
};

#[derive(Debug, Default)]
pub struct Screenshot {
    pub data: Vec<U8Color>,
    pub size: Size<u32>,
}

impl Screenshot {
    pub fn new(data: Vec<U8Color>, size: Size<u32>) -> Self {
        Self { data, size }
    }

    pub fn get_pixel(&self, pos: impl Into<Point>) -> U8Color {
        if self.data.is_empty() {
            return Default::default();
        }

        let pos: Point<usize> = pos.into().lossy_convert();

        let Some(color) = self.data.get(pos.x + pos.y * self.size.width as usize) else {
            return Default::default();
        };

        let color: Color<f32> = (*color).into();

        color.from_srgb().into()
    }
}
