use std::{fs::File, io::BufReader};

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::ItemRarity;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ItemMeta {
    pub id: i32,
    pub name: String,
    pub internal_name: String,
    pub width: i32,
    pub height: i32,
    pub x: i32,
    pub y: i32,
    pub max_stack: i32,
    pub sacrifices: i32,
    pub value: i32,
    pub rarity: ItemRarity,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_time: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub damage: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub knockback: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defense: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_ammo: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mana_cost: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heal_life: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heal_mana: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pickaxe_power: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub axe_power: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hammer_power: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fishing_power: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fishing_bait: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range_boost: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tooltip: Option<Vec<String>>,
    pub consumable: bool,
    pub expert: bool,
}

impl Default for ItemMeta {
    fn default() -> Self {
        Self {
            id: 0,
            name: String::new(),
            internal_name: String::new(),
            width: 0,
            height: 0,
            x: 0,
            y: 0,
            max_stack: 0,
            sacrifices: 0,
            value: 0,
            rarity: ItemRarity::White,
            use_time: None,
            damage: None,
            crit: None,
            knockback: None,
            defense: None,
            use_ammo: None,
            mana_cost: None,
            heal_life: None,
            heal_mana: None,
            pickaxe_power: None,
            axe_power: None,
            hammer_power: None,
            fishing_power: None,
            fishing_bait: None,
            range_boost: None,
            tooltip: None,
            consumable: false,
            expert: false,
        }
    }
}

impl ItemMeta {
    pub fn load() -> Result<Vec<Self>> {
        let file = File::open(
            std::env::current_exe()?
                .parent()
                .unwrap()
                .join("resources")
                .join("items.json"),
        )?;

        let reader = BufReader::new(file);

        let meta: Vec<Self> = serde_json::from_reader(reader)?;

        Ok(meta)
    }
}
