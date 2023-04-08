use std::io::{Read, Write};

use anyhow::Result;
use byteorder::{ReadBytesExt, WriteBytesExt, LE};
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct Buff {
    pub id: i32,
    pub time: i32,
}

impl Buff {
    pub fn load(&mut self, reader: &mut dyn Read) -> Result<()> {
        self.id = reader.read_i32::<LE>()?;
        self.time = reader.read_i32::<LE>()?;

        Ok(())
    }

    pub fn skip(reader: &mut dyn Read) -> Result<()> {
        let _ = reader.read_i32::<LE>()?;
        let _ = reader.read_i32::<LE>()?;

        Ok(())
    }

    pub fn save(&self, writer: &mut dyn Write) -> Result<()> {
        writer.write_i32::<LE>(self.id)?;
        writer.write_i32::<LE>(self.time)?;

        Ok(())
    }
}
