use std::{fs::File, io::Read, path::PathBuf};

use anyhow::Result;

use terra_core::{buff::Buff, item::Item, player::Player, prefix::Prefix};

#[derive(thiserror::Error, Debug)]
pub enum TestError {
    #[error("Error during loading")]
    Load(anyhow::Error),
    #[error("Error during saving")]
    Save(anyhow::Error),
    #[error("Error during comparison")]
    Comparison,
}

const VERSIONS: [usize; 17] = [
    39, 69, 73, 77, 93, 98, 145, 168, 175, 184, 190, 225, 230, 237, 248, 269, 279,
];

fn run_test(
    version: usize,
    directory: &PathBuf,
    items: &Vec<Item>,
    buffs: &Vec<Buff>,
    prefixes: &Vec<Prefix>,
) -> Result<(), TestError> {
    let chara_name = format!("v{}", version);
    let filepath = directory.join(format!("{}.plr", &chara_name));

    let mut plr = Player::default();

    println!("Version {}", version);

    if let Err(err) = plr.load(&filepath, prefixes, items, buffs) {
        return Err(TestError::Load(err));
    }

    println!("Actual Version {}", &plr.version);
    println!("\tName: {}", &plr.name);

    let out_filepath = directory.join(format!("{}.saved.plr", &chara_name));

    if let Err(err) = plr.save(&out_filepath) {
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

fn main() -> Result<()> {
    let mut player_dir = PathBuf::new();
    player_dir.push("./tests");

    let items = Item::load_items()?;
    let buffs = Buff::load_buffs()?;
    let prefixes = Prefix::load_prefixes()?;

    println!("Items count: {}", items.len());
    println!("Buffs count: {}", buffs.len());
    println!("Prefixes count: {}", prefixes.len());

    for version in VERSIONS {
        match run_test(version, &player_dir, &items, &buffs, &prefixes) {
            Ok(_) => println!("v{} loaded/saved successfully", version),
            Err(err) => match err {
                TestError::Load(err) => println!("Error whilst loading\n---\n{:?}\n---", err),
                TestError::Save(err) => println!("Error whilst saving\n---\n{:?}\n---", err),
                TestError::Comparison => println!("Saved file was not the same as the loaded file"),
            },
        }
    }

    Ok(())
}
