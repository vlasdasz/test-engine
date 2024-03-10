use serde::{de::DeserializeOwned, Serialize};

pub trait Storable: Serialize + DeserializeOwned + Send + Default {}

impl<T: Serialize + DeserializeOwned + Send + Default> Storable for T {}
