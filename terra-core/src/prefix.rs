use std::{
    fs::File,
    io::{BufReader, Read, Write},
};

use anyhow::Result;
use byteorder::{ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Prefix {
    pub id: u8,
    pub name: String,
    pub internal_name: String,
}

impl Default for Prefix {
    fn default() -> Self {
        Self {
            id: 0,
            name: "".to_string(),
            internal_name: "".to_string(),
        }
    }
}

impl Prefix {
    pub fn load_prefixes() -> Result<Vec<Self>> {
        let prefixes_file = File::open(
            std::env::current_exe()?
                .parent()
                .unwrap()
                .join("resources")
                .join("prefixes.json"),
        )?;

        let prefixes_reader = BufReader::new(prefixes_file);

        let prefixes: Vec<Self> = serde_json::from_reader(prefixes_reader)?;

        Ok(prefixes)
    }

    fn copy(&mut self, prefix: &Self) {
        self.name = prefix.name.clone();
        self.internal_name = prefix.internal_name.clone();
    }

    pub fn load(&mut self, reader: &mut dyn Read, prefixes: &Vec<Self>) -> Result<()> {
        self.id = reader.read_u8()?;

        if self.id != 0 {
            if let Some(prefix) = prefixes.iter().filter(|p| p.id == self.id).next() {
                self.copy(prefix);
            } else {
                self.name = "Unknown".to_string();
                self.internal_name = "Unknown".to_string();
            }
        }

        Ok(())
    }

    pub fn save(&self, writer: &mut dyn Write) -> Result<()> {
        writer.write_u8(self.id)?;

        Ok(())
    }
}
