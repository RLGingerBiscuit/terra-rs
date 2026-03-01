use serde::de::DeserializeOwned;
use terra_core::{meta::Meta, BuffMeta, ItemMeta, PrefixMeta};

pub(super) struct WasmLoader;

impl WasmLoader {
    pub(super) fn new() -> Self {
        Self {}
    }

    async fn load_meta<T: Meta + serde::de::DeserializeOwned>(
        &self,
        name: &str,
    ) -> anyhow::Result<Vec<T>> {
        todo!()
    }

    async fn load_spritesheet(&self, name: &str) -> anyhow::Result<image::RgbaImage> {
        todo!()
    }
}

impl super::MetaLoader for WasmLoader {
    async fn load_prefixes(&self) -> anyhow::Result<Vec<PrefixMeta>> {
        self.load_meta("prefixes.json").await
    }

    async fn load_items(&self) -> anyhow::Result<Vec<ItemMeta>> {
        self.load_meta("items.json").await
    }

    async fn load_buffs(&self) -> anyhow::Result<Vec<BuffMeta>> {
        self.load_meta("buffs.json").await
    }

    async fn load_spritesheet(&self, name: &str) -> anyhow::Result<image::RgbaImage> {
        self.load_spritesheet(name).await
    }
}
