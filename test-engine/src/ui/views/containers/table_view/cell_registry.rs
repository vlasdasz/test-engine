use std::{any::type_name, collections::HashMap};

use netrun::Function;
use refs::Own;
use ui::{View, ViewData};

#[derive(Default)]
pub struct CellRegistry {
    pub free_cells:   HashMap<String, Vec<Own<dyn View>>>,
    pub constructors: HashMap<&'static str, Function<(), Own<dyn View>>>,
}

impl CellRegistry {
    pub(crate) fn load_old_cells(&mut self, mut cells: Vec<Own<dyn View>>) {
        for cell in cells.drain(..) {
            self.free_cells.entry(cell.label().to_string()).or_default().push(cell);
        }
    }

    pub(crate) fn cell_for_ident(&mut self, ident: &'static str) -> Own<dyn View> {
        let container = self.free_cells.entry(ident.to_string()).or_default();

        if let Some(cell) = container.pop() {
            return cell;
        }

        let constructor = self
            .constructors
            .get(ident)
            .unwrap_or_else(|| panic!("Constructor for cell identifier: {ident} is not registered. Use TableView::register_cell method."));

        constructor.call(())
    }

    pub fn cell<T: View + 'static>(&mut self) -> Own<T> {
        self.cell_for_ident(struct_name::<T>()).downcast::<T>()
    }

    pub fn cell_with_id<T: View + 'static>(&mut self, id: &'static str) -> Own<T> {
        self.cell_for_ident(id).downcast::<T>()
    }
}

pub(crate) fn struct_name<T>() -> &'static str {
    let full_name = type_name::<T>();
    full_name.split("::").last().unwrap_or(full_name)
}
