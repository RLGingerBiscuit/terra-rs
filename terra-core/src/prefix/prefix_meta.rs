use std::{fs::File, io::BufReader};

use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PrefixMeta {
    pub id: u8,
    pub name: String,
    pub internal_name: String,
}

impl Default for PrefixMeta {
    fn default() -> Self {
        Self {
            id: 0,
            name: String::new(),
            internal_name: String::new(),
        }
    }
}

impl PrefixMeta {
    pub fn load() -> Result<Vec<Self>> {
        let file = File::open(
            std::env::current_exe()?
                .parent()
                .unwrap()
                .join("resources")
                .join("prefixes.json"),
        )?;

        let reader = BufReader::new(file);

        let meta: Vec<Self> = serde_json::from_reader(reader)?;

        Ok(meta)
    }
}
