use std::fmt::Display;

#[repr(u8)]
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serialize", derive(serde_repr::Serialize_repr))]
#[cfg_attr(feature = "deserialize", derive(serde_repr::Deserialize_repr))]
pub enum Team {
    #[default]
    None = 0,
    Red = 1,
    Green = 2,
    Blue = 3,
    Yellow = 4,
    Pink = 5,
    Unknown = 255,
}

impl From<u8> for Team {
    fn from(value: u8) -> Self {
        match value {
            0 => Team::None,
            1 => Team::Red,
            2 => Team::Green,
            3 => Team::Blue,
            4 => Team::Yellow,
            5 => Team::Pink,
            _ => Team::Unknown,
        }
    }
}

impl From<&u8> for Team {
    fn from(value: &u8) -> Self {
        Team::from(*value)
    }
}

impl From<Team> for u8 {
    fn from(value: Team) -> Self {
        value as u8
    }
}

impl From<&Team> for u8 {
    fn from(value: &Team) -> Self {
        u8::from(*value)
    }
}

impl PartialEq<u8> for Team {
    fn eq(&self, other: &u8) -> bool {
        u8::from(self) == *other
    }
}

impl PartialEq<Team> for u8 {
    fn eq(&self, other: &Team) -> bool {
        *self == u8::from(other)
    }
}

impl Display for Team {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Team::None => "None",
                Team::Red => "Red",
                Team::Green => "Green",
                Team::Blue => "Blue",
                Team::Yellow => "Yellow",
                Team::Pink => "Pink",
                Team::Unknown => "Unknown",
            }
        )
    }
}
