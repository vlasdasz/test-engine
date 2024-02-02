use std::{cell::RefCell, collections::HashMap, fmt::Debug, marker::PhantomData};

use reflected::{FieldRef, Reflected};
use refs::Weak;
use ui::{view, Labeled, TextFieldConstraint, ViewData, ViewSetup, ViewSubviews};

use crate::{LabeledDrop, LabeledTextField};

#[view]
pub struct FormView<T: Debug + Reflected + 'static> {
    labels:          Vec<Weak<dyn Labeled>>,
    editind_enabled: bool,
    variants:        RefCell<HashMap<FieldRef<T>, Vec<String>>>,
    _p:              PhantomData<T>,
}

impl<T: Debug + Reflected> ViewSetup for FormView<T> {
    fn setup(mut self: Weak<Self>) {
        self.place().all_ver();
        self.editind_enabled = true;
    }
}

impl<T: Debug + Reflected> FormView<T> {
    pub fn add_variants(&self, field: FieldRef<T>, vals: Vec<String>) {
        self.variants.borrow_mut().insert(field, vals);
    }

    pub fn set_data(&mut self, data: Weak<T>) {
        self.remove_all_subviews();
        self.labels.clear();

        for field in T::simple_fields() {
            let variant = self.variants.borrow().get(field).cloned();

            let mut view = if let Some(variants) = variant {
                let variants = variants.clone();
                let mut view = self.add_view::<LabeledDrop>();
                view.set_values(&variants);
                view.labeled()
            } else {
                let mut view = self.add_view::<LabeledTextField>();
                view.set_text(&data.get_value(field));
                view.set_constraint(TextFieldConstraint::from_field(field));
                view.labeled()
            };

            view.set_title(&field.name);

            if self.editind_enabled {
                view.enable_editing();
            } else {
                view.disable_editing();
            }
            self.labels.push(view);
        }
    }

    pub fn get_data(&self, data: &mut T) {
        for (field, label) in T::simple_fields().iter().zip(self.labels.iter()) {
            data.set_value(field, label.text().into());
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
