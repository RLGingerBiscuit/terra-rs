use std::{fs::File, io::BufReader, path::PathBuf};

use serde::de::DeserializeOwned;
use terra_core::{meta::Meta, BuffMeta, ItemMeta, PrefixMeta};

pub(super) struct NativeLoader {
    resources_path: PathBuf,
}

impl NativeLoader {
    pub(super) fn new() -> Self {
        let resources_path = std::env::current_exe()
            .expect("No current exe?")
            .parent()
            .expect("No parent?")
            .join("resources");

        Self { resources_path }
    }

    async fn load_meta<T: Meta + DeserializeOwned>(&self, name: &str) -> anyhow::Result<Vec<T>> {
        let file = File::open(self.resources_path.join(name))?;

        let reader = BufReader::new(file);

        let mut meta: Vec<T> = serde_json::from_reader(reader)?;
        meta.sort_by_key(|m| m.id());

        Ok(meta)
    }
}

impl super::Loader for NativeLoader {
    async fn load_prefixes(&self) -> anyhow::Result<Vec<PrefixMeta>> {
        self.load_meta("prefixes.json").await
    }

    async fn load_items(&self) -> anyhow::Result<Vec<ItemMeta>> {
        self.load_meta("items.json").await
    }

    async fn load_buffs(&self) -> anyhow::Result<Vec<BuffMeta>> {
        self.load_meta("buffs.json").await
    }
}
