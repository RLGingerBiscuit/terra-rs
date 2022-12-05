use std::{fs::File, io::ErrorKind, path::PathBuf, str::FromStr};

use aesstream::AesReader;
use anyhow::Result;
use byteorder::{ReadBytesExt, LE};
use crypto::aessafe::AesSafe128Decryptor;

use crate::{
    bool_byte::BoolByte, buff::Buff, difficulty::Difficulty, io_extensions::TerraReadExt,
    item::Item, loadout::Loadout, prefix::Prefix, spawnpoint::Spawnpoint, Color, AMMO_COUNT,
    BANK_COUNT, BUFF_COUNT, BUILDER_ACCESSORY_COUNT, CELLPHONE_INFO_COUNT, COINS_COUNT,
    CURRENT_VERSION, DPAD_BINDINGS_COUNT, ENCRYPTION_BYTES, EQUIPMENT_COUNT, FEMALE_SKIN_VARIANTS,
    INVENTORY_COUNT, LOADOUT_COUNT, MAGIC_MASK, MAGIC_NUMBER, MALE_SKIN_VARIANTS, SPAWNPOINT_LIMIT,
    TEMPORARY_SLOT_COUNT, TICKS_PER_MICROSECOND,
};

// TODO: Seperate & implement these properly
#[derive(Default, Clone, Debug)]
pub struct JourneyPowerManager;

#[derive(thiserror::Error, Debug)]
pub enum PlayerError {
    #[error("Unknown error with file '{0}.")]
    Failure(String),
    #[error("The file '{0}' cannot be read by the user.")]
    AccessDenied(String),
    #[error("The file '{0}' was not found.")]
    FileNotFound(String),
    #[error("The file '{0}' is for a newer version of Terraria ({1}) than terra-rs supports (<= {CURRENT_VERSION}).")]
    PostDated(String, i32),
    #[error("The file '{0}' is corrupted.")]
    Corrupted(String),
    #[error("Expected Re-Logic file format in '{0}'.")]
    IncorrectFormat(String),
    #[error("Found incorrect file type in '{0}'.")]
    IncorrectFileType(String),
}

#[derive(Debug)]
pub struct Player {
    pub version: i32,
    pub revision: u32,
    pub favourited: u64,

    pub name: String,
    pub difficulty: Difficulty,

    pub playtime: i64,

    pub hair_style: i32,
    pub hair_dye: u8,

    pub hide_equipment: [bool; 5],

    pub male: bool,
    pub skin_variant: u8,

    pub life: i32,
    pub max_life: i32,
    pub mana: i32,
    pub max_mana: i32,

    pub demon_heart: bool,
    pub biome_torches: bool,
    pub biome_torches_enabled: bool,
    pub artisan_loaf: bool,
    pub vital_crystal: bool,
    pub aegis_fruit: bool,
    pub arcane_crystal: bool,
    pub galaxy_pearl: bool,
    pub gummy_worm: bool,
    pub ambrosia: bool,

    pub defeated_dd2: bool,

    pub tax_money: i32,

    pub pve_deaths: i32,
    pub pvp_deaths: i32,

    pub hair_color: Color,
    pub skin_color: Color,
    pub eye_color: Color,
    pub shirt_color: Color,
    pub undershirt_color: Color,
    pub pants_color: Color,
    pub shoe_color: Color,

    pub equipment: [Item; EQUIPMENT_COUNT],
    pub equipment_dyes: [Item; EQUIPMENT_COUNT],

    pub inventory: [Item; INVENTORY_COUNT],
    pub coins: [Item; COINS_COUNT],
    pub ammo: [Item; AMMO_COUNT],

    pub piggy_bank: [Item; BANK_COUNT],
    pub safe: [Item; BANK_COUNT],
    pub defenders_forge: [Item; BANK_COUNT],
    pub void_vault: [Item; BANK_COUNT],
    pub void_vault_enabled: bool,

    pub buffs: [Buff; BUFF_COUNT],

    pub spawnpoints: Vec<Spawnpoint>,

    pub locked_hotbar: bool,

    pub hide_cellphone_info: [bool; CELLPHONE_INFO_COUNT],

    pub angler_quests: i32,

    pub dpad_bindings: [i32; DPAD_BINDINGS_COUNT],

    pub builder_accessory_status: [i32; BUILDER_ACCESSORY_COUNT],

    pub tavernkeep_quests: i32,

    pub dead: bool,
    pub respawn_timer: i32,

    pub last_save: i64,

    pub golfer_score: i32,

    pub research: Vec<Item>,

    pub temporary_slots: [Item; TEMPORARY_SLOT_COUNT],

    pub journey_powers: JourneyPowerManager,

    pub super_cart: bool,
    pub super_cart_enabled: bool,

    pub current_loadout_index: i32,

    pub loadouts: [Loadout; LOADOUT_COUNT],
}

impl Default for Player {
    fn default() -> Self {
        Self {
            version: CURRENT_VERSION,
            revision: 0,
            favourited: 0,

            name: "Player".to_string(),
            difficulty: Difficulty::Classic,

            playtime: 0,

            hair_style: 0,
            hair_dye: 0,

            hide_equipment: [false; EQUIPMENT_COUNT],

            male: true,
            skin_variant: 0,

            life: 100,
            max_life: 100,
            mana: 20,
            max_mana: 20,

            demon_heart: false,
            biome_torches: false,
            biome_torches_enabled: false,
            artisan_loaf: false,
            vital_crystal: false,
            aegis_fruit: false,
            arcane_crystal: false,
            galaxy_pearl: false,
            gummy_worm: false,
            ambrosia: false,

            defeated_dd2: false,

            tax_money: 0,

            pve_deaths: 0,
            pvp_deaths: 0,

            hair_color: Color::from_str("#d75a37").unwrap(),
            skin_color: Color::from_str("#ff7d5a").unwrap(),
            eye_color: Color::from_str("#695a4b").unwrap(),
            shirt_color: Color::from_str("#afa58c").unwrap(),
            undershirt_color: Color::from_str("#a0b4d7").unwrap(),
            pants_color: Color::from_str("#ffe6af").unwrap(),
            shoe_color: Color::from_str("#a0693c").unwrap(),

            equipment: std::array::from_fn(|_| Item::default()),
            equipment_dyes: std::array::from_fn(|_| Item::default()),

            inventory: std::array::from_fn(|_| Item::default()),
            coins: std::array::from_fn(|_| Item::default()),
            ammo: std::array::from_fn(|_| Item::default()),

            piggy_bank: std::array::from_fn(|_| Item::default()),
            safe: std::array::from_fn(|_| Item::default()),
            defenders_forge: std::array::from_fn(|_| Item::default()),
            void_vault: std::array::from_fn(|_| Item::default()),
            void_vault_enabled: false,

            buffs: std::array::from_fn(|_| Buff::default()),

            spawnpoints: Vec::new(),

            locked_hotbar: false,

            hide_cellphone_info: [false; CELLPHONE_INFO_COUNT],

            angler_quests: 0,

            dpad_bindings: [-1; DPAD_BINDINGS_COUNT],

            builder_accessory_status: [0; BUILDER_ACCESSORY_COUNT],

            tavernkeep_quests: 0,

            dead: false,
            respawn_timer: 0,

            last_save: chrono::Utc::now().timestamp_micros() * TICKS_PER_MICROSECOND as i64,

            golfer_score: 0,

            research: Vec::new(),

            temporary_slots: std::array::from_fn(|_| Item::default()),

            journey_powers: JourneyPowerManager::default(),

            super_cart: false,
            super_cart_enabled: false,

            current_loadout_index: 0,

            loadouts: std::array::from_fn(|_| Loadout::default()),
        }
    }
}

impl Player {
    pub fn load(
        &mut self,
        filepath: impl Into<PathBuf>,
        prefixes: &Vec<Prefix>,
        items: &Vec<Item>,
        buffs: &Vec<Buff>,
    ) -> Result<()> {
        let filepath_buf: PathBuf = filepath.into();
        let filepath_string: String = filepath_buf.clone().into_os_string().into_string().unwrap();

        let file = match File::open(&filepath_buf) {
            Ok(f) => f,
            Err(e) => {
                return Err(match e.kind() {
                    ErrorKind::NotFound => PlayerError::FileNotFound(filepath_string),
                    ErrorKind::PermissionDenied => PlayerError::AccessDenied(filepath_string),
                    _ => PlayerError::Failure(filepath_string),
                }
                .into())
            }
        };

        let decryptor = AesSafe128Decryptor::new(ENCRYPTION_BYTES);
        let mut reader = match AesReader::new_with_iv(file, decryptor, ENCRYPTION_BYTES) {
            Ok(r) => r,
            Err(_) => return Err(PlayerError::Failure(filepath_string).into()),
        };

        self.version = reader.read_i32::<LE>()?;

        if self.version > CURRENT_VERSION {
            return Err(PlayerError::PostDated(filepath_string, self.version).into());
        }

        if self.version >= 135 {
            // The string "relogic", followed by a 1-byte filetype
            let magic_num = reader.read_u64::<LE>()?;

            // Both MAGIC_MASK and MAGIC_NUMBER were taken directly from Terraria's exe
            if magic_num & MAGIC_MASK != MAGIC_NUMBER {
                return Err(PlayerError::IncorrectFormat(filepath_string).into());
            }

            // The file type of the file
            //   None = 0
            //    Map = 1
            //  World = 2
            // Player = 3
            if (magic_num >> 56) & 255 != 3 {
                return Err(PlayerError::IncorrectFileType(filepath_string).into());
            }

            // This u32 is a 'revision' field, that is only used for type 1 files (Map)
            self.revision = reader.read_u32::<LE>()?;
            // This u64 is a 'favourited' field, which for Players, is handled by favourites.json
            self.favourited = reader.read_u64::<LE>()?;
        }

        // This method mimics C#'s BinaryReader.ReadString(),
        // prefixing the string with its length in ULEB128 format
        self.name = reader.read_lpstring()?;

        if self.version >= 10 {
            self.difficulty = Difficulty::from(reader.read_u8()?);
        } else if reader.read_bool()? {
            self.difficulty = Difficulty::Hardcore;
        }

        if self.version >= 138 {
            self.playtime = reader.read_i64::<LE>()?;
        }

        self.hair_style = reader.read_i32::<LE>()?;

        if self.version >= 72 {
            self.hair_dye = reader.read_u8()?;
        }

        self.loadouts[0].load_visuals(&mut reader, self.version, true)?;

        if self.version >= 119 {
            let bb1 = BoolByte::from(reader.read_u8()?);

            for i in 0..(EQUIPMENT_COUNT as u8) {
                self.hide_equipment[i as usize] = bb1.get(i)?;
            }
        }

        if self.version <= 17 {
            if FEMALE_SKIN_VARIANTS.contains(&self.hair_style) {
                self.male = false;
                self.skin_variant = 4;
            }
        } else if self.version <= 106 {
            self.male = reader.read_bool()?;
            if self.male {
                self.skin_variant = 4
            }
        } else {
            self.skin_variant = reader.read_u8()?;

            self.male = MALE_SKIN_VARIANTS.contains(&(self.skin_variant as i32));

            if self.version <= 160 && self.skin_variant == 7 {
                self.skin_variant = 9;
            }
        }

        self.life = reader.read_i32::<LE>()?;
        self.max_life = reader.read_i32::<LE>()?;
        self.mana = reader.read_i32::<LE>()?;
        self.max_mana = reader.read_i32::<LE>()?;

        if self.version >= 125 {
            self.demon_heart = reader.read_bool()?;

            if self.version >= 229 {
                self.biome_torches = reader.read_bool()?;
                self.biome_torches_enabled = reader.read_bool()?;

                if self.version >= 256 {
                    self.artisan_loaf = reader.read_bool()?;

                    if self.version >= 260 {
                        self.vital_crystal = reader.read_bool()?;
                        self.aegis_fruit = reader.read_bool()?;
                        self.arcane_crystal = reader.read_bool()?;
                        self.galaxy_pearl = reader.read_bool()?;
                        self.gummy_worm = reader.read_bool()?;
                        self.ambrosia = reader.read_bool()?;
                    }
                }
            }
        }

        if self.version >= 182 {
            self.defeated_dd2 = reader.read_bool()?;
        }

        if self.version >= 182 {
            self.tax_money = reader.read_i32::<LE>()?;
        }

        if self.version >= 256 {
            self.pve_deaths = reader.read_i32::<LE>()?;
            self.pvp_deaths = reader.read_i32::<LE>()?;
        }

        self.hair_color = reader.read_rgb()?;
        self.skin_color = reader.read_rgb()?;
        self.eye_color = reader.read_rgb()?;
        self.shirt_color = reader.read_rgb()?;
        self.undershirt_color = reader.read_rgb()?;
        self.pants_color = reader.read_rgb()?;
        self.shoe_color = reader.read_rgb()?;

        self.loadouts[0].load(&mut reader, prefixes, items, self.version, false)?;

        let inventory_count = if self.version >= 58 { 50 } else { 40 };

        for i in 0..inventory_count {
            self.inventory[i].load(
                &mut reader,
                items,
                prefixes,
                true,
                false,
                true,
                true,
                self.version >= 114,
            )?;
        }

        for i in 0..COINS_COUNT {
            self.coins[i].load(
                &mut reader,
                items,
                prefixes,
                true,
                false,
                true,
                true,
                self.version >= 114,
            )?;
        }

        for i in 0..AMMO_COUNT {
            self.ammo[i].load(
                &mut reader,
                items,
                prefixes,
                true,
                false,
                true,
                true,
                self.version >= 114,
            )?;
        }

        if self.version >= 117 {
            let start = if self.version >= 136 { 0 } else { 1 };

            for i in start..EQUIPMENT_COUNT {
                self.equipment[i].load(
                    &mut reader,
                    items,
                    prefixes,
                    true,
                    false,
                    false,
                    true,
                    false,
                )?;
                self.equipment_dyes[i].load(
                    &mut reader,
                    items,
                    prefixes,
                    true,
                    false,
                    false,
                    true,
                    false,
                )?;
            }
        }

        let bank_count = if self.version >= 58 { 40 } else { 20 };

        for i in 0..bank_count {
            self.piggy_bank[i].load(
                &mut reader,
                items,
                prefixes,
                true,
                false,
                true,
                true,
                false,
            )?;
        }

        if self.version >= 20 {
            for i in 0..bank_count {
                self.safe[i].load(&mut reader, items, prefixes, true, false, true, true, false)?;
            }
        }

        if self.version >= 182 {
            for i in 0..bank_count {
                self.defenders_forge[i].load(
                    &mut reader,
                    items,
                    prefixes,
                    true,
                    false,
                    true,
                    true,
                    false,
                )?;
            }
        }

        if self.version >= 198 {
            for i in 0..bank_count {
                self.void_vault[i].load(
                    &mut reader,
                    items,
                    prefixes,
                    true,
                    false,
                    true,
                    true,
                    self.version >= 255,
                )?;
            }

            if self.version >= 199 {
                let bb1 = BoolByte::from(reader.read_u8()?);
                self.void_vault_enabled = bb1.get(0)?;
            }
        }

        if self.version >= 11 {
            let buff_count = if self.version >= 252 {
                44
            } else if self.version >= 74 {
                22
            } else {
                10
            };

            for i in 0..buff_count {
                self.buffs[i].load(&mut reader, buffs)?;
            }
        }

        for _ in 0..SPAWNPOINT_LIMIT {
            let x = reader.read_i32::<LE>()?;
            if x == -1 {
                break;
            }

            let y = reader.read_i32::<LE>()?;
            let id = reader.read_i32::<LE>()?;
            let name = reader.read_lpstring()?;

            let spawnpoint = Spawnpoint { x, y, id, name };

            self.spawnpoints.push(spawnpoint);
        }

        todo!("Player.load()")
    }
    pub fn save(&self, filepath: impl Into<PathBuf>) -> Result<()> {
        todo!("Player.save()")
    }
}
