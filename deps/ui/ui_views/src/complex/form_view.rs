use reflected::Reflected;
use refs::{Own, ToWeak, Weak};
use ui::{view, ViewSetup, ViewSubviews};

use crate::{basic::TextFieldConstraint, LabeledTextField};

#[view]
pub struct FormView<T: Reflected + 'static> {
    labels: Vec<Weak<LabeledTextField>>,
    data:   T,
}

impl<T: Reflected> ViewSetup for FormView<T> {
    fn setup(self: Weak<Self>) {
        self.place.all_ver();
    }
}

impl<T: Reflected> FormView<T> {
    pub fn set_data(&mut self, data: T) {
        self.remove_all_subviews();
        self.labels.clear();

        self.data = data;

        for field in T::simple_fields() {
            let view = Own::<LabeledTextField>::default();
            let mut rg = view.weak();
            self.add_subview(view);
            rg.text_field().constraint = TextFieldConstraint::from_field(field);
            rg.set_title(field.name);
            rg.set_text(self.data.get_value(field));
            self.labels.push(rg);
        }
    }

    pub fn get_data(&mut self) -> &T {
        for (field, label) in T::simple_fields().iter().zip(self.labels.iter()) {
            self.data.set_value(field, label.text());
        }
        &self.data
    }
}
