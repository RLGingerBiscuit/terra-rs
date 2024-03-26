pub trait Meta {
    type Id: Ord;

    fn id(&self) -> Self::Id;
    fn name(&self) -> &str;
    fn internal_name(&self) -> &str;

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
            .unwrap_or(meta.first().expect("We really should have a zeroth meta"))
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
