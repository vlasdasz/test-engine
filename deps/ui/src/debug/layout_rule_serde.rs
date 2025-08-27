use serde::ser::{Serialize, SerializeStruct, Serializer};

use crate::{Anchor, LayoutRule};

impl Serialize for LayoutRule {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer {
        // pub struct LayoutRule {
        //     pub side:   Anchor,
        //     pub tiling: Option<Tiling>,
        //     pub offset: f32,
        //
        //     #[educe(Debug(ignore))]
        //     pub anchor_view:  WeakView,
        //     #[educe(Debug(ignore))]
        //     pub anchor_view2: WeakView,
        //
        //     pub relative: bool,
        //     pub between:  bool,
        // }

        let mut state = serializer.serialize_struct("LayoutRule", 5)?;

        state.serialize_field("side", &self.side)?;
        state.serialize_field("tiling", &self.tiling)?;
        state.serialize_field("offset", &self.offset)?;

        state.serialize_field("relative", &self.relative)?;
        state.serialize_field("between", &self.between)?;

        // Note: We deliberately skip anchor_view and anchor_view2
        // since WeakView likely can't/shouldn't be serialized

        state.end()
    }
}
