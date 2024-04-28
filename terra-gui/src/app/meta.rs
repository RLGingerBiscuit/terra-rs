use std::rc::Rc;

use terra_core::{BuffMeta, ItemMeta, PrefixMeta};

cfg_if::cfg_if! {
    if #[cfg(target_arch = "wasm32")] {
        mod wasm;
        pub(crate) use wasm::*;
        pub fn platform_meta_loader() -> Rc<dyn MetaLoader> {
            Rc::new(WasmMetaLoader::new())
        }
    } else {
        mod native;
        pub(crate) use native::*;
        pub fn platform_meta_loader() -> Rc<dyn MetaLoader> {
            Rc::new(NativeMetaLoader::new())
        }
    }
}

pub trait MetaLoader {
    fn load_prefixes(&self) -> anyhow::Result<Vec<PrefixMeta>>;
    fn load_items(&self) -> anyhow::Result<Vec<ItemMeta>>;
    fn load_buffs(&self) -> anyhow::Result<Vec<BuffMeta>>;
}
