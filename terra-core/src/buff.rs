use std::{
    fmt::Display,
    fs::File,
    io::{BufReader, Read, Write},
};

use anyhow::Result;
use byteorder::{ReadBytesExt, WriteBytesExt, LE};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[repr(u8)]
#[derive(Clone, Debug, Serialize_repr, Deserialize_repr)]
pub enum BuffType {
    Buff = 0,
    Debuff = 1,
}

impl Display for BuffType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BuffType::Buff => write!(f, "Buff"),
            BuffType::Debuff => write!(f, "Debuff"),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Buff {
    pub id: i32,
    pub name: String,
    pub internal_name: String,
    #[serde(skip)]
    pub time: i32,
    pub buff_type: BuffType,
    pub tooltip: String,
}

impl Default for Buff {
    fn default() -> Self {
        Self {
            id: 0,
            name: "".to_string(),
            internal_name: "".to_string(),
            time: 0,
            buff_type: BuffType::Buff,
            tooltip: "".to_string(),
        }
    }
}

impl Buff {
    pub fn load_buffs() -> Result<Vec<Self>> {
        let buffs_file = File::open(
            std::env::current_exe()?
                .parent()
                .unwrap()
                .join("resources")
                .join("buffs.json"),
        )?;

        let buffs_reader = BufReader::new(buffs_file);

        let buffs: Vec<Self> = serde_json::from_reader(buffs_reader)?;

        Ok(buffs)
    }

    fn copy(&mut self, buff: &Self) {
        self.name = buff.name.clone();
        self.internal_name = buff.internal_name.clone();
        self.buff_type = buff.buff_type.clone();
        self.tooltip = buff.tooltip.clone();
    }

    pub fn load(&mut self, reader: &mut dyn Read, buffs: &Vec<Self>) -> Result<()> {
        self.id = reader.read_i32::<LE>()?;
        self.time = reader.read_i32::<LE>()?;

        if self.id != 0 {
            if let Some(buff) = buffs.iter().filter(|b| b.id == self.id).next() {
                self.copy(buff);
            } else {
                self.name = "Unknown".to_string();
                self.internal_name = "Unknown".to_string();
            }
        }

        Ok(())
    }

    pub fn save(&self, writer: &mut dyn Write) -> Result<()> {
        writer.write_i32::<LE>(self.id)?;
        writer.write_i32::<LE>(self.time)?;

        Ok(())
    }
}
