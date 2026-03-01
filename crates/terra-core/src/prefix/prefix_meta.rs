use once_cell::sync::Lazy;

use crate::{meta::Meta, SharedString};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PrefixMeta {
    pub id: u8,
    pub name: SharedString,
    pub internal_name: SharedString,
}

impl Meta for PrefixMeta {
    type Id = u8;

    fn id(&self) -> Self::Id {
        self.id
    }

    fn name(&self) -> SharedString {
        self.name.clone()
    }

    fn internal_name(&self) -> SharedString {
        self.internal_name.clone()
    }

    fn default_ref() -> &'static Self {
        static DEFAULT: Lazy<PrefixMeta> = Lazy::new(PrefixMeta::default);
        &DEFAULT
    }
}

impl Default for PrefixMeta {
    fn default() -> Self {
        Self {
            id: 0,
            name: SharedString::new(""),
            internal_name: SharedString::new("None"),
        }
    }
}
