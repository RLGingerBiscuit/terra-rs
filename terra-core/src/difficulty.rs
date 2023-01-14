use serde_repr::{Deserialize_repr, Serialize_repr};

#[repr(u8)]
#[derive(
    Default, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize_repr, Deserialize_repr,
)]
pub enum Difficulty {
    #[default]
    Classic = 0,
    Mediumcore = 1,
    Hardcore = 2,
    Journey = 3,
    Unknown = 255,
}

impl From<u8> for Difficulty {
    fn from(u: u8) -> Self {
        match u {
            0 => Difficulty::Classic,
            1 => Difficulty::Mediumcore,
            2 => Difficulty::Hardcore,
            3 => Difficulty::Journey,
            _ => Difficulty::Unknown,
        }
    }
}

impl From<Difficulty> for u8 {
    fn from(d: Difficulty) -> Self {
        match d {
            Difficulty::Classic => 0,
            Difficulty::Mediumcore => 1,
            Difficulty::Hardcore => 2,
            Difficulty::Journey => 3,
            Difficulty::Unknown => 255,
        }
    }
}

impl PartialEq<u8> for Difficulty {
    fn eq(&self, other: &u8) -> bool {
        &u8::from(self.clone()) == other
    }
}

impl PartialEq<Difficulty> for u8 {
    fn eq(&self, other: &Difficulty) -> bool {
        self == &u8::from(other.clone())
    }
}
