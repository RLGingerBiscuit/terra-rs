// NOTE: This is for serde_repr
#![allow(non_camel_case_types)]

use std::ops::Shl;

#[repr(u8)]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
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
        u8::from(self.clone()) == *other
    }
}

impl PartialEq<FileType> for u8 {
    fn eq(&self, other: &FileType) -> bool {
        *self == u8::from(other.clone())
    }
}

impl From<FileType> for u64 {
    fn from(d: FileType) -> Self {
        u64::from(u8::from(d))
    }
}

impl Shl<u64> for FileType {
    type Output = u64;

    fn shl(self, rhs: u64) -> Self::Output {
        u64::from(self) << rhs
    }
}
