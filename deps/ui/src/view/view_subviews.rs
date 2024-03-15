use std::{any::type_name, ops::DerefMut};

use fake::Fake;
use gm::{
    flat::{Point, Size},
    Color,
};
use refs::{Own, Weak};

use crate::{layout::Placer, Container, UIManager, View, ViewData, ViewFrame, WeakView};

pub trait ViewSubviews {
    fn __manually_set_superview(&mut self, superview: WeakView);
    fn superview(&self) -> &WeakView;
    fn subviews(&self) -> &[Own<dyn View>];
    fn subviews_mut(&mut self) -> &mut [Own<dyn View>];
    fn remove_from_superview(&mut self);
    fn take_from_superview(&mut self) -> Own<dyn View>;
    fn remove_all_subviews(&mut self);

    fn add_view<V: 'static + View + Default>(&mut self) -> Weak<V>;
    fn add_subview(&mut self, view: Own<dyn View>) -> WeakView;

    fn add_dummy_view(&mut self);

    fn apply_to_all_subviews(&mut self, action: impl FnMut(&mut dyn View) + Clone + 'static);

    fn get_subview<V: 'static + View + Default>(&mut self) -> Weak<V>;
    fn dump_subviews(&self) -> Vec<String>;
}

impl<T: ?Sized + View> ViewSubviews for T {
    fn __manually_set_superview(&mut self, superview: WeakView) {
        self.base_mut().superview = superview;
        self.base_mut().placer = Placer::new(self.weak_view());
    }

    fn superview(&self) -> &WeakView {
        &self.base().superview
    }

    fn subviews(&self) -> &[Own<dyn View>] {
        &self.base().subviews
    }

    fn subviews_mut(&mut self) -> &mut [Own<dyn View>] {
        &mut self.base_mut().subviews
    }

    fn remove_from_superview(&mut self) {
        let removed = self.take_from_superview();
        UIManager::get().deleted_views.lock().unwrap().push(removed);
    }

    fn take_from_superview(&mut self) -> Own<dyn View> {
        let this_addr = self.weak_view().addr();
        let super_subs = &mut self.base_mut().superview.base_mut().subviews;

        let index = super_subs.iter().position(|a| a.addr() == this_addr).unwrap();

        super_subs.remove(index)
    }

    fn remove_all_subviews(&mut self) {
        UIManager::get()
            .deleted_views
            .lock()
            .unwrap()
            .append(&mut self.base_mut().subviews);
    }

    fn add_view<V: 'static + View + Default>(&mut self) -> Weak<V> {
        let view = Own::<V>::default();
        let result = view.weak();
        self.add_subview(view);
        result
    }

    fn add_subview(&mut self, mut view: Own<dyn View>) -> WeakView {
        if view.base().navigation_view.is_null() {
            view.base_mut().navigation_view = self.base().navigation_view;
        }
        let mut weak = view.weak_view();

        if weak.z_position() == UIManager::ROOT_VIEW_Z_OFFSET {
            weak.base_mut().z_position = self.z_position() - UIManager::SUPERVIEW_Z_OFFSET;
            if let Some(last_subview) = self.subviews().last() {
                weak.base_mut().z_position = last_subview.z_position() - UIManager::subview_z_offset();
            }
        }

        self.base_mut().subviews.push(view);
        weak.__manually_set_superview(self.weak_view());
        weak.init_views();
        weak.__internal_setup();
        weak.base().loaded.trigger(());
        weak
    }

    fn add_dummy_view(&mut self) {
        const MAX_SIZE: f32 = 200.0;
        const MAX_POSITION: f32 = 400.0;

        let mut view = self.add_view::<Container>();

        let size: Size = ((10.0..MAX_SIZE).fake::<f32>(), (10.0..MAX_SIZE).fake::<f32>()).into();
        view.set_size(size);

        let origin: Point = (
            (10.0..MAX_POSITION).fake::<f32>(),
            (10.0..MAX_POSITION).fake::<f32>(),
        )
            .into();

        view.set_position(origin);

        view.set_color(Color::random());
    }

    fn apply_to_all_subviews(&mut self, mut action: impl FnMut(&mut dyn View) + Clone + 'static) {
        action(self.weak_view().deref_mut());
        for view in &mut self.base_mut().subviews {
            view.apply_to_all_subviews(action.clone());
        }
    }

    fn get_subview<V: 'static + View + Default>(&mut self) -> Weak<V> {
        for sub in self.subviews() {
            if let Some(view) = sub.downcast::<V>() {
                return view;
            }
        }

        panic!("View of type: {} not found", type_name::<V>());
    }

    fn dump_subviews(&self) -> Vec<String> {
        self.subviews().iter().map(|v| v.label().to_string()).collect()
    }
}
