use serde::{de::DeserializeOwned, Serialize};

pub trait Storable: Serialize + DeserializeOwned + Default {}

impl<T: Serialize + DeserializeOwned + Send + Default> Storable for T {}
