use chrono::NaiveDateTime;
use gm::flat::Point;
use rust_decimal::Decimal;

pub trait ToLabel {
    fn to_label(&self) -> String;
}

impl ToLabel for &str {
    fn to_label(&self) -> String {
        (*self).to_string()
    }
}

impl ToLabel for String {
    fn to_label(&self) -> String {
        self.to_string()
    }
}

impl ToLabel for &String {
    fn to_label(&self) -> String {
        (*self).to_string()
    }
}

impl ToLabel for bool {
    fn to_label(&self) -> String {
        self.to_string()
    }
}

impl ToLabel for u32 {
    fn to_label(&self) -> String {
        self.to_string()
    }
}

impl ToLabel for usize {
    fn to_label(&self) -> String {
        self.to_string()
    }
}

impl ToLabel for f32 {
    fn to_label(&self) -> String {
        format!("{self:.2}")
    }
}

impl ToLabel for Decimal {
    fn to_label(&self) -> String {
        format!("{self:.2}")
    }
}

impl ToLabel for Point {
    fn to_label(&self) -> String {
        format!("x: {} y: {}", self.x.to_label(), self.y.to_label())
    }
}

// TODO: think about timezones
impl ToLabel for NaiveDateTime {
    fn to_label(&self) -> String {
        self.format("%Y-%m-%d %H:%M:%S").to_string()
    }
}
