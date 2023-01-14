use serde_repr::{Deserialize_repr, Serialize_repr};

#[repr(u8)]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize_repr, Deserialize_repr)]
pub enum FileType {
    Map = 1,
    World = 2,
    Player = 3,
    Unknown = 255,
}

impl From<u8> for FileType {
    fn from(u: u8) -> Self {
        match u {
            1 => FileType::Map,
            2 => FileType::World,
            3 => FileType::Player,
            _ => FileType::Unknown,
        }
    }
}

impl From<FileType> for u8 {
    fn from(d: FileType) -> Self {
        match d {
            FileType::Map => 1,
            FileType::World => 2,
            FileType::Player => 3,
            FileType::Unknown => 255,
        }
    }
}

impl PartialEq<u8> for FileType {
    fn eq(&self, other: &u8) -> bool {
        &u8::from(self.clone()) == other
    }
}

impl PartialEq<FileType> for u8 {
    fn eq(&self, other: &FileType) -> bool {
        self == &u8::from(other.clone())
    }
}
