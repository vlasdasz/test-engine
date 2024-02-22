use std::ops::DerefMut;

use refs::{Own, Weak};
use rtools::Random;

use crate::{layout::Placer, Container, SubView, UIManager, View, ViewFrame, WeakView};

pub trait ViewSubviews {
    /// Use this only if you know what you are doing
    fn manually_set_superview(&mut self, superview: WeakView);
    fn superview(&self) -> WeakView;
    fn subviews(&self) -> &[Own<dyn View>];
    fn subviews_mut(&mut self) -> &mut [Own<dyn View>];
    fn remove_from_superview(&mut self);
    fn take_from_superview(&mut self) -> Own<dyn View>;
    fn remove_all_subviews(&mut self);

    fn __internal_add_view<V: View + Default + 'static>(&mut self) -> SubView<V>;
    fn add_view<V: 'static + View + Default>(&mut self) -> Weak<V>;
    fn add_subview(&mut self, view: Own<dyn View>) -> WeakView;

    fn add_dummy_view(&mut self);

    fn apply_to_all_subviews(&mut self, action: impl FnMut(&mut dyn View) + Clone + 'static);
}

impl<T: ?Sized + View> ViewSubviews for T {
    /// Use this only if you know what you are doing
    fn manually_set_superview(&mut self, superview: WeakView) {
        self.base_mut().superview = superview;
        self.base_mut().placer = Placer::new(self.weak_view());
    }

    fn superview(&self) -> WeakView {
        self.base().superview
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

    fn __internal_add_view<V: 'static + View + Default>(&mut self) -> SubView<V> {
        let view = Own::<V>::default();
        let result = view.weak();
        self.add_subview(view);
        result.into()
    }

    fn add_view<V: 'static + View + Default>(&mut self) -> Weak<V> {
        self.__internal_add_view::<V>().weak()
    }

    fn add_subview(&mut self, mut view: Own<dyn View>) -> WeakView {
        if view.base().navigation_view.is_null() {
            view.base_mut().navigation_view = self.base().navigation_view;
        }
        let mut weak = view.weak_view();
        weak.base_mut().z_position = self.z_position() - UIManager::SUPERVIEW_Z_OFFSET;
        if let Some(last_subview) = self.subviews().last() {
            weak.base_mut().z_position = last_subview.z_position() - UIManager::subview_z_offset();
        }
        self.base_mut().subviews.push(view);
        weak.manually_set_superview(self.weak_view());
        weak.init_views();
        weak.__internal_setup();
        weak.base().loaded.trigger(());
        weak
    }

    fn add_dummy_view(&mut self) {
        let mut view = self.__internal_add_view::<Container>();
        view.set_size((f32::random(), f32::random()));
    }

    fn apply_to_all_subviews(&mut self, mut action: impl FnMut(&mut dyn View) + Clone + 'static) {
        action(self.weak_view().deref_mut());
        for view in &mut self.base_mut().subviews {
            view.apply_to_all_subviews(action.clone());
        }
    }
}
