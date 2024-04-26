use std::{borrow::Borrow, fmt, ops::Deref, sync::Arc};

#[derive(Eq, PartialEq, PartialOrd, Ord, Hash, Clone)]
pub struct SharedString(Arc<str>);

impl SharedString {
    pub fn new(s: &str) -> Self {
        SharedString(Arc::from(s))
    }
}

impl Default for SharedString {
    fn default() -> Self {
        SharedString(Arc::from(""))
    }
}

impl Deref for SharedString {
    type Target = <Arc<str> as Deref>::Target;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<str> for SharedString {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Borrow<str> for SharedString {
    fn borrow(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for SharedString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

impl fmt::Display for SharedString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}
impl PartialEq<String> for SharedString {
    fn eq(&self, other: &String) -> bool {
        self.as_ref() == other
    }
}

impl PartialEq<SharedString> for String {
    fn eq(&self, other: &SharedString) -> bool {
        self == other.as_ref()
    }
}

impl PartialEq<str> for SharedString {
    fn eq(&self, other: &str) -> bool {
        self.as_ref() == other
    }
}

impl<'a> PartialEq<&'a str> for SharedString {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_ref() == *other
    }
}

impl From<SharedString> for Arc<str> {
    fn from(s: SharedString) -> Arc<str> {
        s.0
    }
}

impl From<Arc<str>> for SharedString {
    fn from(s: Arc<str>) -> SharedString {
        SharedString(s)
    }
}

impl From<SharedString> for String {
    fn from(s: SharedString) -> String {
        s.0.to_string()
    }
}

impl From<String> for SharedString {
    fn from(s: String) -> SharedString {
        SharedString(Arc::from(s))
    }
}

impl serde::Serialize for SharedString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_ref())
    }
}

impl<'de> serde::Deserialize<'de> for SharedString {
    fn deserialize<D>(deserializer: D) -> Result<SharedString, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(SharedString::from(s))
    }
}
