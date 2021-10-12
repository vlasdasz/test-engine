use gl_image::Image;
use gm::{Color, Point, Size};
use serde::{ser::SerializeStruct, Serialize, Serializer};
use tools::math::IntoF32;

pub trait Sprite {
    fn size(&self) -> Size {
        self.sprite().size
    }

    fn position(&self) -> Point {
        self.sprite().position
    }

    fn rotation(&self) -> f32 {
        self.sprite().rotation
    }

    fn color(&self) -> Color {
        self.sprite().color
    }

    fn image(&self) -> &Option<Image> {
        &self.sprite().image
    }

    fn set_image(&mut self, image: Image) {
        self.sprite_mut().image = image.into()
    }

    fn sprite(&self) -> &SpriteBase;
    fn sprite_mut(&mut self) -> &mut SpriteBase;
}

pub struct SpriteBase {
    position:  Point,
    size:      Size,
    rotation:  f32,
    pub color: Color,
    pub image: Option<Image>,
}

impl Serialize for dyn Sprite {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("Sprite", 5)?;
        let sprite = self.sprite();

        s.serialize_field("position", &sprite.position)?;
        s.serialize_field("size", &sprite.size)?;
        s.serialize_field("rotation", &sprite.rotation)?;
        s.serialize_field("color", &sprite.color)?;
        s.serialize_field("image", &sprite.image)?;

        s.end()
    }
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
            size:     (data.2, data.3).into(),
            rotation: 0.0,
            color:    Color::random(),
            image:    None,
        }
    }
}
