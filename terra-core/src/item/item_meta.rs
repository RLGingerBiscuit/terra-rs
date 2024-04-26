use crate::{meta::Meta, ItemRarity, SharedString};

#[repr(u8)]
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serialize", derive(serde_repr::Serialize_repr))]
#[cfg_attr(feature = "deserialize", derive(serde_repr::Deserialize_repr))]
// NOTE: Will there be any conflicts here?
pub enum ItemType {
    Tile,
    Wall,
    Ammo,
    Melee,
    Ranged,
    Magic,
    Summon,
    HeadArmor,
    BodyArmor,
    LegArmor,
    Accessory,
    Vanity,
    #[default]
    Other,
}

#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serialize",
    serde_with::skip_serializing_none,
    derive(serde::Serialize)
)]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct ItemMeta {
    pub id: i32,
    pub name: SharedString,
    pub internal_name: SharedString,
    pub width: i32,
    pub height: i32,
    pub x: i32,
    pub y: i32,
    pub max_stack: i32,
    pub sacrifices: i32,
    pub value: i32,
    pub rarity: ItemRarity,
    pub use_time: Option<i32>,
    pub damage: Option<i32>,
    pub crit_chance: Option<i32>,
    pub knockback: Option<f32>,
    pub defense: Option<i32>,
    pub use_ammo: Option<i32>,
    pub mana_cost: Option<i32>,
    pub heal_life: Option<i32>,
    pub heal_mana: Option<i32>,
    pub pickaxe_power: Option<i32>,
    pub axe_power: Option<i32>,
    pub hammer_power: Option<i32>,
    pub fishing_power: Option<i32>,
    pub fishing_bait: Option<i32>,
    pub range_boost: Option<i32>,
    pub tooltip: Option<Vec<SharedString>>,
    pub forbidden: Option<bool>,
    pub consumes_tile: Option<i32>,
    pub item_type: Option<ItemType>,
    pub is_material: Option<bool>,
    pub is_consumable: Option<bool>,
    pub is_quest_item: Option<bool>,
    pub is_expert: Option<bool>,
}

impl Meta for ItemMeta {
    type Id = i32;

    fn id(&self) -> Self::Id {
        self.id
    }

    fn name(&self) -> SharedString {
        self.name.clone()
    }

    fn internal_name(&self) -> SharedString {
        self.internal_name.clone()
    }
}
