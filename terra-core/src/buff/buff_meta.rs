#[cfg(feature = "deserialize")]
use std::{fs::File, io::BufReader};

use crate::{meta::Meta, BuffType};

#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serialize",
    serde_with::skip_serializing_none,
    derive(serde::Serialize)
)]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct BuffMeta {
    pub id: i32,
    pub name: String,
    pub internal_name: String,
    pub x: i32,
    pub y: i32,
    pub buff_type: BuffType,
    pub tooltip: Option<Vec<String>>,
}

impl Meta for BuffMeta {
    type Id = i32;

    fn id(&self) -> Self::Id {
        self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn internal_name(&self) -> &str {
        &self.internal_name
    }

    #[cfg(feature = "deserialize")]
    fn load() -> anyhow::Result<Vec<Self>>
    where
        Self: Sized,
    {
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
}
