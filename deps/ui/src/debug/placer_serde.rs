use serde::{Serialize, Serializer, ser::SerializeStruct};

use crate::Placer;

impl Serialize for Placer {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer {
        let mut state = serializer.serialize_struct("Placer", 5)?;

        state.serialize_field("rules", &self.rules)?;
        state.serialize_field("all_tiling_rules", &self.all_tiling_rules)?;

        // state.serialize_field("view", &self.view)?;
        // state.serialize_field("s_content", &self.s_content)?;

        state.serialize_field("has", &self.all_margin)?;
        state.serialize_field("has", &self.has)?;

        state.end()
    }
}
