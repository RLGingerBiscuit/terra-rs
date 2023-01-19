use std::io::{Read, Write};

use anyhow::Result;
use byteorder::{ReadBytesExt, WriteBytesExt, LE};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::{
    io::{TerraReadExt, TerraWriteExt},
    Difficulty,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct JourneyPowers {
    pub godmode: bool,
    pub far_placement: bool,
    pub spawnrate: f32,
}

#[repr(u16)]
#[derive(Clone, Debug, Serialize_repr, Deserialize_repr)]
pub enum JourneyPowerId {
    Godmode = 5,
    FarPlacement = 11,
    Spawnrate = 14,
    Unknown = u16::MAX,
}

impl From<u16> for JourneyPowerId {
    fn from(value: u16) -> Self {
        match value {
            5 => Self::Godmode,
            11 => Self::FarPlacement,
            14 => Self::Spawnrate,
            _ => Self::Unknown,
        }
    }
}

impl From<JourneyPowerId> for u16 {
    fn from(value: JourneyPowerId) -> Self {
        match value {
            JourneyPowerId::Godmode => 5,
            JourneyPowerId::FarPlacement => 11,
            JourneyPowerId::Spawnrate => 14,
            JourneyPowerId::Unknown => u16::MAX,
        }
    }
}

impl Default for JourneyPowers {
    fn default() -> Self {
        Self {
            godmode: false,
            far_placement: true,
            spawnrate: 0.5,
        }
    }
}

impl JourneyPowers {
    pub fn load(&mut self, reader: &mut dyn Read) -> Result<()> {
        while reader.read_bool()? {
            match JourneyPowerId::from(reader.read_u16::<LE>()?) {
                JourneyPowerId::Godmode => self.godmode = reader.read_bool()?,
                JourneyPowerId::FarPlacement => self.far_placement = reader.read_bool()?,
                JourneyPowerId::Spawnrate => self.spawnrate = reader.read_f32::<LE>()?,
                _ => {}
            }
        }

        Ok(())
    }

    pub fn save(&self, writer: &mut dyn Write, difficulty: &Difficulty) -> Result<()> {
        if difficulty == &Difficulty::Journey {
            writer.write_bool(true)?;
            writer.write_u16::<LE>(u16::from(JourneyPowerId::Godmode))?;
            writer.write_bool(self.godmode)?;
            writer.write_bool(true)?;
            writer.write_u16::<LE>(u16::from(JourneyPowerId::FarPlacement))?;
            writer.write_bool(self.far_placement)?;
            writer.write_bool(true)?;
            writer.write_u16::<LE>(u16::from(JourneyPowerId::Spawnrate))?;
            writer.write_f32::<LE>(self.spawnrate)?;
        } else {
            // Terrasavr just writes a 0x00 in this case, but this is how Terraria itself does it
            let default = Self::default();

            writer.write_bool(true)?;
            writer.write_u16::<LE>(u16::from(JourneyPowerId::Godmode))?;
            writer.write_bool(default.godmode)?;
            writer.write_bool(true)?;
            writer.write_u16::<LE>(u16::from(JourneyPowerId::FarPlacement))?;
            writer.write_bool(default.far_placement)?;
            writer.write_bool(true)?;
            writer.write_u16::<LE>(u16::from(JourneyPowerId::Spawnrate))?;
            writer.write_f32::<LE>(default.spawnrate)?;
        }

        writer.write_bool(false)?;

        Ok(())
    }
}
