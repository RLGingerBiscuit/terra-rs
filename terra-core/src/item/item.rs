use std::io::{Read, Write};

use anyhow::Result;
use byteorder::{ReadBytesExt, WriteBytesExt, LE};
use serde::{Deserialize, Serialize};

use crate::{
    ext::{TerraReadExt, TerraWriteExt},
    ItemMeta, Prefix,
};

#[derive(thiserror::Error, Debug)]
pub enum ItemError {
    #[error("Either id or internal_name need to be true to correctly load/save an item.")]
    OnlyIdOrInternalName,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Item {
    pub id: i32,
    pub stack: i32,
    pub prefix: Prefix,
    pub favourited: bool,
}

impl Default for Item {
    fn default() -> Self {
        Self {
            id: 0,
            stack: 0,
            prefix: Prefix::default(),
            favourited: false,
        }
    }
}

impl Item {
    fn legacy_lookup<'a>(version: i32, legacy_name: &'a str) -> &'a str {
        if version <= 4 {
            if legacy_name == "Cobalt Helmet" {
                "Jungle Hat"
            } else if legacy_name == "Cobalt Breastplate" {
                "Jungle Shirt"
            } else if legacy_name == "Cobalt Greaves" {
                "Jungle Pants"
            } else {
                legacy_name
            }
        } else if version <= 20 {
            if legacy_name == "Gills potion" {
                "Gills Potion"
            } else if legacy_name == "Thorn Chakrum" {
                "Thorm Chakram"
            } else if legacy_name == "Ball 'O Hurt" {
                "Ball O' Hurt"
            } else {
                legacy_name
            }
        } else if version <= 41 && legacy_name == "Iron Chain" {
            "Chain"
        } else if version <= 44 && legacy_name == "Orb of Light" {
            "Shadow Orb"
        } else if version <= 46 {
            if legacy_name == "Black Dye" {
                "Black Thread"
            } else if legacy_name == "Green Dye" {
                "Green Thread"
            } else {
                legacy_name
            }
        } else {
            legacy_name
        }
    }

    fn reverse_legacy_lookup<'a>(version: i32, name: &'a str) -> &'a str {
        if version <= 4 {
            if name == "Jungle Hat" {
                "Cobalt Helmet"
            } else if name == "Jungle Shirt" {
                "Cobalt Breastplate"
            } else if name == "Jungle Pants" {
                "Cobalt Greaves"
            } else {
                name
            }
        } else if version <= 20 {
            if name == "Gills Potion" {
                "Gills potion"
            } else if name == "Thorn Chakram" {
                "Thork Chakrum"
            } else if name == "Ball O' Hurt" {
                "Ball 'O Hurt"
            } else {
                name
            }
        } else if version <= 41 && name == "Chain" {
            "Iron Chain"
        } else if version <= 44 && name == "Shadow Orb" {
            "Orb of Light"
        } else if version <= 46 {
            if name == "Black Thread" {
                "Black Dye"
            } else if name == "Green Thread" {
                "Green Dye"
            } else {
                name
            }
        } else {
            name
        }
    }

    pub fn load(
        &mut self,
        reader: &mut dyn Read,
        item_meta: &Vec<ItemMeta>,
        id: bool,
        internal_name: bool,
        stack: bool,
        prefix: bool,
        favourited: bool,
    ) -> Result<()> {
        if !id && !internal_name || id && internal_name {
            return Err(ItemError::OnlyIdOrInternalName.into());
        }

        if id {
            self.id = reader.read_i32::<LE>()?;
        }
        if internal_name {
            let internal_name = reader.read_lpstring()?;

            if let Some(item) = ItemMeta::meta_from_internal_name(item_meta, &internal_name) {
                self.id = item.id;
            }
        }
        if stack {
            self.stack = reader.read_i32::<LE>()?;
        }
        if prefix {
            self.prefix.load(reader)?;
        }
        if favourited {
            self.favourited = reader.read_bool()?;
        }

        if self.id != 0 && self.stack == 0 {
            self.stack = 1
        }

        Ok(())
    }

    pub fn load_legacy_name(
        &mut self,
        reader: &mut dyn Read,
        item_meta: &Vec<ItemMeta>,
        version: i32,
        stack: bool,
    ) -> Result<()> {
        let legacy_name = reader.read_lpstring()?;
        let name = Self::legacy_lookup(version, &legacy_name);

        if stack {
            self.stack = reader.read_i32::<LE>()?
        }

        if name.len() != 0 {
            if let Some(item) = ItemMeta::meta_from_name(item_meta, &name) {
                self.id = item.id;
            }

            if self.stack == 0 {
                self.stack = 1;
            }
        }

        Ok(())
    }

    pub fn load_new(
        reader: &mut dyn Read,
        item_meta: &Vec<ItemMeta>,
        id: bool,
        internal_name: bool,
        stack: bool,
        prefix: bool,
        favourited: bool,
    ) -> Result<Self> {
        let mut item = Item::default();

        item.load(
            reader,
            item_meta,
            id,
            internal_name,
            stack,
            prefix,
            favourited,
        )?;

        Ok(item)
    }

    pub fn save(
        &self,
        writer: &mut dyn Write,
        item_meta: &Vec<ItemMeta>,
        id: bool,
        internal_name: bool,
        stack: bool,
        prefix: bool,
        favourited: bool,
    ) -> Result<()> {
        if !id && !internal_name || id && internal_name {
            return Err(ItemError::OnlyIdOrInternalName.into());
        }

        if id {
            writer.write_i32::<LE>(self.id)?;
        }
        if internal_name {
            if let Some(item) = ItemMeta::meta_from_id(item_meta, self.id) {
                writer.write_lpstring(&item.internal_name)?;
            } else {
                writer.write_lpstring(&String::new())?;
            }
        }
        if stack {
            writer.write_i32::<LE>(self.stack)?;
        }
        if prefix {
            self.prefix.save(writer)?;
        }
        if favourited {
            writer.write_bool(self.favourited)?;
        }

        Ok(())
    }

    pub fn save_legacy_name(
        &self,
        writer: &mut dyn Write,
        item_meta: &Vec<ItemMeta>,
        version: i32,
        stack: bool,
    ) -> Result<()> {
        if let Some(item) = ItemMeta::meta_from_id(item_meta, self.id) {
            let name = Self::reverse_legacy_lookup(version, &item.name);
            writer.write_lpstring(name)?;
        } else {
            writer.write_lpstring(&String::new())?;
        }

        if stack {
            writer.write_i32::<LE>(self.stack)?;
        }

        Ok(())
    }
}
