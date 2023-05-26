use std::marker::PhantomData;

use reflected::Reflected;
use refs::{Own, ToWeak, Weak};
use ui::{view, ViewSetup, ViewSubviews};

use crate::{basic::TextFieldConstraint, LabeledTextField};

#[view]
pub struct FormView<T: Reflected + 'static> {
    labels:          Vec<Weak<LabeledTextField>>,
    editind_enabled: bool,
    _p:              PhantomData<T>,
}

impl<T: Reflected> ViewSetup for FormView<T> {
    fn setup(mut self: Weak<Self>) {
        self.place.all_ver();
        self.editind_enabled = true;
    }
}

impl<T: Reflected> FormView<T> {
    pub fn set_data(&mut self, data: Weak<T>) {
        self.remove_all_subviews();
        self.labels.clear();

        for field in T::simple_fields() {
            let view = Own::<LabeledTextField>::default();
            let mut rg = view.weak();
            self.add_subview(view);
            rg.text_field().constraint = TextFieldConstraint::from_field(field);
            rg.set_title(field.name);
            rg.set_text(data.get_value(field));
            if self.editind_enabled {
                rg.enable_editing();
            } else {
                rg.disable_editing();
            }
            self.labels.push(rg);
        }
    }

    pub fn get_data(&self, data: &mut T) {
        for (field, label) in T::simple_fields().iter().zip(self.labels.iter()) {
            data.set_value(field, label.text());
        }
    }

    pub fn enable_editing(&mut self) -> &mut Self {
        self.editind_enabled = true;
        for label in &mut self.labels {
            label.enable_editing();
        }
        self
    }

    pub fn disable_editing(&mut self) -> &mut Self {
        self.editind_enabled = false;
        for label in &mut self.labels {
            label.disable_editing();
        }
        self
    }

    pub fn editing_enabled(&self) -> bool {
        self.editind_enabled
    }

    pub fn editing_disabled(&self) -> bool {
        !self.editing_enabled()
    }
}
