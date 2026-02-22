use std::{
    any::Any,
    fs::OpenOptions,
    io::{Read, Write},
    path::PathBuf,
};

use anyhow::Result;
use netrun::Function;
use test_engine::{
    audio::Sound,
    dispatch::{after, from_back, spawn},
    filesystem::Paths,
    gm::Toggle,
    level::LevelManager,
    refs::{Own, Weak, manage::DataManager},
    ui::{
        ALL_VIEWS, Alert, AlertErr, Image, Point, Setup, Spinner, TableData, TableView, UIManager, View,
        ViewData, ViewFrame, ViewTest, all_view_tests, all_views, cast_cell, view_test,
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
        test_game_view::{Node, NodeCell, ScaleCell, UIBenchmarkView},
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
                Node::new("Level", vec![Node::empty("benchmark"), Node::empty("lvl scale")]),
                Node::new(
                    "System",
                    vec![
                        Node::empty("system info"),
                        Node::empty("cloud"),
                        Node::empty("add box"),
                        Node::empty("load assets"),
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
            ScaleCell::make(
                Function::new(|()| UIManager::scale()),
                Function::new(UIManager::set_scale),
            )
        } else if node.value == "lvl scale" {
            ScaleCell::make(
                Function::new(|()| LevelManager::scale()),
                Function::new(LevelManager::set_scale),
            )
        } else {
            NodeCell::new()
        }
    }

    fn setup_cell(mut self: Weak<Self>, cell: &mut dyn Any, index: usize) {
        let node = self.root.val_at_index(index);
        if node.value == "ui scale" || node.value == "lvl scale" {
            cast_cell!(ScaleCell).set_node(node);
        } else {
            cast_cell!(NodeCell).set_node(node);
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
                UIManager::set_view(UIBenchmarkView::new());
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
            "load assets" => {
                from_back(load_assets_test, |result| {
                    let Some(image) = result.alert_err() else {
                        return;
                    };

                    LevelManager::level_weak().background = image;
                });
            }
            "cloud" => {
                let Some(path) = UIManager::cloud_storage_dir() else {
                    Alert::show("No path!");
                    return;
                };

                let path = path.to_string_lossy();
                let path = path.trim_start_matches("file://");

                let mut path = PathBuf::from(path);

                dbg!(&path);

                // 2. IMPORTANT: Append "Documents"
                // iCloud only syncs/shows files inside this specific subfolder
                path.push("Documents");

                dbg!(&path);

                // 3. Create the Documents directory if it's missing
                if !path.exists() {
                    std::fs::create_dir_all(&path).unwrap();
                }

                let mut file = OpenOptions::new()
                    .read(true)
                    .write(true)
                    .create(true)
                    .open(path.join("data.txt"))
                    .unwrap();

                // 2. Read existing content
                let mut content = String::new();
                dbg!(&content);
                file.read_to_string(&mut content).unwrap();

                let mut number: i32 = content.parse().unwrap_or_default();

                println!("Existing content: '{number}'");

                number += 1;

                // 3. Write new content
                // Note: After reading, the cursor is at the end of the file.
                // If you want to overwrite or append, manage the cursor or use .append(true)
                file.write_all(number.to_string().as_bytes()).unwrap();

                Alert::show(format!("{path:?}"));
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

async fn load_assets_test() -> Result<Weak<Image>> {
    let _spin = Spinner::lock();

    Image::download(
        "downloaded.jpg",
        "https://fastly.picsum.photos/id/299/1000/1000.jpg?hmac=DRpkgVaALpt0f0Y4kSTUOtLJ66_ULgUDZn2n6pbuafA",
    )
    .await
}
