use reflected::Reflected;
use refs::{Own, ToWeak};
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
            if field.name.contains("id") || field.is_custom() {
                continue;
            }
            let view = Own::<LabeledTextField>::default();
            let mut rg = view.weak();
            self.add_subview(view);
            rg.set_title(field.name);
            rg.set_text(data.get_value(field));
        }
    }
}
