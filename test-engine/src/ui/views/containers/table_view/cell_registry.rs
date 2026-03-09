use std::{any::type_name, collections::HashMap};

use netrun::Function;
use refs::Own;
use ui::{View, ViewData};

#[derive(Default)]
pub struct CellRegistry {
    pub free_cells:   HashMap<&'static str, Vec<Own<dyn View>>>,
    pub constructors: HashMap<&'static str, Function<(), Own<dyn View>>>,
}

impl CellRegistry {
    pub(crate) fn load_old_cells(&mut self, mut cells: Vec<Own<dyn View>>) {
        for cell in cells.drain(..) {
            dbg!(&cell.label());
        }
    }

    pub(crate) fn cell_for_ident(&mut self, ident: &'static str) -> Own<dyn View> {
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

    pub fn get_cell<T: View + 'static>(&mut self) -> Own<T> {
        self.cell_for_ident(type_name::<T>()).downcast::<T>()
    }
}
