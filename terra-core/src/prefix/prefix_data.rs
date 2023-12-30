use std::io::{Read, Write};

use anyhow::Result;
use byteorder::{ReadBytesExt, WriteBytesExt};

#[derive(Clone, Default, Debug)]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct Prefix {
    pub id: u8,
}

impl Prefix {
    pub fn load(&mut self, reader: &mut dyn Read) -> Result<()> {
        self.id = reader.read_u8()?;

        Ok(())
    }

    pub fn skip(reader: &mut dyn Read) -> Result<()> {
        let _ = reader.read_u8()?;

        Ok(())
    }

    pub fn save(&self, writer: &mut dyn Write) -> Result<()> {
        writer.write_u8(self.id)?;

        Ok(())
    }
}
