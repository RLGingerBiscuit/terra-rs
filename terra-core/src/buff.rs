mod buff_data;
mod buff_meta;

use std::fmt::Display;

pub use buff_data::Buff;
pub use buff_meta::BuffMeta;

#[repr(u8)]
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
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
