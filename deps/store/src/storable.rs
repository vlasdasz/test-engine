use serde::{Serialize, de::DeserializeOwned};

pub trait Storable: Serialize + DeserializeOwned + Default {}

impl<T: Serialize + DeserializeOwned + Send + Default> Storable for T {}
