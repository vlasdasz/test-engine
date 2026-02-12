use std::any::Any;

use anyhow::Result;
use test_engine::{
    audio::Sound,
    dispatch::{after, spawn},
    filesystem::Paths,
    gm::Toggle,
    level::LevelManager,
    refs::{Own, Weak, manage::DataManager},
    ui::{
        ALL_VIEWS, Alert, Point, Setup, Spinner, TableData, TableView, UIManager, View, ViewData, ViewFrame,
        ViewTest, all_view_tests, all_views, cast_cell, view_test,
    },
};

use crate::{
    api::TEST_REST_REQUEST,
    interface::{
        game_view::GameView,
        noise_view::NoiseView,
        polygon_view::PolygonView,
        render_view::RenderView,
        root_layout_view::RootLayoutView,
        test_game_view::{BenchmarkView, Node, NodeCell, ScaleCell},
    },
    levels::{BenchmarkLevel, TestLevel},
    no_physics::NoPhysicsView,
};

#[view_test]
pub struct MenuView {
    root: Node,

    #[init]
    table: TableView,
}

impl Setup for MenuView {
    fn setup(mut self: Weak<Self>) {
        // UIManager::override_scale(2.0);

        self.table.set_data_source(self).place().back();

        self.root = Node::new(
            "Root",
            vec![
                Node::new(
                    "Scenes",
                    vec![
                        Node::empty("main"),
                        Node::empty("ui bench"),
                        Node::empty("polygon"),
                        Node::empty("noise"),
                        Node::empty("render"),
                        Node::empty("no physics"),
                        Node::empty("root view"),
                        Node::empty("empty game"),
                    ],
                ),
                Node::new(
                    "UI",
                    vec![
                        Node::empty("ui scale"),
                        Node::empty("sound"),
                        Node::empty("alert"),
                        Node::empty("spinner"),
                        Node::empty("pick folder"),
                    ],
                ),
                Node::new("Level", vec![Node::empty("benchmark")]),
                Node::new(
                    "System",
                    vec![
                        Node::empty("system info"),
                        Node::empty("add box"),
                        Node::empty("pick folder"),
                        Node::empty("request"),
                        Node::empty("all views"),
                        Node::empty("panic"),
                    ],
                ),
            ],
        )
        .open();
    }

    fn inspect(self: Weak<Self>) {
        dbg!(&self.frame());
    }
}

impl TableData for MenuView {
    fn cell_height(self: Weak<Self>, _: usize) -> f32 {
        28.0
    }

    fn number_of_cells(self: Weak<Self>) -> usize {
        self.root.length()
    }

    fn make_cell(mut self: Weak<Self>, index: usize) -> Own<dyn View> {
        let node = self.root.val_at_index(index);
        if node.value == "ui scale" {
            ScaleCell::new()
        } else {
            NodeCell::new()
        }
    }

    fn setup_cell(mut self: Weak<Self>, cell: &mut dyn Any, index: usize) {
        let node = self.root.val_at_index(index);
        if node.value == "ui scale" {
            let _cell = cast_cell!(ScaleCell);
        } else {
            let cell = cast_cell!(NodeCell);
            cell.set_node(node);
        }
    }

    fn cell_selected(mut self: Weak<Self>, index: usize) {
        let val = self.root.val_at_index(index);

        if val.is_leaf() {
            let command = val.value.clone();
            self.command(command);
            return;
        }

        val.open.toggle();
        self.root.update_indices(0, 0);
        self.table.reload_data();
    }
}

impl MenuView {
    fn command(self: Weak<Self>, command: String) {
        match command.as_str() {
            "panic" => {
                panic!("test panic");
            }
            "request" => {
                spawn(async move {
                    self.rest_pressed().await.unwrap();
                });
            }
            "pick folder" => {
                test_engine::dispatch::spawn(async {
                    Alert::show(format!("{:?}", Paths::pick_folder().await));
                });
            }
            "render" => {
                LevelManager::stop_level();
                UIManager::set_view(RenderView::new());
            }
            "no physics" => {
                UIManager::set_view(NoPhysicsView::new());
            }
            "noise" => {
                LevelManager::stop_level();
                UIManager::set_view(NoiseView::new().on_back(|| {
                    UIManager::set_view(Self::new());
                }));
            }
            "all views" => {
                dbg!(all_views!());
                dbg!(ALL_VIEWS);
                dbg!(all_view_tests!());
            }
            "ui bench" => {
                LevelManager::stop_level();
                UIManager::set_view(BenchmarkView::new());
            }
            "polygon" => {
                UIManager::set_view(PolygonView::new());
            }
            "benchmark" => {
                *LevelManager::camera_pos() = Point::default();
                LevelManager::set_level(BenchmarkLevel::default());
            }
            "root view" => {
                LevelManager::stop_level();
                UIManager::set_view(RootLayoutView::new());
            }
            "empty game" => {
                LevelManager::stop_level();
                UIManager::set_view(GameView::new());
            }
            "add box" => {
                let mut level = LevelManager::downcast_level::<TestLevel>();
                level.add_random_box((-20, 40));
            }
            "sound" => {
                Sound::get("retro.wav").play();
            }
            "alert" => {
                Alert::show("Hello!");
            }
            "spinner" => {
                let spin = Spinner::lock();
                after(2.0, || {
                    spin.animated_stop();
                });
            }
            "main" => {
                *LevelManager::camera_pos() = Point::default();
                LevelManager::set_level(TestLevel::default());
            }
            "system info" => {
                Alert::with_label(|l| {
                    l.set_text_size(15);
                })
                .show(netrun::System::get_info().dump());
            }
            _ => {
                panic!("Invalid command: {command}");
            }
        }
    }

    async fn rest_pressed(self: Weak<Self>) -> anyhow::Result<()> {
        let spin = Spinner::lock();

        let users = TEST_REST_REQUEST.await?;

        spin.stop();

        Alert::show(format!(
            "Got {} users. First name: {}",
            users.len(),
            users.first().unwrap().name
        ));

        Ok(())
    }
}

impl ViewTest for MenuView {
    fn perform_test(_view: Weak<Self>) -> Result<()> {
        // record_ui_test();

        Ok(())
    }
}
