use std::io::{Read, Write};

use byteorder::{ReadBytesExt, WriteBytesExt, LE};

#[derive(Default, Debug, Clone)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct Buff {
    pub id: i32,
    pub time: i32,
}

impl Buff {
    pub fn load(&mut self, reader: &mut dyn Read) -> anyhow::Result<()> {
        self.id = reader.read_i32::<LE>()?;
        self.time = reader.read_i32::<LE>()?;

        Ok(())
    }

    pub fn skip(reader: &mut dyn Read) -> anyhow::Result<()> {
        let _ = reader.read_i32::<LE>()?;
        let _ = reader.read_i32::<LE>()?;

        Ok(())
    }

    pub fn save(&self, writer: &mut dyn Write) -> anyhow::Result<()> {
        writer.write_i32::<LE>(self.id)?;
        writer.write_i32::<LE>(self.time)?;

        Ok(())
    }
}
