use gl_wrapper::path_data::{initialize_path_data, DrawMode, PathData};
use gm::{
    axis::Axis,
    flat::{Points, Size},
    Color,
};
use ui::{view, ViewFrame};

#[view]
pub struct DrawingView {
    pub rescale: bool,
    paths:       Vec<PathData>,
}

impl DrawingView {
    pub fn paths(&self) -> &[PathData] {
        &self.paths
    }

    pub fn add_path(&mut self, path: Points, color: &Color, mode: DrawMode) -> &mut Self {
        let path = self.process_points(path);
        if path.is_empty() {
            return self;
        }
        self.paths.push(initialize_path_data(path, color, mode));
        self
    }

    fn process_points(&self, path: Points) -> Points {
        if !self.rescale {
            return path.into();
        }

        let max_x = path.iter().map(|p| p.x).fold(f32::NAN, f32::max);
        let max_y = path.iter().map(|p| p.y).fold(f32::NAN, f32::max);

        let path_size = Size::new(max_x, max_y);

        let fitted_size = path_size.fit_in_rect::<{ Axis::X }>(self.frame()).size;

        let ratios = path_size.ratios(fitted_size);

        path.into_iter().map(|point| point * ratios).collect()
    }

    pub fn remove_all_paths(&mut self) {
        self.paths.clear()
    }
}
