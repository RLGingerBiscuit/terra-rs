use std::io::{Read, Write};

use byteorder::{ReadBytesExt, WriteBytesExt};

#[derive(Clone, Default, Debug)]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
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
