use crate::image::Image;
use crate::te::paths;

pub struct Images {
    // pub ak: Image,
    pub cat: Image,
    // pub palm: Image,
    // pub round: Image,
    // pub square: Image,
    //
    // pub file: Image,
    // pub text: Image,
    // pub cmake: Image,
    // pub image: Image,
    // pub folder: Image,
    //
    // pub up: Image,
    // pub down: Image,
    // pub left: Image,
    // pub right: Image,
    //
    pub frisk: Image,
    //
    pub full_hd: Image,
    // pub scale_test: Image,
}

impl Images {
    pub fn init() -> Images {
        Images {
            // ak: Image::load(&paths::images().join("ak.png")),
            cat: Image::load(&paths::images().join("cat.jpg")),
            // palm: Image::load(&paths::images().join("palm.png")),
            // round: Image::load(&paths::images().join("round.png")),
            // square: Image::load(&paths::images().join("square.png")),
            // file: Image::load(&paths::images().join("file.png")),
            // text: Image::load(&paths::images().join("text.png")),
            // cmake: Image::load(&paths::images().join("cmake.png")),
            // image: Image::load(&paths::images().join("image.png")),
            // folder: Image::load(&paths::images().join("folder.png")),
            // up: Image::load(&paths::images().join("up.png")),
            // down: Image::load(&paths::images().join("down.png")),
            // left: Image::load(&paths::images().join("left.png")),
            // right: Image::load(&paths::images().join("right.png")),
            frisk: Image::load(&paths::images().join("frisk.png")),
            full_hd: Image::load(&paths::images().join("full_hd.jpg")),
            // scale_test: Image::load(&paths::images().join("scale_test.png")),
        }
    }
}
