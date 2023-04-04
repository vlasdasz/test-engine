#![allow(unused_imports)]
#![allow(dead_code)]

use std::{collections::HashMap, ops::Index};

use gm::{flat::Size, Color};
use refs::{set_current_thread_as_main, ToWeak, Weak};
use rtools::{data_manager::Handle, hash, IntoF32};
use smart_default::SmartDefault;
use text::{render_text, text_size, Font};
use ui::{view, SubView, ViewCallbacks, ViewData, ViewFrame, ViewLayout, ViewSetup, ViewSubviews};

use crate::ImageView;

#[view]
pub struct MultilineLabel {
    font:          Handle<Font>,
    text:          String,
    size:          f32,
    split_storage: HashMap<u64, (Vec<String>, f32)>,
}

impl MultilineLabel {
    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn set_text(&mut self, text: impl ToString) -> &mut Self {
        self.text = text.to_string();
        self
    }

    pub fn set_text_size(&mut self, size: impl IntoF32) -> &mut Self {
        self.size = size.into_f32() as _;
        self
    }

    pub fn append_text(&mut self, text: impl ToString) -> &mut Self {
        self.set_text(format!("{}{}", self.text, text.to_string()));
        self
    }

    pub fn pop_letter(&mut self) {
        if !self.text.is_empty() {
            self.text.pop();
        }
    }

    pub fn clear(&mut self) -> &Self {
        self.set_text("")
    }

    fn calculate_split(&mut self) -> &(Vec<String>, f32) {
        let size = self.size();

        let hash = hash(size);

        if let Some(split) = self.split_storage.get(&hash) {
            return split;
        }

        let mut text_size = self.size;

        let mut split = self.split_text(&self.text, text_size);

        while !self.fits_height(&split, text_size) {
            text_size -= 1.0;
            split = self.split_text(&self.text, text_size);
        }

        let mut this = self.weak();
        this.split_storage.insert(hash, (split, text_size));

        self.split_storage.get(&hash).unwrap()
    }

    fn set_letters(&mut self) {
        self.remove_all_subviews();

        // Sorry borrow checker
        let mut this = self.weak();
        let split = this.calculate_split();

        for line in &split.0 {
            let mut image_view = self.add_view::<ImageView>();
            let image = render_text(line, &self.font, split.1);
            image_view.set_size(image.size);
            image_view.set_image(image);
        }
    }

    fn get_rest(&self, text: &str, size: impl IntoF32) -> (String, Option<String>) {
        if self.fits_width(text, size) {
            return (text.to_string(), None);
        }

        let mut index = text.len();

        loop {
            let slice = &text[..index];

            if slice.is_empty() {
                return (text[0..1].to_string(), text[1..].to_string().into());
            }

            if self.fits_width(slice, size) {
                return (slice.to_string(), text[index..].to_string().into());
            }
            assert!(index > 0);
            index -= 1;
        }
    }

    fn split_text(&self, text: &str, size: impl IntoF32) -> Vec<String> {
        let mut str = text.to_string();

        let mut split = vec![];

        loop {
            match self.get_rest(&str, size) {
                (line, Some(rest)) => {
                    split.push(line);
                    str = rest;
                }
                (line, None) => {
                    split.push(line);
                    return split;
                }
            }
        }
    }

    fn fits_width(&self, text: &str, size: impl IntoF32) -> bool {
        text_size(text, &self.font, size).width <= self.width()
    }

    fn fits_height(&self, text: &[String], size: impl IntoF32) -> bool {
        let total_height: f32 = text.iter().map(|a| text_size(a, &self.font, size).height).sum();

        total_height <= self.height()
    }

    fn layout(&mut self) {
        let height = self.height() / self.subviews().len() as f32;

        for (i, view) in self.subviews_mut().iter_mut().enumerate() {
            view.set_y(height * i as f32);
            view.calculate_absolute_frame();
        }
    }
}

impl ViewSetup for MultilineLabel {
    fn setup(mut self: Weak<Self>) {
        self.font = Font::san_francisco();
        self.size = text::DEFAULT_FONT_SIZE;

        self.set_letters();
    }
}

impl ViewCallbacks for MultilineLabel {
    fn update(&mut self) {
        self.set_letters();
        self.layout();

        // dbg!(self.size());
        // dbg!(self.subviews().first().map(|a| a.frame().size));
        // dbg!(self.subviews().first().map(|a| a.image().size));
    }
}

#[cfg(test)]
mod test {
    use std::ops::Deref;

    use refs::{set_current_thread_as_main, Own};
    use rtools::Random;
    use serial_test::serial;
    use text::{text_size, Font, DEFAULT_FONT_SIZE};
    use ui::{ViewFrame, ViewInternalSetup};

    use crate::MultilineLabel;

    #[test]
    #[serial]
    fn size() {
        set_current_thread_as_main();
        Font::disable_render();

        assert_eq!(
            text_size("sos", &Font::san_francisco(), DEFAULT_FONT_SIZE),
            (94, 62).into()
        );
        assert_eq!(
            text_size("kok", &Font::san_francisco(), DEFAULT_FONT_SIZE),
            (96, 61).into()
        );
        assert_eq!(
            text_size("lol", &Font::san_francisco(), DEFAULT_FONT_SIZE),
            (60, 61).into()
        );
        assert_eq!(text_size("lol", &Font::san_francisco(), 100), (95, 96).into());
    }

    #[test]
    #[serial]
    fn fits() {
        set_current_thread_as_main();
        Font::disable_render();

        let mut view = Own::<MultilineLabel>::default();
        view.internal_setup();
        view.set_size((100, 100));

        assert!(view.fits_width("lo", view.size));
        assert!(view.fits_width("lolo", view.size));
        assert!(!view.fits_width("lolol", view.size));
        assert!(!view.fits_width("lolololol", view.size));
    }

    #[test]
    #[serial]
    fn split_one() {
        set_current_thread_as_main();
        Font::disable_render();

        let mut view = Own::<MultilineLabel>::default();
        view.internal_setup();
        view.set_size((100, 100));

        assert_eq!(view.split_text("lolo", view.size), vec!["lolo".to_string()]);
    }

    #[test]
    #[serial]
    fn rest() {
        set_current_thread_as_main();
        Font::disable_render();

        let mut view = Own::<MultilineLabel>::default();
        view.internal_setup();
        view.set_size((200, 100));

        assert_eq!(
            view.get_rest("123456789abcdefg", view.size),
            ("12345".to_string(), Some("6789abcdefg".to_string()))
        );
    }

    #[test]
    #[serial]
    fn split_many() {
        set_current_thread_as_main();
        Font::disable_render();

        let mut view = Own::<MultilineLabel>::default();
        view.internal_setup();
        view.set_size((1200, 100));

        let long_string = (0..u64::random_in(50..100))
            .map(|_| String::random())
            .collect::<Vec<_>>()
            .join("");

        assert!(view
            .split_text(&long_string, view.size)
            .iter()
            .all(|line| view.fits_width(&line, view.size)));
    }

    #[test]
    #[serial]
    /// Check if space between leters is variable
    /// It is
    fn letter_margin() {
        set_current_thread_as_main();
        Font::disable_render();

        let mut view = Own::<MultilineLabel>::default();
        view.internal_setup();
        view.set_size((100, 100));

        let letter_a = u8::random() as char;
        let letter_b = u8::random() as char;

        let text_a_b = format!("{letter_a} {letter_b}");
        let text_b_a = format!("{letter_b} {letter_a}");

        let a = text_size(letter_a, Font::san_francisco().deref(), DEFAULT_FONT_SIZE).width;
        let b = text_size(letter_b, Font::san_francisco().deref(), DEFAULT_FONT_SIZE).width;

        let a_b = text_size(&text_a_b, Font::san_francisco().deref(), DEFAULT_FONT_SIZE).width;
        let b_a = text_size(&text_b_a, Font::san_francisco().deref(), DEFAULT_FONT_SIZE).width;

        let space = a_b - a - b;
        let space2 = b_a - a - b;

        dbg!(&a);
        dbg!(&b);
        dbg!(&a_b);
        dbg!(&b_a);
        dbg!(&space);
        dbg!(&space2);
        dbg!(&letter_a);
        dbg!(&letter_b);
        dbg!(&text_a_b);
        dbg!(&text_b_a);
    }
}
