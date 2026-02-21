#[cfg(not(any(feature = "2d", feature = "3d")))]
compile_error!("To use game crate you must enable one of '2d' or '3d' features.");

#[cfg(all(feature = "2d", feature = "3d"))]
compile_error!("Features '2d' and '3d' are mutually exclusive and cannot be enabled together.");

mod game;
mod object;
mod primitives;
mod shape;

pub use game::*;
pub use object::*;
pub use primitives::*;
pub use shape::*;
