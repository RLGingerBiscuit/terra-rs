use std::{fs::File, io::BufReader, path::PathBuf};

use terra_core::{meta::Meta, BuffMeta, ItemMeta, PrefixMeta};

pub(crate) struct NativeMetaLoader {
    resources_path: PathBuf,
}

impl NativeMetaLoader {
    pub(crate) fn new() -> Self {
        let resources_path = std::env::current_exe()
            .expect("No current exe?")
            .parent()
            .expect("No parent?")
            .join("resources");

        Self { resources_path }
    }

    fn load<T: Meta + serde::de::DeserializeOwned>(&self, name: &str) -> anyhow::Result<Vec<T>> {
        let file = File::open(self.resources_path.join(name))?;

        let reader = BufReader::new(file);

        let mut meta: Vec<T> = serde_json::from_reader(reader)?;
        meta.sort_by_key(|m| m.id());

        Ok(meta)
    }
}

impl super::MetaLoader for NativeMetaLoader {
    fn load_prefixes(&self) -> anyhow::Result<Vec<PrefixMeta>> {
        self.load("prefixes.json")
    }

    fn load_items(&self) -> anyhow::Result<Vec<ItemMeta>> {
        self.load("items.json")
    }

    fn load_buffs(&self) -> anyhow::Result<Vec<BuffMeta>> {
        self.load("buffs.json")
    }
}
