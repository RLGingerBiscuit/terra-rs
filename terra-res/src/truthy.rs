pub trait Truthy {
    fn truthy(&self) -> bool;
}

impl Truthy for bool {
    fn truthy(&self) -> bool {
        *self
    }
}

impl Truthy for i32 {
    fn truthy(&self) -> bool {
        *self != 0
    }
}

impl Truthy for f32 {
    fn truthy(&self) -> bool {
        *self != 0.
    }
}

pub trait TruthyOption<T> {
    fn truthy_option(self) -> Option<T>;
}

impl<T> TruthyOption<T> for Option<T>
where
    T: Truthy,
{
    fn truthy_option(self) -> Option<T> {
        if self.as_ref().is_some_and(|v| v.truthy()) {
            self
        } else {
            None
        }
    }
}

impl<T> TruthyOption<T> for mlua::Result<T>
where
    T: Truthy,
{
    fn truthy_option(self) -> Option<T> {
        if self.as_ref().is_ok_and(|v| v.truthy()) {
            self.ok()
        } else {
            None
        }
    }
}
