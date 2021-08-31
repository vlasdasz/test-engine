use gl_image::Image;
use gm::{Color, IntoF32, Point, Size};
use proc_macro::New;

pub trait Sprite {
    fn sprite(&self) -> &SpriteBase;
    fn sprite_mut(&mut self) -> &mut SpriteBase;

    fn size(&self) -> &Size {
        &self.sprite().size
    }

    fn position(&self) -> &Point {
        &self.sprite().position
    }

    fn rotation(&self) -> f32 {
        self.sprite().rotation
    }

    fn color(&self) -> &Color {
        &self.sprite().color
    }

    fn image(&self) -> &Option<Image> {
        &self.sprite().image
    }

    fn set_image(&mut self, image: Image) {
        self.sprite_mut().image = image.into()
    }
}

#[derive(New)]
pub struct SpriteBase {
    pub position: Point,
    pub size: Size,
    pub rotation: f32,
    pub color: Color,
    pub image: Option<Image>,
}

impl Sprite for SpriteBase {
    fn sprite(&self) -> &SpriteBase {
        self
    }

    fn sprite_mut(&mut self) -> &mut SpriteBase {
        self
    }
}

impl<X: IntoF32, Y: IntoF32, W: IntoF32, H: IntoF32> From<(X, Y, W, H)> for SpriteBase {
    fn from(data: (X, Y, W, H)) -> Self {
        Self {
            position: (data.0, data.1).into(),
            size: (data.2, data.3).into(),
            rotation: 0.0,
            color: Color::random(),
            image: None,
        }
    }
}
