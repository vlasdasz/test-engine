#![allow(incomplete_features)]
#![allow(improper_ctypes_definitions)]
#![feature(specialization)]
#![feature(trait_upcasting)]
#![feature(arbitrary_self_types)]
#![feature(const_trait_impl)]

pub mod benchmark;
mod test_game;

#[cfg(mobile)]
#[no_mangle]
extern "C" fn make_app(
    ppi: std::os::raw::c_int,
    scale: std::os::raw::c_float,
    refresh_rate: std::os::raw::c_int,
    resolution_x: std::os::raw::c_int,
    resolution_y: std::os::raw::c_int,
    width: std::os::raw::c_float,
    height: std::os::raw::c_float,
    diagonal: std::os::raw::c_float,
) -> Box<dyn test_engine::App> {
    use test_engine::MakeApp;

    crate::test_game::TestApp::make_app(
        ppi,
        scale,
        refresh_rate,
        resolution_x,
        resolution_y,
        width,
        height,
        diagonal,
    )
}
