use std::{future::Future, sync::Arc};

use terra_core::{BuffMeta, ItemMeta, PrefixMeta};

#[cfg(target_arch = "wasm32")]
mod wasm;

#[cfg(target_arch = "wasm32")]
pub fn platform_loader() -> Arc<impl Loader> {
    Arc::new(wasm::WasmLoader::new())
}

#[cfg(not(target_arch = "wasm32"))]
mod native;

#[cfg(not(target_arch = "wasm32"))]
pub fn platform_loader() -> Arc<impl Loader> {
    Arc::new(native::NativeLoader::new())
}

#[cfg(target_arch = "wasm32")]
pub trait Loader
where
    Self: Send + Sync,
{
    fn load_prefixes(&self) -> impl Future<Output = anyhow::Result<Vec<PrefixMeta>>>;
    fn load_items(&self) -> impl Future<Output = anyhow::Result<Vec<ItemMeta>>>;
    fn load_buffs(&self) -> impl Future<Output = anyhow::Result<Vec<BuffMeta>>>;
    fn load_spritesheet(
        &self,
        name: &str,
    ) -> impl Future<Output = anyhow::Result<image::RgbaImage>>;
}

#[cfg(not(target_arch = "wasm32"))]
pub trait Loader
where
    Self: Send + Sync,
{
    fn load_prefixes(&self) -> impl Future<Output = anyhow::Result<Vec<PrefixMeta>>> + Send;
    fn load_items(&self) -> impl Future<Output = anyhow::Result<Vec<ItemMeta>>> + Send;
    fn load_buffs(&self) -> impl Future<Output = anyhow::Result<Vec<BuffMeta>>> + Send;
    fn load_spritesheet(
        &self,
        name: &str,
    ) -> impl Future<Output = anyhow::Result<image::RgbaImage>> + Send;
}
