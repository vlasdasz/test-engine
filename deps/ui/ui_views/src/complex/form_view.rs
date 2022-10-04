use reflected::Reflected;
use refs::Own;
use ui::{view, ViewCallbacks, ViewSubviews};

use crate::LabeledTextField;

#[view]
#[derive(Default)]
pub struct FormView {}

impl ViewCallbacks for FormView {
    fn setup(&mut self) {
        self.place.all_ver();
    }
}

impl FormView {
    pub fn set_data<T: Reflected>(&mut self, data: T) {
        self.remove_all_subviews();

        for field in T::fields() {
            let mut view = Own::<LabeledTextField>::default();
            view.set_title(field.name);
            view.set_text(data.get_value(field));
            self.add_subview(view);
        }
    }
}
