use refs::{Own, ToWeak, Weak};

use crate::{layout::Placer, SubView, View};
pub trait ViewSubviews {
    fn superview(&self) -> Weak<dyn View>;
    fn subviews(&self) -> &[Own<dyn View>];
    fn subviews_mut(&mut self) -> &mut [Own<dyn View>];
    fn remove_from_superview(&mut self);
    fn remove_all_subviews(&mut self);

    fn add_view<V: 'static + View + Default>(&mut self) -> SubView<V>;
    fn add_subview(&mut self, view: Own<dyn View>) -> Weak<dyn View>;
}

impl<T: ?Sized + View> ViewSubviews for T {
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
        let this_addr = self.weak_view().addr();
        let super_subs = &mut self.superview.subviews;

        let index = super_subs.iter().position(|a| a.addr() == this_addr).unwrap();

        let removed = super_subs.remove(index);

        self.superview.deleted_subviews.push(removed);
    }

    fn remove_all_subviews(&mut self) {
        self.deleted_subviews = self.subviews.drain(..).collect();
    }

    fn add_view<V: 'static + View + Default>(&mut self) -> SubView<V> {
        let view = Own::<V>::default();
        let result = view.weak();
        self.add_subview(view);
        result.into()
    }

    fn add_subview(&mut self, mut view: Own<dyn View>) -> Weak<dyn View> {
        view.superview = self.weak_view();
        view.place = Placer::new(view.weak_view()).into();
        view.init_views();
        view.setup();
        let res = view.weak();
        self.subviews.push(view);
        res
    }
}
