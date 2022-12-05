use std::io::{Read, Write};

use anyhow::Result;
use byteorder::{ReadBytesExt, WriteBytesExt};

use crate::{
    bool_byte::BoolByte,
    io_extensions::{TerraReadExt, TerraWriteExt},
    item::Item,
    prefix::Prefix,
    utils, ACCESSORY_COUNT, ARMOR_COUNT, HIDDEN_VISUAL_COUNT,
};

#[derive(Clone, Debug)]
pub struct Loadout {
    pub hide_visual: [bool; HIDDEN_VISUAL_COUNT],

    pub armor: [Item; ARMOR_COUNT],
    pub vanity_armor: [Item; ARMOR_COUNT],
    pub armor_dyes: [Item; ARMOR_COUNT],

    pub accessories: [Item; ACCESSORY_COUNT],
    pub vanity_accessories: [Item; ACCESSORY_COUNT],
    pub accessory_dyes: [Item; ACCESSORY_COUNT],
}

impl Default for Loadout {
    fn default() -> Self {
        Self {
            hide_visual: [false; HIDDEN_VISUAL_COUNT],

            armor: std::array::from_fn(|_| Item::default()),
            vanity_armor: std::array::from_fn(|_| Item::default()),
            armor_dyes: std::array::from_fn(|_| Item::default()),

            accessories: std::array::from_fn(|_| Item::default()),
            vanity_accessories: std::array::from_fn(|_| Item::default()),
            accessory_dyes: std::array::from_fn(|_| Item::default()),
        }
    }
}

impl Loadout {
    pub fn load(
        &mut self,
        reader: &mut dyn Read,
        prefixes: &Vec<Prefix>,
        items: &Vec<Item>,
        version: i32,
        stack: bool,
    ) -> Result<()> {
        let accessory_count = if version >= 124 { 7 } else { 5 };

        for armor in self.armor.iter_mut() {
            armor.load(reader, items, prefixes, true, false, stack, true, false)?;
        }

        for i in 0..accessory_count {
            self.accessories[i].load(reader, items, prefixes, true, false, stack, true, false)?;
        }

        for vanity in self.vanity_armor.iter_mut() {
            vanity.load(reader, items, prefixes, true, false, stack, true, false)?;
        }

        if version >= 38 {
            for i in 0..accessory_count {
                self.vanity_accessories[i]
                    .load(reader, items, prefixes, true, false, stack, true, false)?;
            }
        }

        if version >= 47 {
            for dye in self.armor_dyes.iter_mut() {
                dye.load(reader, items, prefixes, true, false, stack, true, false)?;
            }
        }

        if version >= 81 {
            for i in 0..accessory_count {
                self.accessory_dyes[i]
                    .load(reader, items, prefixes, true, false, stack, true, false)?;
            }
        }

        Ok(())
    }

    pub fn load_visuals(
        &mut self,
        reader: &mut dyn Read,
        version: i32,
        bb_visuals: bool,
    ) -> Result<()> {
        if bb_visuals {
            let bb1 = BoolByte::from(reader.read_u8()?);

            for i in 0u8..8 {
                self.hide_visual[i as usize] = bb1.get(i)?;
            }

            if version >= 124 {
                let bb2 = BoolByte::from(reader.read_u8()?);

                for i in 0u8..2 {
                    self.hide_visual[(i + 8) as usize] = bb2.get(i)?;
                }
            }
        } else {
            // We don't need to do version checking here since this only happens in 1.4.4+
            for v in self.hide_visual.iter_mut() {
                *v = reader.read_bool()?;
            }
        }

        Ok(())
    }

    pub fn save(&self, writer: &mut dyn Write, version: i32, stack: bool) -> Result<()> {
        let accessory_count = if version >= 124 { 7 } else { 5 };

        for armor in self.armor.iter() {
            armor.save(writer, true, false, stack, true, false)?;
        }

        for i in 0..accessory_count {
            self.accessories[i].save(writer, true, false, stack, true, false)?;
        }

        for vanity in self.vanity_armor.iter() {
            vanity.save(writer, true, false, stack, true, false)?;
        }

        if version >= 38 {
            for i in 0..accessory_count {
                self.vanity_accessories[i].save(writer, true, false, stack, true, false)?;
            }
        }

        if version >= 47 {
            for dye in self.armor_dyes.iter() {
                dye.save(writer, true, false, stack, true, false)?;
            }
        }

        if version >= 81 {
            for i in 0..accessory_count {
                self.accessory_dyes[i].save(writer, true, false, stack, true, false)?;
            }
        }

        Ok(())
    }

    pub fn save_visuals(
        &mut self,
        writer: &mut dyn Write,
        version: i32,
        bb_visuals: bool,
    ) -> Result<()> {
        if bb_visuals {
            let mut bb1 = BoolByte::default();

            for i in 0u8..8 {
                bb1.set(i, self.hide_visual[i as usize])?;
            }

            writer.write_u8(u8::from(bb1))?;

            if version >= 124 {
                let mut bb2 = BoolByte::default();

                for i in 0u8..2 {
                    bb2.set(i, self.hide_visual[i as usize])?;
                }

                writer.write_u8(u8::from(bb2))?;
            }
        } else {
            // We don't need to do version checking here since this only happens in 1.4.4+
            for v in self.hide_visual.iter() {
                writer.write_bool(v.to_owned())?;
            }
        }

        Ok(())
    }

    pub fn has_item(&self, id: i32) -> bool {
        utils::has_item(id, &self.armor)
            || utils::has_item(id, &self.accessories)
            || utils::has_item(id, &self.vanity_armor)
            || utils::has_item(id, &self.vanity_accessories)
            || utils::has_item(id, &self.armor_dyes)
            || utils::has_item(id, &self.accessory_dyes)
    }
}
