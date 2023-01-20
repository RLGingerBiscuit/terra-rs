use std::path::PathBuf;

use dirs_next::{data_local_dir, document_dir};

use crate::Item;

pub fn get_terraria_dir() -> PathBuf {
    match std::env::consts::OS {
        "windows" => document_dir().unwrap().join("My Games"),
        _ => data_local_dir().unwrap(),
    }
    .join("Terraria")
}

pub fn get_player_dir() -> PathBuf {
    match std::env::consts::OS {
        "windows" => document_dir().unwrap().join("My Games"),
        _ => data_local_dir().unwrap(),
    }
    .join("Terraria")
    .join("Players")
}

pub fn version_lookup(version: i32) -> &'static str {
    if version <= 1 {
        "1.0.0"
    } else if version <= 2 {
        "1.0.1"
    } else if version <= 8 {
        "1.0.2/1.0.3"
    } else if version <= 9 {
        "1.0.4"
    } else if version <= 12 {
        "1.0.5"
    } else if version <= 21 {
        "1.0.6"
    } else if version <= 22 {
        "1.0.6.1"
    } else if version <= 66 {
        "1.1.x"
    } else if version <= 67 {
        "1.2"
    } else if version <= 68 {
        "1.2.0.1"
    } else if version <= 69 {
        "1.2.0.2"
    } else if version <= 70 {
        "1.2.0.3"
    } else if version <= 71 {
        "1.2.0.3.1"
    } else if version <= 72 {
        "1.2.1/1.2.1.1"
    } else if version <= 73 {
        "1.2.1.2"
    } else if version <= 77 {
        "1.2.2"
    } else if version <= 93 {
        "1.2.3"
    } else if version <= 94 {
        "1.2.3.1"
    } else if version <= 101 {
        "1.2.4"
    } else if version <= 102 {
        "1.2.4.1"
    } else if version <= 146 {
        "1.3.0.1"
    } else if version <= 147 {
        "1.3.0.2"
    } else if version <= 149 {
        "1.3.0.3"
    } else if version <= 151 {
        "1.3.0.4"
    } else if version <= 153 {
        "1.3.0.5"
    } else if version <= 154 {
        "1.3.0.6"
    } else if version <= 155 {
        "1.3.0.7"
    } else if version <= 156 {
        "1.3.0.8"
    } else if version <= 168 {
        "1.3.1"
    } else if version <= 169 {
        "1.3.1.1"
    } else if version <= 172 {
        "1.3.2"
    } else if version <= 173 {
        "1.3.2.1"
    } else if version <= 175 {
        "1.3.3"
    } else if version <= 176 {
        "1.3.3.1/1.3.3.2/1.3.3.3"
    } else if version <= 177 {
        "1.3.3.4"
    } else if version <= 184 {
        "1.3.4"
    } else if version <= 185 {
        "1.3.4.1"
    } else if version <= 186 {
        "1.3.4.2"
    } else if version <= 187 {
        "1.3.4.3"
    } else if version <= 188 {
        "1.3.4.4"
    } else if version <= 191 {
        "1.3.5"
    } else if version <= 192 {
        "1.3.5.1"
    } else if version <= 193 {
        "1.3.5.2"
    } else if version <= 194 {
        "1.3.5.3"
    } else if version <= 225 {
        "1.4.0.1"
    } else if version <= 226 {
        "1.4.0.2"
    } else if version <= 227 {
        "1.4.0.3"
    } else if version <= 228 {
        "1.4.0.4"
    } else if version <= 230 {
        "1.4.0.5"
    } else if version <= 232 {
        "1.4.1"
    } else if version <= 233 {
        "1.4.1.1"
    } else if version <= 234 {
        "1.4.1.2"
    } else if version <= 235 {
        "1.4.2/1.4.2.1"
    } else if version <= 237 {
        "1.4.2.2"
    } else if version <= 238 {
        "1.4.2.3"
    } else if version <= 242 {
        "1.4.3"
    } else if version <= 243 {
        "1.4.3.1"
    } else if version <= 244 {
        "1.4.3.2"
    } else if version <= 245 {
        "1.4.3.3"
    } else if version <= 246 {
        "1.4.3.4"
    } else if version <= 247 {
        "1.4.3.5"
    } else if version <= 248 {
        "1.4.3.6"
    } else if version <= 269 {
        "1.4.4"
    } else if version <= 270 {
        "1.4.4.1"
    } else if version <= 271 {
        "1.4.4.2"
    } else if version <= 272 {
        "1.4.4.3"
    } else if version <= 273 {
        "1.4.4.4"
    } else if version <= 274 {
        "1.4.4.5"
    } else if version <= 275 {
        "1.4.4.6"
    } else if version <= 276 {
        "1.4.4.7"
    } else if version <= 277 {
        "1.4.4.8"
    } else if version <= 278 {
        "1.4.4.8.1"
    } else if version <= 279 {
        "1.4.4.9"
    } else {
        "Unknown"
    }
}

// pub(crate) because it only takes [Item]
pub(crate) fn has_item(id: i32, inventory: &[Item]) -> bool {
    inventory.iter().any(|a| a.id == id)
}
