use std::{
    num::ParseIntError,
    path::{Path, PathBuf},
};

use dirs_next::{data_local_dir, document_dir};

use crate::{Color, Item, NANOSECONDS_PER_TICK};

pub fn get_data_dir() -> PathBuf {
    match std::env::consts::OS {
        "windows" => document_dir().unwrap(),
        _ => data_local_dir().unwrap(),
    }
}

pub fn get_terraria_dir() -> PathBuf {
    match std::env::consts::OS {
        "windows" => get_data_dir().join("My Games"),
        _ => get_data_dir(),
    }
    .join("Terraria")
}

pub fn get_player_dir() -> PathBuf {
    get_terraria_dir().join("Players")
}

pub fn get_player_dir_or_default(player_path: &Path) -> PathBuf {
    let parent = player_path.parent();

    let fallback = || {
        let player_dir = get_player_dir();

        if player_dir.exists() {
            player_dir
        } else {
            get_data_dir()
        }
    };

    match parent {
        Some(directory) => {
            if directory.exists() {
                directory.to_path_buf()
            } else {
                fallback()
            }
        }
        None => fallback(),
    }
}

pub fn version_lookup(version: i32) -> &'static str {
    match version {
        i32::MIN..=-1 => "Unknown",
        0 => "1.0",
        1 => "1.0 (or newer)",
        2 => "1.0.1",
        3 => "1.0.2",
        4 => "1.0.3",
        5..=8 => "1.0.3 (or newer)",
        9 => "1.0.4",
        10..=11 => "1.0.4 (or newer)",
        12 => "1.0.5",
        13..=19 => "1.0.5 (or newer)",
        20 => "1.0.6",
        21 => "1.0.6 (or newer)",
        22 => "1.0.6.1",
        23..=35 => "1.0.6.1 (or newer)",
        36 => "1.1",
        37 => "1.1.1",
        38 => "1.1.1 (or newer)",
        39 => "1.1.2",
        40..=66 => "1.1.2 (or newer)",
        67 => "1.2",
        68 => "1.2.0.1",
        69 => "1.2.0.2",
        70 => "1.2.0.3",
        71 => "1.2.0.3.1",
        72 => "1.2.1/1.2.1.1",
        73 => "1.2.1.2",
        74..=76 => "1.2.1.2 (or newer)",
        77 => "1.2.2",
        78..=92 => "1.2.2 (or newer)",
        93 => "1.2.3",
        94 => "1.2.3.1",
        95..=100 => "1.2.3.1 (or newer)",
        101 => "1.2.4",
        102 => "1.2.4.1",
        103..=145 => "1.2.4.1 (or newer)",
        146 => "1.3.0.1",
        147 => "1.3.0.2",
        148 => "1.3.0.2 (or newer)",
        149 => "1.3.0.3",
        150 => "1.3.0.3 (or newer)",
        151 => "1.3.0.4",
        152 => "1.3.0.4 (or newer)",
        153 => "1.3.0.5",
        154 => "1.3.0.6",
        155 => "1.3.0.7",
        156 => "1.3.0.8",
        157..=167 => "1.3.0.8 (or newer)",
        168 => "1.3.1",
        169 => "1.3.1.1",
        170..=171 => "1.3.1.1 (or newer)",
        172 => "1.3.2",
        173 => "1.3.2.1",
        174 => "1.3.2.1 (or newer)",
        175 => "1.3.3",
        176 => "1.3.3.1/1.3.3.2",
        177 => "1.3.3.3",
        178..=183 => "1.3.3.3 (or newer)",
        184 => "1.3.4",
        185 => "1.3.4.1",
        186 => "1.3.4.2",
        187 => "1.3.4.3",
        188 => "1.3.4.4",
        189..=190 => "1.3.4.4 (or newer)",
        191 => "1.3.5",
        192 => "1.3.5.1",
        193 => "1.3.5.2",
        194 => "1.3.5.3",
        195..=224 => "1.3.5.3 (or newer)",
        225 => "1.4.0.1",
        226 => "1.4.0.2",
        227 => "1.4.0.3",
        228 => "1.4.0.4",
        229 => "1.4.0.4 (or newer)",
        230 => "1.4.0.5",
        231 => "1.4.0.5 (or newer)",
        232 => "1.4.1",
        233 => "1.4.1.1",
        234 => "1.4.1.2",
        235 => "1.4.2",
        236 => "1.4.2.1",
        237 => "1.4.2.2",
        238 => "1.4.2.3",
        239..=241 => "1.4.2.3 (or newer)",
        242 => "1.4.3",
        243 => "1.4.3.1",
        244 => "1.4.3.2",
        245 => "1.4.3.3",
        246 => "1.4.3.4",
        247 => "1.4.3.5",
        248 => "1.4.3.6",
        249..=268 => "1.4.3.6 (or newer)",
        269 => "1.4.4",
        270 => "1.4.4.1",
        271 => "1.4.4.2",
        272 => "1.4.4.3",
        273 => "1.4.4.4",
        274 => "1.4.4.5",
        275 => "1.4.4.6",
        276 => "1.4.4.7",
        277 => "1.4.4.8",
        278 => "1.4.4.8.1",
        279 => "1.4.4.9",
        _ => "1.4.4.9 (or newer)",
    }
}

pub fn use_time_lookup(use_time: i32) -> &'static str {
    match use_time {
        i32::MIN..=8 => "insanely fast",
        9..=20 => "very fast",
        21..=25 => "fast",
        26..=30 => "average",
        31..=35 => "slow",
        36..=45 => "very slow",
        46..=55 => "extremely slow",
        _ => "insanely slow",
    }
}

pub fn knockback_lookup(knockback: f32) -> &'static str {
    // NOTE: This is the only way this'll work, floating-point is weird
    if knockback <= 1.5 {
        "extremely weak"
    } else if knockback <= 3. {
        "very weak"
    } else if knockback <= 4. {
        "weak"
    } else if knockback <= 6. {
        "average"
    } else if knockback <= 7. {
        "strong"
    } else if knockback <= 9. {
        "very strong"
    } else if knockback <= 11. {
        "extremely strong"
    } else {
        "insane"
    }
}

pub fn ticks_to_string(ticks: i32) -> String {
    const FRAMES_PER_SECOND: i32 = 60;
    const FRAMES_PER_MINUTE: i32 = FRAMES_PER_SECOND * 60;
    const FRAMES_PER_HOUR: i32 = FRAMES_PER_MINUTE * 60;
    const FRAMES_PER_THOUSAND_HOURS: i32 = FRAMES_PER_HOUR * 1000;

    if ticks < FRAMES_PER_SECOND {
        format!("{}f", ticks)
    } else if ticks < FRAMES_PER_MINUTE {
        format!("{}s", ticks / FRAMES_PER_SECOND)
    } else if ticks < FRAMES_PER_HOUR {
        format!("{}m", ticks / FRAMES_PER_MINUTE)
    } else if ticks < FRAMES_PER_THOUSAND_HOURS {
        format!("{}h", ticks / FRAMES_PER_HOUR)
    } else {
        "∞".to_owned()
    }
}

pub fn coins_to_string(value: i32) -> String {
    if value <= 0 {
        return "Nothing".to_owned();
    }

    let mut parts = Vec::with_capacity(4);

    let value = (value / 5) as f32;

    let platinum = (value / 100. / 100. / 100.).floor() as i32;
    let gold = (value / 100. / 100.).floor() as i32 % 100;
    let silver = (value / 100.).floor() as i32 % 100;
    let copper = value as i32 % 100;

    if platinum > 0 {
        parts.push(format!("{} Platinum", platinum));
    }
    if gold > 0 {
        parts.push(format!("{} Gold", gold));
    }
    if silver > 0 {
        parts.push(format!("{} Silver", silver));
    }
    if copper > 0 {
        parts.push(format!("{} Copper", copper));
    }

    let mut string = String::new();

    let mut iter = parts.into_iter().peekable();

    while let Some(next) = iter.next() {
        string += &next;

        if iter.peek().is_some() {
            string += ", ";
        }
    }

    string
}

pub trait AsTicks {
    fn as_ticks(&self) -> i64;
}

impl AsTicks for std::time::Duration {
    fn as_ticks(&self) -> i64 {
        (self.as_nanos() / NANOSECONDS_PER_TICK as u128) as i64
    }
}

impl AsTicks for time::Duration {
    fn as_ticks(&self) -> i64 {
        (self.whole_nanoseconds() / NANOSECONDS_PER_TICK) as i64
    }
}

pub fn from_hex(hex: &str) -> anyhow::Result<Color, ParseIntError> {
    let start = if hex.starts_with('#') { 1 } else { 0 };
    let r = u8::from_str_radix(&hex[start..(start + 2)], 16)?;
    let g = u8::from_str_radix(&hex[(start + 2)..(start + 4)], 16)?;
    let b = u8::from_str_radix(&hex[(start + 4)..(start + 6)], 16)?;
    Ok([r, g, b])
}

pub fn to_hex(color: Color) -> String {
    format!("#{:x}{:x}{:x}", color[0], color[1], color[2])
}

pub fn has_item(id: i32, inventory: &[Item]) -> bool {
    inventory.iter().any(|a| a.id == id)
}

pub(crate) fn current_save_time() -> i64 {
    (time::OffsetDateTime::now_utc().unix_timestamp_nanos() / NANOSECONDS_PER_TICK) as i64
}
