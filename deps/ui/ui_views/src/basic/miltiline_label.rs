#![allow(unused_imports)]
#![allow(dead_code)]

use std::ops::Index;

use gm::Color;
use refs::{set_current_thread_as_main, Weak};
use rtools::data_manager::Handle;
use smart_default::SmartDefault;
use text::{render_text, text_size, Font};
use ui::{view, SubView, ViewCallbacks, ViewData, ViewFrame, ViewSubviews};

use crate::ImageView;

#[view]
#[derive(SmartDefault)]
pub struct MultilineLabel {
    #[default(Font::san_francisco())]
    font:   Handle<Font>,
    text:   String,
    images: Vec<(String, Weak<ImageView>)>,
    #[default(text::DEFAULT_FONT_SIZE as f32)]
    size:   f32,
}

impl MultilineLabel {
    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn set_text(&mut self, text: impl ToString) -> &mut Self {
        self.text = text.to_string();
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

    fn set_letters(&mut self) {
        self.remove_all_subviews();

        let size = text_size(&self.text, &self.font, self.size);

        if size.width < self.width() {
            let image = render_text(&self.text, &self.font, self.size);
            let mut view = self.add_view::<ImageView>();
            view.set_image(image);
        } else {
            let text = self.text.clone();
            let (a, b) = text.split_at(self.text.len() / 2);
            let image = render_text(a, &self.font, self.size);
            let mut view = self.add_view::<ImageView>();
            view.set_image(image);
            let image = render_text(b, &self.font, self.size);
            let mut view = self.add_view::<ImageView>();
            view.set_image(image);
        }
    }

    fn split_text(&self, text: &str) -> (String, Option<String>) {
        if self.fits(text) {
            return (text.to_string(), None);
        }

        let mut index = text.len();

        loop {
            let slice = &text[..index];
            if self.fits(slice) {
                return (slice.to_string(), text[index..].to_string().into());
            }
            assert!(index > 0);
            index -= 1;
        }
    }

    fn fits(&self, text: &str) -> bool {
        text_size(text, &self.font, self.size).width <= self.width()
    }
}

impl ViewCallbacks for MultilineLabel {
    fn setup(&mut self) {
        // self.text_color = Color::GREEN;kjlfdgfsd
        // self.font = Font::san_francisco();
        //
        // self.image_view.place.center();
        //
        self.set_letters();
        self.place.all_ver();
    }

    fn update(&mut self) {
        self.set_letters();
        // if self.needs_update {
        //     self.set_letters();
        //     self.needs_update = false;
        // }
        // self.fit_size();
    }
}

#[cfg(test)]
mod test {
    use refs::set_current_thread_as_main;
    use rtools::Random;
    use serial_test::serial;
    use text::{text_size, Font, DEFAULT_FONT_SIZE};
    use ui::ViewFrame;

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

        let mut view = MultilineLabel::default();
        view.set_size((100, 100));

        assert!(view.fits("lo"));
        assert!(view.fits("lolo"));
        assert!(!view.fits("lolol"));
        assert!(!view.fits("lolololol"));
    }

    #[test]
    #[serial]
    fn split_one() {
        set_current_thread_as_main();
        Font::disable_render();

        let mut view = MultilineLabel::default();
        view.set_size((100, 100));

        assert_eq!(view.split_text("lolo"), ("lolo".to_string(), None));
    }

    #[test]
    #[serial]
    fn split_two() {
        set_current_thread_as_main();
        Font::disable_render();

        let mut view = MultilineLabel::default();
        view.set_size((200, 100));

        assert_eq!(
            view.split_text("123456789abcdefg"),
            ("12345".to_string(), Some("6789abcdefg".to_string()))
        );

        dbg!("poros");
    }

    #[test]
    #[serial]
    fn split_many() {
        set_current_thread_as_main();
        Font::disable_render();

        let mut view = MultilineLabel::default();
        view.set_size((1200, 100));

        let mut long_string = (0..u64::random_in(50..100))
            .map(|_| String::random())
            .collect::<Vec<_>>()
            .join("");

        let mut split = vec![];

        while let (line, Some(rest)) = view.split_text(&long_string) {
            split.push(line);
            long_string = rest;
        }

        assert!(split.iter().all(|line| view.fits(&line)));
    }
}
