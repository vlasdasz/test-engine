use std::{fmt::Debug, ops::DerefMut};

use gl_image::Image;
use gm::{Color, Point, Rect};
use rtools::{address::Address, Boxed, Rglica, ToRglica};

use crate::{basic::Placer, complex::PathData, input::Touch, view_base::ViewBase};

pub trait View: Boxed + Debug {
    fn setup(&mut self) {}

    fn layout(&mut self) {}

    fn on_touch(&mut self, _: &Touch) {}

    fn update(&mut self) {}

    fn color(&self) -> Color {
        self.view().color
    }

    fn set_color(&mut self, color: Color) {
        self.view_mut().color = color
    }

    fn is_hidden(&self) -> bool {
        self.view().is_hidden
    }

    fn set_hidden(&mut self, hidden: bool) {
        self.view_mut().is_hidden = hidden
    }

    fn root_view(&self) -> Rglica<ViewBase> {
        let mut root = self.superview();
        loop {
            if root.superview().is_null() {
                return root;
            }
            root = root.superview();
        }
    }

    fn superview(&self) -> Rglica<ViewBase> {
        self.view().superview
    }

    fn super_frame(&self) -> &Rect {
        if self.view().superview.is_ok() {
            return self.view().superview.frame();
        }
        self.frame()
    }

    fn super_absolute_frame(&self) -> &Rect {
        if self.view().superview.is_ok() {
            return self.view().superview.absolute_frame();
        }
        self.absolute_frame()
    }

    fn frame(&self) -> &Rect {
        &self.view().frame
    }

    fn frame_mut(&mut self) -> &mut Rect {
        &mut self.view_mut().frame
    }

    fn set_frame(&mut self, rect: Rect) {
        self.view_mut().frame = rect
    }

    fn add_view_at(&mut self, point: Point) {
        let mut view = ViewBase::dummy();
        view.frame_mut().origin = point;
        self.add_subview(view);
    }

    fn x(&self) -> f32 {
        self.frame().origin.x
    }

    fn y(&self) -> f32 {
        self.frame().origin.y
    }

    fn max_x(&self) -> f32 {
        self.frame().max_x()
    }

    fn max_y(&self) -> f32 {
        self.frame().max_y()
    }

    fn width(&self) -> f32 {
        self.frame().size.width
    }

    fn height(&self) -> f32 {
        self.frame().size.height
    }

    fn absolute_frame(&self) -> &Rect {
        &self.view().absolute_frame
    }

    fn add_subview(&mut self, mut view: Box<dyn View>) {
        view.view_mut().superview = self.view().to_rglica();
        view.view_mut().placer = Placer::make(view.deref_mut());
        view.setup();
        self.view_mut().subviews.push(view);
    }

    fn remove_all_subviews(&mut self) {
        self.view_mut().subviews.clear()
    }

    fn remove_from_superview(&mut self) {
        let index = self
            .superview()
            .subviews()
            .iter()
            .position(|view| self.address() == view.address())
            .unwrap();

        self.superview().remove_subview_at(index);
    }

    fn remove_subview_at(&mut self, index: usize) {
        self.view_mut().subviews.remove(index);
    }

    fn subviews(&self) -> &[Box<dyn View>] {
        &self.view().subviews
    }

    fn subviews_mut(&mut self) -> &mut [Box<dyn View>] {
        &mut self.view_mut().subviews
    }

    fn calculate_frames(&mut self) {
        let view = self.view_mut();
        view.absolute_frame = view.frame;
        view.absolute_frame.origin += view.super_absolute_frame().origin;
        self.layout();
        for view in self.subviews_mut() {
            view.calculate_frames();
        }
    }

    fn enable_touch(&mut self) {
        self.view_mut().touch_enabled = true
    }

    fn touch_enabled(&self) -> bool {
        self.view().touch_enabled
    }

    fn touch_id(&self) -> u64 {
        self.view().touch_id
    }

    fn set_touch_id(&mut self, id: u64) {
        self.view_mut().touch_id = id;
    }

    fn check_touch(&mut self, touch: &mut Touch) -> bool {
        if self.touch_enabled() {
            if touch.is_moved() && self.touch_id() == touch.id {
                touch.position -= self.absolute_frame().origin;
                self.on_touch(touch);
                return true;
            }

            if touch.is_moved() {
                return false;
            }

            if touch.is_ended() && self.touch_id() == touch.id {
                touch.position -= self.absolute_frame().origin;
                self.set_touch_id(0);
                self.on_touch(touch);
                return true;
            }

            if self.absolute_frame().contains(&touch.position) {
                touch.position -= self.absolute_frame().origin;
                self.set_touch_id(touch.id);
                self.on_touch(touch);
                return true;
            }
        }

        for view in self.subviews_mut() {
            if view.check_touch(touch) {
                return true;
            }
        }

        false
    }

    fn paths(&self) -> Option<&[PathData]> {
        None
    }

    fn image(&self) -> Option<Image> {
        None
    }

    fn set_image(&mut self, _: Image) {}

    fn place(&mut self) -> &mut Placer {
        &mut self.view_mut().placer
    }

    fn view(&self) -> &ViewBase;
    fn view_mut(&mut self) -> &mut ViewBase;

    fn with_frame(frame: Rect) -> Box<Self>
    where
        Self: Sized,
    {
        let mut new = Self::boxed();
        new.set_frame(frame);
        new
    }
}
