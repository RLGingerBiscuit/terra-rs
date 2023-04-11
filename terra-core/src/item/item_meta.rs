use std::{fs::File, io::BufReader};

use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::ItemRarity;

#[skip_serializing_none]
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
    pub use_time: Option<i32>,
    pub damage: Option<i32>,
    pub crit: Option<i32>,
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
            forbidden: None,
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

        let mut meta: Vec<Self> = serde_json::from_reader(reader)?;
        meta.sort_by(|a, b| a.id.cmp(&b.id));

        Ok(meta)
    }

    pub fn meta_from_id(item_meta: &[Self], id: i32) -> Option<&Self> {
        item_meta.iter().find(|i| i.id == id)
    }

    pub fn meta_from_internal_name<'a>(
        item_meta: &'a [Self],
        internal_name: &str,
    ) -> Option<&'a Self> {
        item_meta.iter().find(|i| i.internal_name == internal_name)
    }

    pub fn meta_from_name<'a>(item_meta: &'a [Self], name: &str) -> Option<&'a Self> {
        item_meta.iter().find(|i| i.name == name)
    }
}
