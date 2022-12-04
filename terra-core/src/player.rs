#![allow(unused)]

use std::str::FromStr;

use crate::{
    buff::Buff, item::Item, Color, AMMO_COUNT, ARMOR_COUNT, BANK_COUNT, BUFF_COUNT,
    BUILDER_ACCESSORY_COUNT, CELLPHONE_INFO_COUNT, COINS_COUNT, CURRENT_VERSION,
    DPAD_BINDINGS_COUNT, EQUIPMENT_COUNT, INVENTORY_COUNT, LOADOUT_COUNT, TEMPORARY_SLOT_COUNT,
    TICKS_PER_MICROSECOND,
};

// TODO: Seperate & implement these properly
#[derive(Default, Clone, Debug)]
pub struct Spawnpoint;

#[derive(Default, Clone, Debug)]
pub struct JourneyPowerManager;

#[derive(Default, Clone, Debug)]
pub struct Loadout;

#[repr(u8)]
#[derive(Default, Debug)]
pub enum Difficulty {
    #[default]
    Classic = 0,
    Mediumcore = 1,
    Hardcore = 2,
    Journey = 3,
}

#[derive(Debug)]
pub struct Player {
    pub version: i32,
    pub revision: u32,
    pub is_favourite: bool,

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
            is_favourite: false,

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
