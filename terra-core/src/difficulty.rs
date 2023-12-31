use std::fmt::Display;

#[repr(u8)]
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub enum Difficulty {
    #[default]
    Classic = 0,
    Mediumcore = 1,
    Hardcore = 2,
    Journey = 3,
    Unknown = 255,
}

impl From<u8> for Difficulty {
    fn from(value: u8) -> Self {
        match value {
            0 => Difficulty::Classic,
            1 => Difficulty::Mediumcore,
            2 => Difficulty::Hardcore,
            3 => Difficulty::Journey,
            _ => Difficulty::Unknown,
        }
    }
}

impl From<&u8> for Difficulty {
    fn from(value: &u8) -> Self {
        Difficulty::from(*value)
    }
}

impl From<Difficulty> for u8 {
    fn from(value: Difficulty) -> Self {
        match value {
            Difficulty::Classic => 0,
            Difficulty::Mediumcore => 1,
            Difficulty::Hardcore => 2,
            Difficulty::Journey => 3,
            Difficulty::Unknown => 255,
        }
    }
}

impl From<&Difficulty> for u8 {
    fn from(value: &Difficulty) -> Self {
        u8::from(*value)
    }
}

impl PartialEq<u8> for Difficulty {
    fn eq(&self, other: &u8) -> bool {
        u8::from(self) == *other
    }
}

impl PartialEq<Difficulty> for u8 {
    fn eq(&self, other: &Difficulty) -> bool {
        *self == u8::from(other)
    }
}

impl Display for Difficulty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Difficulty::Classic => "Classic",
                Difficulty::Mediumcore => "Mediumcore",
                Difficulty::Hardcore => "Hardcore",
                Difficulty::Journey => "Journey",
                Difficulty::Unknown => "Unknown",
            }
        )
    }
}
