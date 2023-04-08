use std::io::{Read, Write};

use anyhow::Result;
use byteorder::{ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
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
