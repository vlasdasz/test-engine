use std::{
    collections::{HashMap, HashSet},
    marker::PhantomData,
};

use convert_case::{Case, Casing};
use reflected::{Field, Reflected};
use refs::{weak_from_ref, Own, Weak};
use ui_proc::view;
use vents::Event;

use crate::{
    view::{ViewData, ViewSubviews},
    DropDown, InputView, Labeled, NumberView, Switch, TextAlignment, TextField, TextFieldConstraint,
    ViewSetup,
};
mod test_engine {
    pub(crate) use educe;
    pub(crate) use refs;

    pub(crate) use crate as ui;
}

#[view]
pub struct FormView<T: Reflected + 'static> {
    pub on_change: Event,

    editind_enabled: bool,

    labels:   Vec<Weak<dyn InputView>>,
    variants: HashMap<Field<T>, Vec<String>>,
    buttons:  HashSet<Field<T>>,
    _p:       PhantomData<T>,
}

impl<T: Reflected> ViewSetup for FormView<T> {
    fn setup(mut self: Weak<Self>) {
        self.place().all_ver();
        self.editind_enabled = true;
    }
}

impl<T: Reflected> FormView<T> {
    pub fn add_variants(&mut self, field: Field<T>, vals: Vec<String>) {
        self.variants.insert(field, vals);
    }

    pub fn buttons(&mut self, field: Field<T>) {
        self.buttons.insert(field);
    }

    pub fn set_data(&mut self, data: &T) {
        self.remove_all_subviews();
        self.labels.clear();

        let weak_self = weak_from_ref(self);

        for field in T::simple_fields() {
            let variant = self.variants.get(field).cloned();

            let text = &data.get_value(*field);

            let mut view: Weak<dyn InputView> = if field.is_bool() {
                let mut view = self.add_view::<Labeled<Switch>>();
                view.input.selected.sub(move || {
                    weak_self.on_change.trigger(());
                });
                view.input.set_on(text == "1");
                view.as_input_view()
            } else if let Some(variants) = variant {
                let variants = variants.clone();
                let mut view = self.add_view::<Labeled<DropDown>>();
                view.input.set_values(&variants);
                view.as_input_view()
            } else if field.is_number() && self.buttons.contains(field) {
                let view = labeled_for_field(*field);
                let mut weak = view.as_input_view();
                self.add_subview(view);
                weak.set_text(text);
                weak
            } else {
                let mut view = self.add_view::<Labeled<TextField>>();
                view.input.set_text(text);
                view.input.set_alignment(TextAlignment::Right);
                view.input.constraint = TextFieldConstraint::from_field(field);
                view.as_input_view()
            };

            view.set_title(&field.name.to_case(Case::Title));

            if self.editind_enabled {
                view.enable_editing();
            } else {
                view.disable_editing();
            }
            self.labels.push(view);
        }
    }

    pub fn get_data(&self) -> T {
        let mut data = T::default();
        for (field, label) in T::simple_fields().iter().zip(self.labels.iter()) {
            data.set_value(*field, label.text().into());
        }
        data
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

fn labeled_for_field<T: Send>(field: Field<T>) -> Own<dyn InputView> {
    match field.type_name {
        "i8" | "i16" | "i32" | "i64" => Labeled::<NumberView<i64>>::new(),
        "u8" | "u16" | "u32" | "u64" => Labeled::<NumberView<u64>>::new(),
        "f32" | "f64" => Labeled::<NumberView<f64>>::new(),
        _ => unimplemented!(),
    }
}
