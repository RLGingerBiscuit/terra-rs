use std::{
    fs::File,
    io::{BufReader, Read, Write},
};

use anyhow::Result;
use byteorder::{ReadBytesExt, WriteBytesExt, LE};
use serde::{Deserialize, Serialize};

use crate::{
    io::{TerraReadExt, TerraWriteExt},
    Prefix,
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
            name: "".to_owned(),
            internal_name: "".to_owned(),
            max_stack: 0,
            stack: 0,
            prefix: Prefix::default(),
            sacrifices: 0,
            tooltip: "".to_owned(),
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

    fn legacy_lookup(version: i32, legacy_name: &str) -> String {
        let legacy_name = legacy_name.into();

        if version <= 4 {
            if legacy_name == "Cobalt Helmet" {
                "Jungle Hat".to_owned()
            } else if legacy_name == "Cobalt Breastplate" {
                "Jungle Shirt".to_owned()
            } else if legacy_name == "Cobalt Greaves" {
                "Jungle Pants".to_owned()
            } else {
                legacy_name
            }
        } else if version <= 20 {
            if legacy_name == "Gills potion" {
                "Gills Potion".to_owned()
            } else if legacy_name == "Thorn Chakrum" {
                "Thorm Chakram".to_owned()
            } else if legacy_name == "Ball 'O Hurt" {
                "Ball O' Hurt".to_owned()
            } else {
                legacy_name
            }
        } else if version <= 41 && legacy_name == "Iron Chain" {
            "Chain".to_owned()
        } else if version <= 44 && legacy_name == "Orb of Light" {
            "Shadow Orb".to_owned()
        } else if version <= 46 {
            if legacy_name == "Black Dye" {
                "Black Thread".to_owned()
            } else if legacy_name == "Green Dye" {
                "Green Thread".to_owned()
            } else {
                legacy_name
            }
        } else {
            legacy_name
        }
    }

    fn reverse_legacy_lookup(version: i32, name: &str) -> String {
        if version <= 4 {
            if name == "Jungle Hat" {
                "Cobalt Helmet".to_owned()
            } else if name == "Jungle Shirt" {
                "Cobalt Breastplate".to_owned()
            } else if name == "Jungle Pants" {
                "Cobalt Greaves".to_owned()
            } else {
                name.to_owned()
            }
        } else if version <= 20 {
            if name == "Gills Potion" {
                "Gills potion".to_owned()
            } else if name == "Thorn Chakram" {
                "Thork Chakrum".to_owned()
            } else if name == "Ball O' Hurt" {
                "Ball 'O Hurt".to_owned()
            } else {
                name.to_owned()
            }
        } else if version <= 41 && name == "Chain" {
            "Iron Chain".to_owned()
        } else if version <= 44 && name == "Shadow Orb" {
            "Orb of Light".to_owned()
        } else if version <= 46 {
            if name == "Black Thread" {
                "Black Dye".to_owned()
            } else if name == "Green Thread" {
                "Green Dye".to_owned()
            } else {
                name.to_owned()
            }
        } else {
            name.to_owned()
        }
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

        let mut internal_name_string = "".to_owned();

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

        if (id && self.id != 0) || (internal_name && internal_name_string.len() != 0) {
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
                    self.internal_name = "Unknown".to_owned()
                }

                self.name = "Unknown".to_owned();
            }

            if self.stack == 0 {
                self.stack = 1
            }
        }

        Ok(())
    }

    pub fn load_legacy_name(
        &mut self,
        reader: &mut dyn Read,
        items: &Vec<Self>,
        version: i32,
        stack: bool,
    ) -> Result<()> {
        let legacy_name = reader.read_lpstring()?;
        let name = Self::legacy_lookup(version, &legacy_name);

        if stack {
            self.stack = reader.read_i32::<LE>()?
        }

        if name == "" {
            self.id = 0;
            self.name = name;
            self.internal_name = "Air".to_owned();
        } else if let Some(item) = items.iter().filter(|i| i.name == name).next() {
            self.copy(item);
            if self.stack == 0 {
                self.stack = 1;
            }
        } else {
            self.id = 0;
            self.name = "Unknown".to_owned();
            self.stack = 0;
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

    pub fn save_legacy_name(
        &self,
        writer: &mut dyn Write,
        version: i32,
        stack: bool,
    ) -> Result<()> {
        let name = Self::reverse_legacy_lookup(version, &self.name);

        writer.write_lpstring(&name)?;

        if stack {
            writer.write_i32::<LE>(self.stack)?;
        }

        Ok(())
    }
}
