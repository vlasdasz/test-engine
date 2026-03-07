use std::collections::HashMap;

use netrun::Function;
use refs::Own;
use ui::View;

#[derive(Default)]
pub(crate) struct CellRegistry {
    pub free_cells:   HashMap<&'static str, Vec<Own<dyn View>>>,
    pub constructors: HashMap<&'static str, Function<(), Own<dyn View>>>,
}

impl CellRegistry {
    pub fn cell_for_ident(&mut self, ident: &'static str) -> Own<dyn View> {
        let container = self.free_cells.entry(ident).or_default();

        if let Some(cell) = container.pop() {
            return cell;
        }

        let constructor = self
            .constructors
            .get(ident)
            .unwrap_or_else(|| panic!("Constructor for cell identifier: {ident} is not registered"));

        constructor.call(())
    }
}
