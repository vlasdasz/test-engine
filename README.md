# TestEngine

[![](http://github-actions.40ants.com/vlasdasz/test-engine/matrix.svg)](https://github.com/vlasdasz/test-engine)

---

My attempt to create a cross platform, OpenGL based game engine using Rust.

Previously written in C++: https://github.com/VladasZ/test_engine_cpp

Inspired by Cross++: https://github.com/maxon887/Cross

---

Simplest example:

```rust
// main.rs

#![allow(incomplete_features)]
#![feature(specialization)]
#![feature(arbitrary_self_types)]

use test_engine::{
    App,
    refs::{Own, Weak},
    ui::{HasText, Label, Setup, U8Color, UIManager, View, ViewData, view},
};

#[view]
struct MainScreen {
    #[init]
    hello_label: Label,
}

impl Setup for MainScreen {
    fn setup(self: Weak<Self>) {
        UIManager::set_clear_color("#4E4D5C");

        self.hello_label
            .set_text("Hello Test Engine!")
            .set_color(U8Color::rgba(156, 149, 220, 255))
            .set_corner_radius(10)
            .set_border_color("#228CDB")
            .set_border_width(5)
            .set_text_size(40);

        self.hello_label.place().center().size(400, 80);
    }
}

#[derive(Default)]
struct ExampleApp;

impl App for ExampleApp {
    fn make_root_view(&self) -> Own<dyn View> {
        MainScreen::new()
    }
}

fn main() {
    ExampleApp::start();
}

```




Result:

<img width="550" height="305" alt="image" src="https://github.com/user-attachments/assets/ac661f61-8984-4d02-a273-5e24897f8691" />
