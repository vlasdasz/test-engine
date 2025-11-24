use std::{any::type_name, ops::DerefMut};

use gm::{LossyConvert, color::Color};
use refs::{Own, Weak, weak_from_ref};

use crate::{Container, UIManager, View, ViewData, ViewFrame, WeakView};

pub trait ViewSubviews {
    fn __manually_set_superview(&mut self, superview: WeakView);
    fn superview(&self) -> &WeakView;
    fn subviews(&self) -> &[Own<dyn View>];
    fn subviews_mut(&mut self) -> Vec<WeakView>;
    fn remove_from_superview(&mut self);
    fn take_from_superview(&mut self) -> Own<dyn View>;
    fn remove_all_subviews(&mut self);

    fn add_view<V: 'static + View + Default>(&mut self) -> Weak<V>;
    fn add_subview(&mut self, view: Own<dyn View>) -> WeakView;
    fn __add_subview_internal(&mut self, view: Own<dyn View>, is_root: bool) -> WeakView;

    fn apply_if<V: View + 'static>(&mut self, action: impl FnMut(Weak<V>) + Clone + 'static);

    fn apply_to<V: View + 'static>(&mut self, action: impl FnMut(&mut V) + Clone + 'static);

    fn apply_to_all_subviews(&mut self, action: impl FnMut(&mut dyn View) + Clone + 'static);

    fn get_subview<V: 'static + View + Default>(&mut self) -> Weak<V>;
    fn dump_subviews(&self) -> Vec<String>;

    fn downcast_view<V: 'static + View>(&self) -> Option<Weak<V>>;

    fn outline(&mut self, color: Color) -> Weak<Self>;

    fn find_superview<V: View + 'static>(&self) -> Weak<V>;

    fn draw_on_top(&mut self);
}

impl<T: ?Sized + View> ViewSubviews for T {
    fn __manually_set_superview(&mut self, superview: WeakView) {
        self.base_view_mut().superview = superview;
        let weak = self.weak_view();
        self.base_view_mut().placer.init(weak);
    }

    fn superview(&self) -> &WeakView {
        &self.base_view().superview
    }

    fn subviews(&self) -> &[Own<dyn View>] {
        &self.base_view().subviews
    }

    fn subviews_mut(&mut self) -> Vec<WeakView> {
        self.base_view_mut().subviews.iter().map(Own::weak).collect()
    }

    fn remove_from_superview(&mut self) {
        let removed = self.take_from_superview();
        UIManager::get().deleted_views.lock().unwrap().push(removed);
    }

    fn take_from_superview(&mut self) -> Own<dyn View> {
        let this_addr = self.weak_view().raw();
        let super_subs = &mut self.base_view_mut().superview.base_view_mut().subviews;

        let index = super_subs.iter().position(|a| a.raw() == this_addr).unwrap();

        super_subs.remove(index)
    }

    fn remove_all_subviews(&mut self) {
        UIManager::get()
            .deleted_views
            .lock()
            .unwrap()
            .append(&mut self.base_view_mut().subviews);
    }

    fn add_view<V: 'static + View + Default>(&mut self) -> Weak<V> {
        let view = Own::<V>::default();
        let result = view.weak();
        self.add_subview(view);
        result
    }

    fn add_subview(&mut self, view: Own<dyn View>) -> WeakView {
        self.__add_subview_internal(view, false)
    }

    fn __add_subview_internal(&mut self, mut view: Own<dyn View>, is_root: bool) -> WeakView {
        assert!(
            is_root || self.superview().is_ok(),
            "Adding subview to view without superview is not allowed"
        );

        view.__internal_before_setup();

        if view.base_view().navigation_view.is_null() {
            view.base_view_mut().navigation_view = self.base_view().navigation_view;
        }
        let mut weak = view.weak_view();

        if weak.z_position() == UIManager::ROOT_VIEW_Z_OFFSET {
            weak.base_view_mut().z_position = self.z_position()
                - UIManager::subview_z_offset()
                - UIManager::additional_z_offset() * self.base_view().subviews.len().lossy_convert();
        }

        self.base_view_mut().subviews.push(view);
        weak.__manually_set_superview(self.weak_view());
        weak.init_views();
        weak.__internal_setup();
        weak
    }

    fn apply_if<V: View + 'static>(&mut self, mut action: impl FnMut(Weak<V>) + Clone + 'static) {
        if let Some(view_type) = self.downcast_view::<V>() {
            action(view_type);
        }
    }

    fn apply_to<V: View + 'static>(&mut self, mut action: impl FnMut(&mut V) + Clone + 'static) {
        for view in &mut self.base_view_mut().subviews {
            if let Some(mut view_type) = view.downcast_view::<V>() {
                action(&mut view_type);
            }
        }
    }

    fn apply_to_all_subviews(&mut self, mut action: impl FnMut(&mut dyn View) + Clone + 'static) {
        action(self.weak_view().deref_mut());
        for view in &mut self.base_view_mut().subviews {
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

    fn downcast_view<V: 'static + View>(&self) -> Option<Weak<V>> {
        self.weak_view().downcast::<V>()
    }

    fn outline(&mut self, color: Color) -> Weak<Self> {
        const WIDTH: f32 = 2.0;

        self.add_view::<Container>().set_color(color).place().lrt(0).h(WIDTH);
        self.add_view::<Container>().set_color(color).place().lrb(0).h(WIDTH);
        self.add_view::<Container>().set_color(color).place().t(0).l(0).b(0).w(WIDTH);
        self.add_view::<Container>().set_color(color).place().t(0).r(0).b(0).w(WIDTH);

        weak_from_ref(self)
    }

    fn find_superview<V: View + 'static>(&self) -> Weak<V> {
        let mut superview = self.base_view().superview;

        while superview.is_ok() {
            if let Some(view) = superview.downcast_view::<V>() {
                return view;
            }
            superview = superview.base_view().superview;
        }

        panic!("This view doesn't have `{}` in superview chain", type_name::<V>());
    }

    fn draw_on_top(&mut self) {
        let neighbours = self.superview().subviews();
        let last_z_pos = neighbours.last().unwrap().z_position();
        self.base_view_mut().z_position = last_z_pos - UIManager::subview_z_offset() * 2.0;
    }
}
