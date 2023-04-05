use std::{fs::File, io::BufReader};

use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ItemMeta {
    pub id: i32,
    pub name: String,
    pub internal_name: String,
    pub max_stack: i32,
    pub tooltip: Option<Vec<String>>,
    pub sacrifices: i32,
}

impl Default for ItemMeta {
    fn default() -> Self {
        Self {
            id: 0,
            name: String::new(),
            internal_name: String::new(),
            max_stack: 0,
            sacrifices: 0,
            tooltip: None,
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
