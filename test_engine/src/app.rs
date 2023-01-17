use std::ops::{Deref, DerefMut};

use crate::AppCore;

pub trait App: Deref<Target = AppCore> + DerefMut<Target = AppCore> {}
