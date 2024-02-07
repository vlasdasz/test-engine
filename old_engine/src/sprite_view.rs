// use ui::{refs::Weak, view, SubView, ViewCallbacks, ViewData, ViewSetup};
// use ui_views::{Button, LabeledView};
//
// #[view]
// pub struct SpriteView {
//     position: SubView<LabeledView>,
//     size:     SubView<LabeledView>,
//     color:    SubView<LabeledView>,
//
//     delete_button: SubView<Button>,
//     //sprite: Weak<dyn Sprite>,
// }
//
// impl SpriteView {
//     pub fn set_sprite(&mut self, _sprite: Weak<dyn Sprite>) {
//         // self.sprite = sprite;
//         // self.delete_button.set_hidden(sprite.is_null());
//         // if sprite.is_null() {
//         //     self.position.clear();
//         //     self.size.clear();
//         //     self.color.clear();
//         //     return;
//         // }
//         // self.position.set_text(sprite.position());
//         // self.size.set_text(sprite.size());
//         // self.color.set_text(*sprite.color());
//     }
//
//     fn setup_delete_button(self: Weak<Self>) {
//         _ = self;
//         // self.delete_button.place.size(20, 20).tl(0);
//         // self.delete_button.set_hidden(true).set_image(Images::delete());
//     }
// }
//
// impl ViewSetup for SpriteView {
//     fn setup(mut self: Weak<Self>) {
//         self.place().all_ver();
//
//         self.position.set_title("position:");
//         self.size.set_title("size:");
//         self.color.set_title("color:");
//
//         self.setup_delete_button();
//     }
// }
//
// impl ViewCallbacks for SpriteView {
//     fn update(&mut self) {
//         // if self.sprite.is_null() {
//         //     return;
//         // }
//         // self.set_sprite(self.sprite);
//     }
// }
