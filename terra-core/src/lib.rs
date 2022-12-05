use palette::{encoding::Srgb, rgb::Rgb};

pub const MAGIC_MASK: u64 = 72057594037927935;
pub const MAGIC_NUMBER: u64 = 27981915666277746;

/// `h3y_gUyZ`, with null bytes every other bit
pub const ENCRYPTION_BYTES: &[u8; 16] = b"h\x003\x00y\x00_\x00g\x00U\x00y\x00Z\x00";

pub const CURRENT_VERSION: i32 = 279;

/// C#'s DateTime uses Ticks, so here is the conversion factor
pub const TICKS_PER_MICROSECOND: usize = 10;

pub const ARMOR_COUNT: usize = 3;
pub const ACCESSORY_COUNT: usize = 7;
/// Not entirely sure why armor is also counted in this.
/// My best guess is something to do with the Familiar set?
pub const HIDDEN_VISUAL_COUNT: usize = ARMOR_COUNT + ACCESSORY_COUNT;
pub const INVENTORY_COUNT: usize = 50;
pub const COINS_COUNT: usize = 4;
pub const AMMO_COUNT: usize = 4;
pub const EQUIPMENT_COUNT: usize = 5;
pub const BANK_COUNT: usize = 40;
pub const BUFF_COUNT: usize = 44;
pub const SPAWNPOINT_LIMIT: usize = 200;
pub const CELLPHONE_INFO_COUNT: usize = 13;
pub const DPAD_BINDINGS_COUNT: usize = 4;
/// Ruler, MechanicalRuler, Presserator, PaintSprayer,
/// RedWire, GreenWire, BlueWire, YellowWire, WireViewMode,
/// Actuators, BlockSwap, TorchGodsFavor
pub const BUILDER_ACCESSORY_COUNT: usize = 12;
/// Mouse, ItemByIndex, GuideItem, ReforgeItem
pub const TEMPORARY_SLOT_COUNT: usize = 4;
/// The current armor/accessories are also counted as a loadout
pub const LOADOUT_COUNT: usize = 4;

pub const MAX_RESPAWN_TIME: i32 = 60000;

pub const FEMALE_SKIN_VARIANTS: [i32; 4] = [5, 6, 9, 11];
pub const MALE_SKIN_VARIANTS: [i32; 6] = [0, 1, 2, 3, 8, 10];

pub type Color = Rgb<Srgb, u8>;

pub mod bool_byte;
pub mod buff;
pub mod difficulty;
pub mod io_extensions;
pub mod item;
pub mod loadout;
pub mod player;
pub mod prefix;
pub mod spawnpoint;
pub mod utils;
pub mod journey_powers;
