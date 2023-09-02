use std::{fs::File, io::BufReader};

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::meta::Meta;

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct PrefixMeta {
    pub id: u8,
    pub name: String,
    pub internal_name: String,
}

impl Meta for PrefixMeta {
    type Id = u8;

    fn id(&self) -> Self::Id {
        self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn internal_name(&self) -> &str {
        &self.internal_name
    }

    fn load() -> Result<Vec<Self>> {
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
}
