mod aes;
pub mod bool_byte;
pub mod buff;
pub mod difficulty;
mod ext;
pub mod file_type;
pub mod item;
pub mod journey_powers;
pub mod loadout;
pub mod meta;
pub mod player;
pub mod prefix;
mod shared_string;
pub mod spawnpoint;
pub mod utils;

pub const MAGIC_MASK: u64 = 0xFFFFFFFFFFFFFF;
pub const MAGIC_NUMBER: u64 = 0x6369676F6C6572;

/// `h3y_gUyZ`, with null bytes every other byte
pub const ENCRYPTION_BYTES: &[u8; 16] = b"h\x003\x00y\x00_\x00g\x00U\x00y\x00Z\x00";

pub const CURRENT_VERSION: i32 = 279;

/// C#'s DateTime uses Ticks, so here is the conversion factor
pub const NANOSECONDS_PER_TICK: i128 = 100;

pub const ARMOR_COUNT: usize = 3;
pub const ACCESSORY_COUNT: usize = 7;
/// Not entirely sure why armor is also counted in this.
/// My best guess is something to do with the Familiar set?
pub const HIDDEN_VISUAL_COUNT: usize = ARMOR_COUNT + ACCESSORY_COUNT;
pub const INVENTORY_COUNT: usize = 50;
pub const INVENTORY_STRIDE: usize = 10;
pub const COINS_COUNT: usize = 4;
pub const AMMO_COUNT: usize = 4;
pub const EQUIPMENT_COUNT: usize = 5;
pub const BANK_COUNT: usize = 40;
pub const BANK_STRIDE: usize = 10;
pub const BUFF_COUNT: usize = 44;
pub const BUFF_STRIDE: usize = 11;
pub const SPAWNPOINT_LIMIT: usize = 200;
pub const CELLPHONE_INFO_COUNT: usize = 13;
pub const DPAD_BINDINGS_COUNT: usize = 4;
/// Ruler, MechanicalRuler, Presserator, PaintSprayer,
/// RedWire, GreenWire, BlueWire, YellowWire, WireViewMode,
/// Actuators, BlockSwap, TorchGodsFavor
pub const BUILDER_ACCESSORY_COUNT: usize = 12;
/// Mouse, ItemByIndex, GuideItem, ReforgeItem
pub const TEMPORARY_SLOT_COUNT: usize = 4;
pub const LOADOUT_COUNT: usize = 3;

pub const MAX_RESPAWN_TIME: i32 = 60000;

// Strange brew is strange
pub const STRANGE_BREW_ID: i32 = 3001;
pub const STRANGE_BREW_MAX_HEAL: i32 = 120;

pub const FEMALE_SKIN_VARIANTS: [i32; 4] = [5, 6, 9, 11];
pub const MALE_SKIN_VARIANTS: [i32; 6] = [0, 1, 2, 3, 8, 10];
// TODO: skin variant 7 should be saved as skin variant 9 (I think)
pub const SKIN_VARIANT_COUNT: u8 = 11;

pub const HAIR_STYLE_COUNT: i32 = 165;
pub const HAIR_DYE_COUNT: u8 = 13;

pub const BUFF_SPRITE_SIZE: usize = 16;

pub type Color = [u8; 3];

pub use bool_byte::{BoolByte, BoolByteError};
pub use buff::{Buff, BuffMeta, BuffType};
pub use difficulty::Difficulty;
pub use file_type::FileType;
pub use item::{Item, ItemError, ItemMeta, ItemRarity, ItemType, ResearchItem};
pub use journey_powers::{JourneyPowerId, JourneyPowers};
pub use loadout::Loadout;
pub use player::{Player, PlayerError};
pub use prefix::{Prefix, PrefixMeta};
pub use shared_string::SharedString;
pub use spawnpoint::Spawnpoint;
