use std::io::{Read, Write};

use anyhow::Result;
use byteorder::{ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Prefix {
    pub id: u8,
}

impl Default for Prefix {
    fn default() -> Self {
        Self { id: 0 }
    }
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
