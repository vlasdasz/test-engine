// use manage::data_manager::DataManager;
// use old_engine::{gm::flat::Shape, rtools::Animation};
// use ui::refs::{weak_from_ref, Weak};
//
// #[derive(Default)]
// pub struct BenchmarkLevel {
//     base:       LevelBase,
//     left_wall:  Weak<Wall>,
//     right_wall: Weak<Wall>,
//     floor:      Weak<Wall>,
//
//     left_animation:  Animation,
//     right_animation: Animation,
//     floor_animation: Animation,
//
//     pub player:        Weak<Player>,
//     pub bullets_count: u64,
// }
//
// impl BenchmarkLevel {
//     fn make_walls(&mut self) {
//         let square = GlImage::get("square.png");
//
//         self.floor = self.add_sprite(Shape::Rect((100, 10).into()), (0, 0));
//         self.floor.set_image(square);
//
//         self.left_wall = self.add_sprite(Shape::Rect((10, 200).into()), (-40,
// 0));         self.left_wall.set_image(square);
//
//         self.right_wall = self.add_sprite(Shape::Rect((10, 200).into()), (40,
// 0));         self.right_wall.set_image(square);
//
//         self.left_animation = Animation::new(-60, -55, 10);
//         self.right_animation = Animation::new(60, 55, 10);
//         self.floor_animation = Animation::new(-10, 0, 4);
//     }
// }
//
// impl Level for BenchmarkLevel {
//     fn setup(&mut self) {
//         self.player = self.add_sprite(Shape::Rect((2, 2).into()), (0, 5));
//
//         self.player.set_image("frisk.png");
//
//         self.player.weapon.set_image("ak.png");
//         self.player.weapon.bullet_image = GlImage::get("bullet.png");
//         self.player.weapon.bullet_speed = 100.0;
//         self.player.weapon.bullet_shape = Shape::Rect((1, 0.28).into());
//
//         self.set_scale(1.0);
//         self.make_walls();
//     }
//
//     fn update(&mut self) {
//         self.player.weapon.weak().shoot_at((10, 15));
//         self.bullets_count += 1;
//         self.left_wall.set_x(self.left_animation.value());
//         self.right_wall.set_x(self.right_animation.value());
//     }
//
//     fn base(&self) -> &LevelBase {
//         &self.base
//     }
//
//     fn base_mut(&mut self) -> &mut LevelBase {
//         &mut self.base
//     }
//
//     fn weak_level(&self) -> Weak<dyn Level> {
//         weak_from_ref(self as &dyn Level)
//     }
// }
