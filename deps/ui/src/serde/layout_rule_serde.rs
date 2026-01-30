use std::fmt;

use serde::{
    Deserialize, Deserializer, de,
    de::{MapAccess, Visitor},
    ser::{Serialize, SerializeStruct, Serializer},
};

use crate::{
    LayoutRule,
    serde::weak_serde::{deserialize_weak, serialize_weak},
};

impl Serialize for LayoutRule {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer {
        let mut state = serializer.serialize_struct("LayoutRule", 7)?;

        state.serialize_field("side", &self.side)?;
        state.serialize_field("tiling", &self.tiling)?;
        state.serialize_field("offset", &self.offset)?;

        serialize_weak::<S>("anchor_view", self.anchor_view, &mut state)?;
        serialize_weak::<S>("anchor_view2", self.anchor_view2, &mut state)?;

        state.serialize_field("relative", &self.relative)?;
        state.serialize_field("between", &self.between)?;
        state.serialize_field("same", &self.same)?;
        state.serialize_field("enabled", &self.enabled)?;

        state.end()
    }
}

impl<'de> Deserialize<'de> for LayoutRule {
    #[allow(clippy::too_many_lines)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: Deserializer<'de> {
        enum Field {
            Side,
            Tiling,
            Offset,
            AnchorView,
            AnchorView2,
            Relative,
            Between,
            Same,
            Enabled,
        }

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
            where D: Deserializer<'de> {
                struct FieldVisitor;

                impl Visitor<'_> for FieldVisitor {
                    type Value = Field;
                    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        f.write_str("field identifier")
                    }
                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where E: de::Error {
                        match value {
                            "side" => Ok(Field::Side),
                            "tiling" => Ok(Field::Tiling),
                            "offset" => Ok(Field::Offset),
                            "anchor_view" => Ok(Field::AnchorView),
                            "anchor_view2" => Ok(Field::AnchorView2),
                            "relative" => Ok(Field::Relative),
                            "between" => Ok(Field::Between),
                            "same" => Ok(Field::Same),
                            "enabled" => Ok(Field::Enabled),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct LayoutRuleVisitor;

        impl<'de> Visitor<'de> for LayoutRuleVisitor {
            type Value = LayoutRule;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("struct LayoutRule")
            }

            fn visit_map<V>(self, mut map: V) -> Result<LayoutRule, V::Error>
            where V: MapAccess<'de> {
                let mut side = None;
                let mut tiling = None;
                let mut offset = None;
                let mut anchor_view = None;
                let mut anchor_view2 = None;
                let mut relative = None;
                let mut between = None;
                let mut same = None;
                let mut enabled = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Side => {
                            side = Some(map.next_value()?);
                        }
                        Field::Tiling => {
                            tiling = Some(map.next_value()?);
                        }
                        Field::Offset => {
                            offset = Some(map.next_value()?);
                        }
                        Field::AnchorView => {
                            anchor_view = Some(deserialize_weak(map.next_value()?));
                        }
                        Field::AnchorView2 => {
                            anchor_view2 = Some(deserialize_weak(map.next_value()?));
                        }
                        Field::Relative => {
                            relative = Some(map.next_value()?);
                        }
                        Field::Between => {
                            between = Some(map.next_value()?);
                        }
                        Field::Same => {
                            same = Some(map.next_value()?);
                        }
                        Field::Enabled => {
                            enabled = Some(map.next_value()?);
                        }
                    }
                }

                Ok(LayoutRule {
                    side:         side.ok_or_else(|| de::Error::missing_field("side"))?,
                    tiling:       tiling.unwrap_or(None),
                    offset:       offset.ok_or_else(|| de::Error::missing_field("offset"))?,
                    anchor_view:  anchor_view.ok_or_else(|| de::Error::missing_field("anchor_view"))?,
                    anchor_view2: anchor_view2.ok_or_else(|| de::Error::missing_field("anchor_view2"))?,
                    relative:     relative.ok_or_else(|| de::Error::missing_field("relative"))?,
                    between:      between.ok_or_else(|| de::Error::missing_field("between"))?,
                    same:         same.ok_or_else(|| de::Error::missing_field("same"))?,
                    enabled:      enabled.ok_or_else(|| de::Error::missing_field("enabled"))?,
                })
            }
        }

        const FIELDS: &[&str] = &[
            "side",
            "tiling",
            "offset",
            "anchor_view",
            "anchor_view2",
            "relative",
            "between",
            "same",
            "enabled",
        ];
        deserializer.deserialize_struct("LayoutRule", FIELDS, LayoutRuleVisitor)
    }
}
