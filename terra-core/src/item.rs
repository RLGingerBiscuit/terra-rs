mod item_data;
mod item_meta;

use serde_repr::{Deserialize_repr, Serialize_repr};

pub use item_data::{Item, ItemError};
pub use item_meta::ItemMeta;

#[repr(i32)]
#[derive(Default, Clone, Debug, Serialize_repr, Deserialize_repr)]
pub enum ItemRarity {
    Gray = -1,
    #[default]
    White = 0,
    Blue = 1,
    Green = 2,
    Orange = 3,
    LightRed = 4,
    Pink = 5,
    LightPurple = 6,
    Lime = 7,
    Yellow = 8,
    Cyan = 9,
    Red = 10,
    Purple = 11,
    Expert = -12,
    Master = -13,
    Quest = -11,
    Unknown = i32::MAX,
}

impl From<i32> for ItemRarity {
    fn from(value: i32) -> Self {
        match value {
            -1 => ItemRarity::Gray,
            0 => ItemRarity::White,
            1 => ItemRarity::Blue,
            2 => ItemRarity::Green,
            3 => ItemRarity::Orange,
            4 => ItemRarity::LightRed,
            5 => ItemRarity::Pink,
            6 => ItemRarity::LightPurple,
            7 => ItemRarity::Lime,
            8 => ItemRarity::Yellow,
            9 => ItemRarity::Cyan,
            10 => ItemRarity::Red,
            11 => ItemRarity::Purple,
            -12 => ItemRarity::Expert,
            -13 => ItemRarity::Master,
            -11 => ItemRarity::Quest,
            _ => ItemRarity::Unknown,
        }
    }
}

impl From<ItemRarity> for i32 {
    fn from(value: ItemRarity) -> Self {
        match value {
            ItemRarity::Gray => -1,
            ItemRarity::White => 0,
            ItemRarity::Blue => 1,
            ItemRarity::Green => 2,
            ItemRarity::Orange => 3,
            ItemRarity::LightRed => 4,
            ItemRarity::Pink => 5,
            ItemRarity::LightPurple => 6,
            ItemRarity::Lime => 7,
            ItemRarity::Yellow => 8,
            ItemRarity::Cyan => 9,
            ItemRarity::Red => 10,
            ItemRarity::Purple => 11,
            ItemRarity::Expert => -12,
            ItemRarity::Master => -13,
            ItemRarity::Quest => -11,
            ItemRarity::Unknown => i32::MAX,
        }
    }
}
