use std::{fs::File, io::BufReader};

use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct PrefixMeta {
    pub id: u8,
    pub name: String,
    pub internal_name: String,
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

        let mut meta: Vec<Self> = serde_json::from_reader(reader)?;
        meta.sort_by(|a, b| a.id.cmp(&b.id));

        Ok(meta)
    }

    pub fn meta_from_id(prefix_meta: &[Self], id: u8) -> Option<&Self> {
        prefix_meta.iter().find(|i| i.id == id)
    }
}
