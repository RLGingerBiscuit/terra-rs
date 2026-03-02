use eframe::wasm_bindgen::{JsCast, JsValue};
use image::ImageReader;
use terra_core::{meta::Meta, BuffMeta, ItemMeta, PrefixMeta};
use wasm_bindgen_futures::JsFuture;
use web_sys::{
    js_sys::{JsString, Uint8Array},
    Request, RequestInit,
};

pub(super) struct WasmLoader;

impl WasmLoader {
    pub(super) fn new() -> Self {
        Self {}
    }

    fn map_error(err: JsValue) -> anyhow::Error {
        anyhow::anyhow!(err
            .dyn_ref::<JsString>()
            .map(|s| s.into())
            .unwrap_or_else(|| "Unknown error".to_string()))
    }

    async fn load_meta<T: Meta + serde::de::DeserializeOwned>(
        &self,
        name: &str,
    ) -> anyhow::Result<Vec<T>> {
        let url = format!("/resources/{}", name);
        let opts = RequestInit::new();
        opts.set_method("GET");

        let get_json = async || -> Result<JsValue, JsValue> {
            let request = Request::new_with_str_and_init(&url, &opts)?;

            let window = web_sys::window().expect("no window?");
            let resp = JsFuture::from(window.fetch_with_request(&request)).await?;
            assert!(resp.is_instance_of::<web_sys::Response>());
            let resp = resp.dyn_into::<web_sys::Response>()?;
            JsFuture::from(resp.json()?).await
        };

        let json = get_json().await.map_err(Self::map_error)?;
        let meta: Vec<T> = serde_wasm_bindgen::from_value(json)?;
        Ok(meta)
    }

    async fn load_spritesheet(&self, name: &str) -> anyhow::Result<image::RgbaImage> {
        let url = format!("/resources/{}", name);
        let opts = RequestInit::new();
        opts.set_method("GET");

        let get_bytes = async || -> Result<Vec<u8>, JsValue> {
            let request = Request::new_with_str_and_init(&url, &opts)?;

            let window = web_sys::window().expect("no window?");
            let resp = JsFuture::from(window.fetch_with_request(&request)).await?;
            assert!(resp.is_instance_of::<web_sys::Response>());
            let resp = resp.dyn_into::<web_sys::Response>()?;
            let buf = JsFuture::from(resp.array_buffer()?).await?;
            Ok(Uint8Array::new(&buf).to_vec())
        };

        let data = get_bytes().await.map_err(Self::map_error)?;
        let cursor = std::io::Cursor::new(&data);

        Ok(ImageReader::new(cursor)
            .with_guessed_format()?
            .decode()?
            .into_rgba8())
    }
}

impl super::Loader for WasmLoader {
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
