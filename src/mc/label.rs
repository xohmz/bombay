use serde_repr::{Deserialize_repr, Serialize_repr};

/// Variants of Monstercat brands.
#[derive(Clone, Debug, PartialEq, Serialize_repr, Deserialize_repr)]
#[serde(rename_all = "PascalCase")]
#[repr(u8)]
pub enum Brand {
    Uncaged = 1,
    Instinct = 2,
    CallofTheWild = 3,
    Silk = 4,
    MonstercatSilkShowcase = 5,
}
