use std::{fs::File, io::BufReader};

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::BuffType;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BuffMeta {
    pub id: i32,
    pub name: String,
    pub internal_name: String,
    pub buff_type: BuffType,
    pub tooltip: Option<Vec<String>>,
}

impl Default for BuffMeta {
    fn default() -> Self {
        Self {
            id: 0,
            name: String::new(),
            internal_name: String::new(),
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

        let meta: Vec<Self> = serde_json::from_reader(reader)?;

        Ok(meta)
    }
}
