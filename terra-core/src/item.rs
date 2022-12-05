use std::{
    fs::File,
    io::{BufReader, Read, Write},
};

use anyhow::Result;
use byteorder::{ReadBytesExt, WriteBytesExt, LE};
use serde::{Deserialize, Serialize};

use crate::{
    io_extensions::{TerraReadExt, TerraWriteExt},
    prefix::Prefix,
};

#[derive(thiserror::Error, Debug)]
pub enum ItemError {
    #[error("Either id or internal_name need to be true to correctly load/save an item.")]
    NoIdOrInternalName,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Item {
    pub id: i32,
    pub name: String,
    pub internal_name: String,
    pub max_stack: i32,
    #[serde(skip)]
    pub stack: i32,
    #[serde(skip)]
    pub prefix: Prefix,
    pub sacrifices: i32,
    pub tooltip: String,
    pub favourited: bool,
}

impl Default for Item {
    fn default() -> Self {
        Self {
            id: 0,
            name: "".to_string(),
            internal_name: "".to_string(),
            max_stack: 0,
            stack: 0,
            prefix: Prefix::default(),
            sacrifices: 0,
            tooltip: "".to_string(),
            favourited: false,
        }
    }
}

impl Item {
    pub fn load_items() -> Result<Vec<Self>> {
        let items_file = File::open(
            std::env::current_exe()?
                .parent()
                .unwrap()
                .join("resources")
                .join("items.json"),
        )?;

        let items_reader = BufReader::new(items_file);

        let items: Vec<Self> = serde_json::from_reader(items_reader)?;

        Ok(items)
    }

    fn copy(&mut self, item: &Self) {
        self.id = item.id.clone();
        self.internal_name = item.internal_name.clone();
        self.name = item.name.clone();
        self.max_stack = item.max_stack.clone();
        self.sacrifices = item.sacrifices.clone();
        self.tooltip = item.tooltip.clone();
    }

    pub fn load(
        &mut self,
        reader: &mut dyn Read,
        items: &Vec<Self>,
        prefixes: &Vec<Prefix>,
        id: bool,
        internal_name: bool,
        stack: bool,
        prefix: bool,
        favourited: bool,
    ) -> Result<()> {
        if !id && !internal_name {
            return Err(ItemError::NoIdOrInternalName.into());
        }

        let mut internal_name_string = "".to_string();

        if id {
            self.id = reader.read_i32::<LE>()?
        }
        if internal_name {
            internal_name_string = reader.read_lpstring()?;
        }
        if stack {
            self.stack = reader.read_i32::<LE>()?
        }
        if prefix {
            self.prefix.load(reader, prefixes)?
        }
        if favourited {
            self.favourited = reader.read_bool()?
        }

        if !((id && self.id != 0) || (internal_name && internal_name_string.len() == 0)) {
            if let Some(item) = items
                .iter()
                .filter(|i| {
                    (id && i.id == self.id)
                        || (internal_name && i.internal_name == internal_name_string)
                })
                .next()
            {
                self.copy(item)
            } else {
                if !id {
                    self.id = 0
                }
                if !internal_name {
                    self.internal_name = "Unknown".to_string()
                }
                self.name = "Unknown".to_string();
            }
        }

        if self.stack == 0 {
            self.stack = 1
        }

        Ok(())
    }

    pub fn load_new(
        reader: &mut dyn Read,
        items: &Vec<Self>,
        prefixes: &Vec<Prefix>,
        id: bool,
        internal_name: bool,
        stack: bool,
        prefix: bool,
        favourited: bool,
    ) -> Result<Self> {
        let mut item = Item::default();

        item.load(
            reader,
            items,
            prefixes,
            id,
            internal_name,
            stack,
            prefix,
            favourited,
        )?;

        return Ok(item);
    }

    pub fn save(
        &self,
        writer: &mut dyn Write,
        id: bool,
        internal_name: bool,
        stack: bool,
        prefix: bool,
        favourited: bool,
    ) -> Result<()> {
        if !id && !internal_name {
            return Err(ItemError::NoIdOrInternalName.into());
        }

        if id {
            writer.write_i32::<LE>(self.id)?
        }
        if internal_name {
            writer.write_lpstring(&self.internal_name)?
        }
        if stack {
            writer.write_i32::<LE>(self.stack)?
        }
        if prefix {
            self.prefix.save(writer)?
        }
        if favourited {
            writer.write_bool(self.favourited)?
        }

        Ok(())
    }
}
