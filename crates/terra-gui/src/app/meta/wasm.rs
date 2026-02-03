use terra_core::{meta::Meta, BuffMeta, ItemMeta, PrefixMeta};

pub(crate) struct WasmMetaLoader;

impl WasmMetaLoader {
    pub(crate) fn new() -> Self {
        Self {}
    }

    fn load<T: Meta + serde::de::DeserializeOwned>(&self, name: &str) -> anyhow::Result<Vec<T>> {
        todo!()
    }
}

impl super::MetaLoader for WasmMetaLoader {
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
