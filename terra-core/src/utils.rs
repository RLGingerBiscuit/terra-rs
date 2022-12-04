use std::path::PathBuf;

use dirs_next::{data_local_dir, document_dir};

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
    .join("Terraria").join("Players")
}
