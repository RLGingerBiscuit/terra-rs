use crate::{meta::Meta, BuffType, SharedString};

#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serialize",
    serde_with::skip_serializing_none,
    derive(serde::Serialize)
)]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct BuffMeta {
    pub id: i32,
    pub name: SharedString,
    pub internal_name: SharedString,
    pub x: i32,
    pub y: i32,
    pub buff_type: BuffType,
    pub tooltip: Option<Vec<SharedString>>,
}

impl Meta for BuffMeta {
    type Id = i32;

    fn id(&self) -> Self::Id {
        self.id
    }

    fn name(&self) -> SharedString {
        self.name.clone()
    }

    fn internal_name(&self) -> SharedString {
        self.internal_name.clone()
    }
}
