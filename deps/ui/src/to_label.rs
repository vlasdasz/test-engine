use rust_decimal::Decimal;

pub trait ToLabel {
    fn to_label(&self) -> String;
}

impl ToLabel for &str {
    fn to_label(&self) -> String {
        self.to_string()
    }
}

impl ToLabel for String {
    fn to_label(&self) -> String {
        self.to_string()
    }
}

impl ToLabel for &String {
    fn to_label(&self) -> String {
        self.to_string()
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
