
pub fn log<T: std::fmt::Debug>(message: &T) {
    print!("{:?}\n", message)
}
