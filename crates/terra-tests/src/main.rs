use std::{fs::File, io::Read, path::PathBuf};

use terra_core::{BuffMeta, ItemMeta, Player, PrefixMeta};

#[derive(thiserror::Error, Debug)]
pub enum TestError {
    #[error("Error during loading")]
    Load(anyhow::Error),
    #[error("Error during saving")]
    Save(anyhow::Error),
    #[error("Error during comparison")]
    Comparison,
}

const VERSIONS: [usize; 19] = [
    39, 69, 73, 77, 93, 98, 145, 168, 175, 184, 190, 225, 230, 237, 248, 269, 279, 315, 316,
];

fn run_test(
    chara_name: &String,
    directory: &PathBuf,
    item_meta: &Vec<ItemMeta>,
) -> anyhow::Result<(), TestError> {
    let filepath = directory.join(format!("{}.plr", chara_name));

    println!("Filepath: {}", filepath.display());

    {
        let decrypted_filepath = directory.join(format!("{}.dplr", chara_name));
        if let Err(err) = Player::decrypt_file(&filepath, &decrypted_filepath) {
            return Err(TestError::Save(err));
        }
    }

    let mut plr = Player::default();

    if let Err(err) = plr.load(item_meta, &filepath) {
        return Err(TestError::Load(err));
    }

    println!("\tVersion: {}", &plr.version);
    println!("\tName: {}", &plr.name);

    let out_filepath = directory.join(format!("{}.saved.plr", &chara_name));
    let out_decrypted_filepath = directory.join(format!("{}.saved.dplr", &chara_name));

    if let Err(err) = plr.save(item_meta, &out_filepath) {
        return Err(TestError::Save(err));
    }

    if let Err(err) = plr.save_decrypted(item_meta, &out_decrypted_filepath) {
        return Err(TestError::Save(err));
    }

    let mut old_file = File::open(&filepath).expect("Could not open old file");
    let mut new_file = File::open(&out_filepath).expect("Could not open new file");

    let mut old_buf = [0; 10000];
    let mut new_buf = [0; 10000];

    loop {
        if let Ok(r1) = old_file.read(&mut old_buf) {
            if r1 > 0 {
                if let Ok(r2) = new_file.read(&mut new_buf) {
                    if r1 == r2 {
                        if old_buf == new_buf {
                            continue;
                        }
                    }
                    return Err(TestError::Comparison);
                } else {
                    break;
                }
            } else {
                break;
            }
        } else {
            break;
        }
    }

    Ok(())
}

fn main() {
    let mut player_dir = PathBuf::new();
    player_dir.push("tests");

    let item_meta: Vec<ItemMeta> =
        serde_json::from_str(include_str!("../../../data/resources/items.json"))
            .expect("Could not load items");
    let buff_meta: Vec<BuffMeta> =
        serde_json::from_str(include_str!("../../../data/resources/buffs.json"))
            .expect("Could not load buffs");
    let prefix_meta: Vec<PrefixMeta> =
        serde_json::from_str(include_str!("../../../data/resources/prefixes.json"))
            .expect("Could not load prefixes");

    println!("Items count: {}", item_meta.len());
    println!("Buffs count: {}", buff_meta.len());
    println!("Prefixes count: {}", prefix_meta.len());

    let mut tests: Vec<String> = Vec::new();
    tests.extend(VERSIONS.iter().map(|v| format!("v{v}")));
    tests.push("テラリア".to_owned());

    for chara_name in tests {
        match run_test(&chara_name, &player_dir, &item_meta) {
            Ok(_) => println!("'{}.plr' loaded/saved successfully", &chara_name),
            Err(err) => match err {
                TestError::Load(err) => println!("Error whilst loading\n---\n{:?}\n---", err),
                TestError::Save(err) => println!("Error whilst saving\n---\n{:?}\n---", err),
                TestError::Comparison => println!("Saved file was not the same as the loaded file"),
            },
        }
    }
}
