use gm::{flat::point::Direction, Point};

pub trait Control {
    fn jump(&mut self);
    fn go_left(&mut self);
    fn go_right(&mut self);
    fn go_down(&mut self);
    fn add_impulse(&mut self, impulse: Point);

    fn move_by_key(&mut self, key: String) {
        match key.as_ref() {
            "a" => self.go_left(),
            "d" => self.go_right(),
            "w" | " " => self.jump(),
            "s" => self.go_down(),
            _ => {}
        }
    }

    fn move_by_direction(&mut self, direction: Direction) {
        match direction {
            Direction::Up => self.jump(),
            Direction::Down => self.go_down(),
            Direction::Left => self.go_left(),
            Direction::Right => self.go_right(),
        }
    }
}
