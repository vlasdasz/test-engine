use std::{
    future::{Future, IntoFuture},
    pin::Pin,
};

use gm::{
    color::{BLACK, BLUE, GRAY},
    flat::Size,
};
use refs::Weak;
use ui_proc::view;
use vents::OnceEvent;

use crate::{ModalView, Setup, has_data::HasText, view::ViewData};
mod test_engine {
    pub(crate) use educe;
    pub(crate) use refs;

    pub(crate) use crate as ui;
}

use crate::{Anchor::Width, Button, Label, ViewSubviews};

#[view]
pub struct Question {
    question: String,

    left:  String,
    right: String,

    event: OnceEvent<bool>,

    #[init]
    label:         Label,
    ok_button:     Button,
    cancel_button: Button,
}

impl ModalView<(), bool> for Question {
    fn modal_event(&self) -> &OnceEvent<bool> {
        &self.event
    }

    fn modal_size() -> Size {
        (380, 240).into()
    }
}

impl Question {
    pub fn ask(question: impl Into<String>) -> Self {
        Question {
            question: question.into(),
            left: "No".to_string(),
            right: "Yes".to_string(),
            ..Default::default()
        }
    }

    pub fn options(mut self, left: impl Into<String>, right: impl Into<String>) -> Self {
        self.left = left.into();
        self.right = right.into();
        self
    }

    ///bool == true -> right choice
    pub fn callback(self, callback: impl FnOnce(bool) + Send + 'static) {
        Self::make_modal(self).event.val(callback);
    }

    pub fn on_yes(self, callback: impl FnOnce() + Send + 'static) {
        self.callback(|yes| {
            if yes {
                callback();
            }
        });
    }

    async fn callback_async(self) -> bool {
        // let (se, rc) = channel::<bool>();
        //
        // from_main(move || {
        //     Self::make_modal(self).event.val(|answer| {
        //         se.send(answer).unwrap();
        //     });
        // })
        // .await;
        //
        // rc.await.unwrap()

        false
    }
}

impl IntoFuture for Question {
    type Output = bool;
    type IntoFuture = Pin<Box<dyn Future<Output = Self::Output> + Send>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move { self.callback_async().await })
    }
}

impl Setup for Question {
    fn setup(mut self: Weak<Self>) {
        self.set_corner_radius(10).set_border_color(BLACK);

        let question = self.question.clone();
        let left = self.left.clone();
        let right = self.right.clone();

        self.label.place().lrt(10).h(140);
        self.label.set_text_size(35);
        self.label.set_text(question);
        self.label.multiline = true;

        self.ok_button.place().h(50).br(2).relative(Width, self, 0.5);

        self.ok_button.set_text(right).set_border_color(GRAY).set_text_color(BLUE);

        self.ok_button.on_tap(move || self.hide_modal(true));

        self.cancel_button.place().h(50).bl(2).relative(Width, self, 0.5);
        self.cancel_button.set_text(left).set_border_color(GRAY).set_text_color(BLUE);

        self.cancel_button.on_tap(move || self.hide_modal(false));

        self.outline(BLACK);
    }
}
