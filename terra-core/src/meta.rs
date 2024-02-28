pub trait Meta {
    type Id: PartialOrd;

    fn id(&self) -> Self::Id;
    fn name(&self) -> &str;
    fn internal_name(&self) -> &str;

    #[cfg(feature = "deserialize")]
    fn load() -> anyhow::Result<Vec<Self>>
    where
        Self: Sized;

    fn get(meta: &[Self], id: Self::Id) -> Option<&Self>
    where
        Self: Sized,
    {
        meta.iter().find(|m| m.id() == id)
    }

    fn get_or_default(meta: &[Self], id: Self::Id) -> &Self
    where
        Self: Sized,
    {
        meta.iter()
            .find(|m| m.id() == id)
            .unwrap_or(meta.get(0).expect("We really should have a zeroth meta"))
    }

    fn get_by_name<'a>(meta: &'a [Self], name: &'a str) -> Option<&'a Self>
    where
        Self: Sized,
    {
        meta.iter().find(|m| m.name() == name)
    }

    fn get_by_internal_name<'a>(meta: &'a [Self], internal_name: &'a str) -> Option<&'a Self>
    where
        Self: Sized,
    {
        meta.iter().find(|m| m.internal_name() == internal_name)
    }
}
