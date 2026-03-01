use crate::SharedString;

pub trait Meta
where
    Self: Sized + 'static,
{
    type Id: Ord;

    fn id(&self) -> Self::Id;
    fn name(&self) -> SharedString;
    fn internal_name(&self) -> SharedString;

    fn default_ref() -> &'static Self;

    fn get(meta: &[Self], id: Self::Id) -> Option<&Self> {
        meta.iter().find(|m| m.id() == id)
    }

    fn get_or_default(meta: &[Self], id: Self::Id) -> &Self {
        meta.iter()
            .find(|m| m.id() == id)
            .unwrap_or(meta.first().unwrap_or(Self::default_ref()))
    }

    fn get_by_name<'a>(meta: &'a [Self], name: &'a str) -> Option<&'a Self> {
        meta.iter().find(|m| m.name() == name)
    }

    fn get_by_internal_name<'a>(meta: &'a [Self], internal_name: &'a str) -> Option<&'a Self> {
        meta.iter().find(|m| m.internal_name() == internal_name)
    }
}
