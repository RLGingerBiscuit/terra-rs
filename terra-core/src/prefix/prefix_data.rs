use std::io::{Read, Write};

use byteorder::{ReadBytesExt, WriteBytesExt};

#[derive(Default, Debug, Clone)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct Prefix {
    pub id: u8,
}

impl Prefix {
    pub fn load(&mut self, reader: &mut dyn Read) -> anyhow::Result<()> {
        self.id = reader.read_u8()?;

        Ok(())
    }

    pub fn skip(reader: &mut dyn Read) -> anyhow::Result<()> {
        let _ = reader.read_u8()?;

        Ok(())
    }

    pub fn save(&self, writer: &mut dyn Write) -> anyhow::Result<()> {
        writer.write_u8(self.id)?;

        Ok(())
    }
}
