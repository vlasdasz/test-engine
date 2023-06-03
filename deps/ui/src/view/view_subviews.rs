use refs::{Own, ToWeak, Weak};
use rtools::Random;

use crate::{layout::Placer, Container, SubView, UIManager, View, ViewFrame};
pub trait ViewSubviews {
    /// Use this only if you know what you are doing
    fn manually_set_superview(&mut self, superview: Weak<dyn View>);
    fn superview(&self) -> Weak<dyn View>;
    fn subviews(&self) -> &[Own<dyn View>];
    fn subviews_mut(&mut self) -> &mut [Own<dyn View>];
    fn remove_from_superview(&mut self);
    fn take_from_superview(&mut self) -> Own<dyn View>;
    fn remove_all_subviews(&mut self);

    fn internal_add_view<V: View + Default + 'static>(&mut self) -> SubView<V>;
    fn add_view<V: 'static + View + Default>(&mut self) -> Weak<V>;
    fn add_subview(&mut self, view: Own<dyn View>) -> Weak<dyn View>;

    fn add_dummy_view(&mut self);
}

impl<T: ?Sized + View> ViewSubviews for T {
    /// Use this only if you know what you are doing
    fn manually_set_superview(&mut self, superview: Weak<dyn View>) {
        self.superview = superview;
        self.place = Placer::new(self.weak_view()).into();
    }

    fn superview(&self) -> Weak<dyn View> {
        self.superview
    }

    fn subviews(&self) -> &[Own<dyn View>] {
        &self.subviews
    }

    fn subviews_mut(&mut self) -> &mut [Own<dyn View>] {
        &mut self.subviews
    }

    fn remove_from_superview(&mut self) {
        let removed = self.take_from_superview();
        UIManager::get().deleted_views.push(removed);
    }

    fn take_from_superview(&mut self) -> Own<dyn View> {
        let this_addr = self.weak_view().addr();
        let super_subs = &mut self.superview.subviews;

        let index = super_subs.iter().position(|a| a.addr() == this_addr).unwrap();

        super_subs.remove(index)
    }

    fn remove_all_subviews(&mut self) {
        UIManager::get().deleted_views.append(&mut self.subviews);
    }

    fn internal_add_view<V: 'static + View + Default>(&mut self) -> SubView<V> {
        let view = Own::<V>::default();
        let result = view.weak();
        self.add_subview(view);
        result.into()
    }

    fn add_view<V: 'static + View + Default>(&mut self) -> Weak<V> {
        self.internal_add_view::<V>().weak()
    }

    fn add_subview(&mut self, mut view: Own<dyn View>) -> Weak<dyn View> {
        view.priority = self.priority;
        view.manually_set_superview(self.weak_view());
        view.init_views();
        view.internal_setup();
        let res = view.weak();
        self.subviews.push(view);
        res
    }

    fn add_dummy_view(&mut self) {
        let mut view = self.internal_add_view::<Container>();
        view.set_size((f32::random(), f32::random()));
    }
}
