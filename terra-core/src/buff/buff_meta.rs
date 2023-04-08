use std::{fs::File, io::BufReader};

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::BuffType;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BuffMeta {
    pub id: i32,
    pub name: String,
    pub internal_name: String,
    pub x: i32,
    pub y: i32,
    pub buff_type: BuffType,
    pub tooltip: Option<Vec<String>>,
}

impl Default for BuffMeta {
    fn default() -> Self {
        Self {
            id: 0,
            name: String::new(),
            internal_name: String::new(),
            x: 0,
            y: 0,
            buff_type: BuffType::Buff,
            tooltip: None,
        }
    }
}

impl BuffMeta {
    pub fn load() -> Result<Vec<Self>> {
        let file = File::open(
            std::env::current_exe()?
                .parent()
                .unwrap()
                .join("resources")
                .join("buffs.json"),
        )?;

        let reader = BufReader::new(file);

        let mut meta: Vec<Self> = serde_json::from_reader(reader)?;
        meta.sort_by(|a, b| a.id.cmp(&b.id));

        Ok(meta)
    }

    pub fn meta_from_id(buff_meta: &[Self], id: i32) -> Option<&Self> {
        buff_meta.iter().find(|i| i.id == id)
    }
}
