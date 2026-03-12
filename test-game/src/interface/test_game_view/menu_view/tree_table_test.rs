use anyhow::Result;
use parking_lot::Mutex;
use test_engine::{
    gm::Toggle,
    refs::Weak,
    ui::{CellRegistry, Setup, TableData, TableView, View, ViewData, ViewFrame, ViewTest, WHITE, view_test},
};

use crate::interface::test_game_view::{MenuEntry, Node, NodeCell};

static DATA: Mutex<String> = Mutex::new(String::new());

#[view_test]
struct TreeTableTest {
    root: Node<MenuEntry>,

    #[init]
    table: TableView,
}

impl Setup for TreeTableTest {
    fn setup(mut self: Weak<Self>) {
        self.table.set_size(400, 400);
        self.table.set_data_source(self).register_cell::<NodeCell>();

        self.root = Node::new(
            MenuEntry::new("Root"),
            vec![
                Node::new(
                    MenuEntry::new("food"),
                    vec![
                        MenuEntry::new("pizza")
                            .action(|| {
                                *DATA.lock() += "|pizza|";
                            })
                            .into(),
                        MenuEntry::new("borgor")
                            .action(|| {
                                *DATA.lock() += "|borgor|";
                            })
                            .into(),
                        MenuEntry::new("ojje")
                            .action(|| {
                                *DATA.lock() += "|ojje|";
                            })
                            .into(),
                    ],
                ),
                Node::new(
                    MenuEntry::new("dogs"),
                    vec![
                        MenuEntry::new("barker")
                            .action(|| {
                                *DATA.lock() += "|barker|";
                            })
                            .into(),
                        MenuEntry::new("woofer")
                            .action(|| {
                                *DATA.lock() += "|woofer|";
                            })
                            .into(),
                    ],
                ),
            ],
        );
    }
}

impl TableData for TreeTableTest {
    fn cell_height(&self, _: usize) -> f32 {
        100.0
    }

    fn number_of_cells(&self) -> usize {
        self.root.length()
    }

    fn setup_cell(&mut self, index: usize, registry: &mut CellRegistry) -> Weak<dyn View> {
        let node = self.root.val_at_index(index);

        let cell = registry.cell::<NodeCell>();

        let node = node.clone();

        cell.set_node(&node);
        cell.label.set_color(WHITE);

        cell
    }

    fn cell_selected(&mut self, index: usize) {
        let val = self.root.val_at_index(index);

        if val.is_leaf() {
            val.value.run();
            return;
        }

        val.open.toggle();
        self.root.update_indices(0, 0);
        self.table.reload_data();
    }
}

impl ViewTest for TreeTableTest {
    fn perform_test(_view: Weak<Self>) -> Result<()> {
        // test_engine::ui_test::record_ui_test();

        Ok(())
    }
}
