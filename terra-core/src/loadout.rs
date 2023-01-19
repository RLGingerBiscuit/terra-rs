use std::io::{Read, Write};

use anyhow::Result;
use byteorder::{ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};

use crate::{
    bool_byte::BoolByte,
    io_extensions::{TerraReadExt, TerraWriteExt},
    item::Item,
    prefix::Prefix,
    utils, ACCESSORY_COUNT, ARMOR_COUNT, HIDDEN_VISUAL_COUNT,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
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
        prefix: bool,
    ) -> Result<()> {
        let accessory_count = if version >= 124 { 7 } else { 5 };

        for armor in self.armor.iter_mut() {
            if version >= 38 {
                armor.load(reader, items, prefixes, true, false, stack, prefix, false)?;
            } else {
                armor.load_legacy_name(reader, items, version, stack)?;
            }
        }

        for i in 0..accessory_count {
            if version >= 38 {
                self.accessories[i]
                    .load(reader, items, prefixes, true, false, stack, prefix, false)?;
            } else {
                self.accessories[i].load_legacy_name(reader, items, version, stack)?;
            }
        }

        if version >= 6 {
            for vanity in self.vanity_armor.iter_mut() {
                if version >= 38 {
                    vanity.load(reader, items, prefixes, true, false, stack, prefix, false)?;
                } else {
                    vanity.load_legacy_name(reader, items, version, stack)?;
                }
            }
        }

        if version >= 81 {
            for i in 0..accessory_count {
                self.vanity_accessories[i]
                    .load(reader, items, prefixes, true, false, stack, prefix, false)?;
            }
        }

        if version >= 47 {
            for dye in self.armor_dyes.iter_mut() {
                dye.load(reader, items, prefixes, true, false, stack, prefix, false)?;
            }
        }

        if version >= 81 {
            for i in 0..accessory_count {
                self.accessory_dyes[i]
                    .load(reader, items, prefixes, true, false, stack, prefix, false)?;
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
            let mut bb = BoolByte::from(reader.read_u8()?);

            for i in 0u8..8 {
                self.hide_visual[i as usize] = bb.get(i)?;
            }

            if version >= 124 {
                bb = BoolByte::from(reader.read_u8()?);

                for i in 0u8..2 {
                    self.hide_visual[(i + 8) as usize] = bb.get(i)?;
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

    pub fn save(
        &self,
        writer: &mut dyn Write,
        version: i32,
        stack: bool,
        prefix: bool,
    ) -> Result<()> {
        let accessory_count = if version >= 124 { 7 } else { 5 };

        for armor in self.armor.iter() {
            if version >= 38 {
                armor.save(writer, true, false, stack, prefix, false)?;
            } else {
                armor.save_legacy_name(writer, version, stack)?;
            }
        }

        for i in 0..accessory_count {
            if version >= 38 {
                self.accessories[i].save(writer, true, false, stack, prefix, false)?;
            } else {
                self.accessories[i].save_legacy_name(writer, version, stack)?;
            }
        }

        if version >= 6 {
            for vanity in self.vanity_armor.iter() {
                if version >= 38 {
                    vanity.save(writer, true, false, stack, prefix, false)?;
                } else {
                    vanity.save_legacy_name(writer, version, stack)?;
                }
            }
        }

        if version >= 81 {
            for i in 0..accessory_count {
                self.vanity_accessories[i].save(writer, true, false, stack, prefix, false)?;
            }
        }

        if version >= 47 {
            for dye in self.armor_dyes.iter() {
                dye.save(writer, true, false, stack, prefix, false)?;
            }
        }

        if version >= 81 {
            for i in 0..accessory_count {
                self.accessory_dyes[i].save(writer, true, false, stack, prefix, false)?;
            }
        }

        Ok(())
    }

    pub fn save_visuals(
        &self,
        writer: &mut dyn Write,
        version: i32,
        bb_visuals: bool,
    ) -> Result<()> {
        if bb_visuals {
            let mut bb = BoolByte::default();

            for i in 0u8..8 {
                bb.set(i, self.hide_visual[i as usize])?;
            }

            writer.write_u8(u8::from(&bb))?;

            if version >= 124 {
                bb = BoolByte::default();

                for i in 0u8..2 {
                    bb.set(i, self.hide_visual[(i + 8) as usize])?;
                }

                writer.write_u8(u8::from(&bb))?;
            }
        } else {
            // We don't need to do version checking here since this only happens in 1.4.4+
            for v in self.hide_visual.iter() {
                writer.write_bool(*v)?;
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
