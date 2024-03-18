use smart_default::SmartDefault;

use crate::num::into_f32::IntoF32;

#[derive(Debug, SmartDefault)]
pub struct Converter {
    #[default(0.0)]
    min:  f32,
    #[default(1.0)]
    max:  f32,
    #[default(1.0)]
    span: f32,
}

impl Converter {
    pub fn new(min: impl IntoF32, max: impl IntoF32) -> Self {
        let min = min.into_f32();
        let max = max.into_f32();
        Self {
            min,
            max,
            span: max - min,
        }
    }

    pub fn set_min(&mut self, min: impl IntoF32) -> &mut Self {
        self.min = min.into_f32();
        self.span = self.max - self.min;
        self
    }

    pub fn set_max(&mut self, max: impl IntoF32) -> &mut Self {
        self.max = max.into_f32();
        self.span = self.max - self.min;
        self
    }

    pub fn convert(&self, val: impl IntoF32) -> f32 {
        self.min + val.into_f32() * self.span
    }

    pub fn reverse_convert(&self, val: impl IntoF32) -> f32 {
        (val.into_f32() - self.min) / self.span
    }
}

#[cfg(test)]
mod test {
    use fake::Fake;

    use crate::converter::Converter;

    #[test]
    fn test_converter() {
        for (min, max) in [
            (-5.0, 10.0),
            (-100000.0, 1000000.0),
            (0.00001, 0.00004),
            (100.0, 500.0),
            (0.0, 1.0),
            (5000.0, 2.0),
            (50543500.0, -1000.0),
        ] {
            let conv = Converter::new(min, max);

            let mut edited_conv = Converter::default();

            edited_conv.set_max(max);
            edited_conv.set_min(min);

            assert_eq!(conv.convert(0), min);
            assert_eq!(conv.convert(1), max);

            assert_eq!(conv.reverse_convert(min), 0.0);
            assert_eq!(conv.reverse_convert(max), 1.0);

            assert_eq!(edited_conv.convert(0), min);
            assert_eq!(edited_conv.convert(1), max);

            assert_eq!(edited_conv.reverse_convert(min), 0.0);
            assert_eq!(edited_conv.reverse_convert(max), 1.0);

            for _ in 0..100 {
                let val = (-5.0..5.0).fake::<f32>();
                assert_eq!(conv.convert(val), min + val * (max - min));
                assert_eq!(edited_conv.convert(val), min + val * (max - min));

                let converted = conv.convert(val);
                let rev = conv.reverse_convert(converted);

                assert!((rev - val).abs() < 0.000001);
            }
        }
    }
}
