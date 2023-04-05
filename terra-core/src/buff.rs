use std::fmt::Display;

use serde_repr::{Deserialize_repr, Serialize_repr};

mod buff;
mod buff_meta;

pub use buff::Buff;
pub use buff_meta::BuffMeta;

#[repr(u8)]
#[derive(Clone, Debug, Serialize_repr, Deserialize_repr)]
pub enum BuffType {
    Buff = 0,
    Debuff = 1,
}

impl Display for BuffType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                BuffType::Buff => "Buff",
                BuffType::Debuff => "Debuff",
            }
        )
    }
}
