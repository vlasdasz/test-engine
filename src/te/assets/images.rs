
use crate::te::paths;
use crate::image::Image;


pub struct Images {

    pub ak:     Image,
    pub cat:    Image,
    pub palm:   Image,
    pub round:  Image,
    pub square: Image,

    pub file:   Image,
    pub text:   Image,
    pub cmake:  Image,
    pub image:  Image,
    pub folder: Image,

    pub up:    Image,
    pub down:  Image,
    pub left:  Image,
    pub right: Image,

    pub frisk: Image,

    pub full_hd:    Image,
    pub scale_test: Image,
}

impl Images {
    pub fn init() -> Images {

        let ak         = Image::load(&paths::images().join("ak.png"));
        let cat        = Image::load(&paths::images().join("cat.jpg"));
        let palm       = Image::load(&paths::images().join("palm.png"));
        let round      = Image::load(&paths::images().join("round.png"));
        let square     = Image::load(&paths::images().join("square.png"));
        let file       = Image::load(&paths::images().join("file.png"));
        let text       = Image::load(&paths::images().join("text.png"));
        let cmake      = Image::load(&paths::images().join("cmake.png"));
        let image      = Image::load(&paths::images().join("image.png"));
        let folder     = Image::load(&paths::images().join("folder.png"));
        let up         = Image::load(&paths::images().join("up.png"));
        let down       = Image::load(&paths::images().join("down.png"));
        let left       = Image::load(&paths::images().join("left.png"));
        let right      = Image::load(&paths::images().join("right.png"));
        let frisk      = Image::load(&paths::images().join("frisk.png"));
        let full_hd    = Image::load(&paths::images().join("full_hd.jpg"));
        let scale_test = Image::load(&paths::images().join("scale_test.png"));

        Images {
            ak,
            cat,
            palm,
            round,
            square,
            file,
            text,
            cmake,
            image,
            folder,
            up,
            down,
            left,
            right,
            frisk,
            full_hd,
            scale_test
        }
    }
}