
pub trait ViewCallbacks {
    fn on_update<Obj: 'static>(&self, _: &Obj, _: impl FnMut(&mut Obj) + 'static);
}
