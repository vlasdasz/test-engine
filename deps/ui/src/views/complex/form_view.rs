use std::{collections::HashMap, fmt::Debug, marker::PhantomData};

use reflected::{FieldRef, Reflected};
use refs::Weak;
use ui_proc::view;

use crate::{
    view::{ViewData, ViewSubviews},
    DropDown, InputView, Labeled, Switch, TextField, TextFieldConstraint, ViewSetup,
};
mod test_engine {
    pub(crate) use refs;

    pub(crate) use crate as ui;
}

#[view]
pub struct FormView<T: Debug + Reflected + 'static> {
    editind_enabled: bool,

    labels:   Vec<Weak<dyn InputView>>,
    variants: HashMap<FieldRef<T>, Vec<String>>,
    _p:       PhantomData<T>,
}

impl<T: Debug + Reflected> ViewSetup for FormView<T> {
    fn setup(mut self: Weak<Self>) {
        self.place().all_ver();
        self.editind_enabled = true;
    }
}

impl<T: Debug + Reflected> FormView<T> {
    pub fn add_variants(&mut self, field: FieldRef<T>, vals: Vec<String>) {
        self.variants.insert(field, vals);
    }

    pub fn set_data(&mut self, data: &T) {
        self.remove_all_subviews();
        self.labels.clear();

        for field in T::simple_fields() {
            let variant = self.variants.get(field).cloned();

            let text = &data.get_value(field);

            let mut view = if field.is_bool() {
                let mut view = self.add_view::<Labeled<Switch>>();
                view.input.set_on(text == "1");
                view.as_input_view()
            } else if let Some(variants) = variant {
                let variants = variants.clone();
                let mut view = self.add_view::<Labeled<DropDown>>();
                view.input.set_values(&variants);
                view.as_input_view()
            } else {
                let mut view = self.add_view::<Labeled<TextField>>();
                view.input.set_text(text);
                view.input.constraint = TextFieldConstraint::from_field(field);
                view.as_input_view()
            };

            view.set_title(field.name);

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
