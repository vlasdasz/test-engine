pub(super) fn srgb_to_linear(u: f32) -> f32 {
    if u <= 0.0404_5 {
        u / 12.92
    } else {
        f32::powf((u + 0.055) / 1.055, 2.4)
    }
}
