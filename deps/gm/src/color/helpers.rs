pub(super) fn linear_to_srgb(u: f32) -> f32 {
    if u <= 0.0031_308 {
        12.92 * u
    } else {
        1.055 * f32::powf(u, 0.416_666) - 0.055
    }
}

pub(super) fn srgb_to_linear(u: f32) -> f32 {
    if u <= 0.0404_5 {
        u / 12.92
    } else {
        f32::powf((u + 0.055) / 1.055, 2.4)
    }
}
