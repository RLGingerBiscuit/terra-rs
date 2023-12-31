use std::io::{Read, Write};

use byteorder::{ReadBytesExt, WriteBytesExt, LE};

use crate::ext::{TerraReadExt, TerraWriteExt};

#[derive(Default, Debug, Clone)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct ResearchItem {
    pub internal_name: String,
    pub stack: i32,
}

impl ResearchItem {
    pub fn load(&mut self, reader: &mut dyn Read) -> anyhow::Result<()> {
        self.internal_name = reader.read_lpstring()?;
        self.stack = reader.read_i32::<LE>()?;

        Ok(())
    }

    pub fn load_new(reader: &mut dyn Read) -> anyhow::Result<Self> {
        let mut item = ResearchItem::default();
        item.load(reader)?;

        Ok(item)
    }

    pub fn skip(reader: &mut dyn Read) -> anyhow::Result<()> {
        let _ = reader.read_lpstring()?;
        _ = reader.read_i32::<LE>()?;

        Ok(())
    }

    pub fn save(&self, writer: &mut dyn Write) -> anyhow::Result<()> {
        writer.write_lpstring(&self.internal_name)?;
        writer.write_i32::<LE>(self.stack)?;

        Ok(())
    }
}
