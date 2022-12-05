use std::{fs::OpenOptions, io::BufWriter};

use anyhow::Result;

use terra_core::{buff::Buff, item::Item, player::Player, prefix::Prefix, utils::get_player_dir};

fn main() -> Result<()> {
    let chara_name = "Pogo";

    let player_dir = get_player_dir();

    let filepath = player_dir.join(format!("{}.plr", chara_name));

    let items = Item::load_items()?;
    let buffs = Buff::load_buffs()?;
    let prefixes = Prefix::load_prefixes()?;

    println!("Items count: {}", items.len());
    println!("Buffs count: {}", buffs.len());
    println!("Prefixes count: {}", prefixes.len());

    let mut plr = Player::default();

    println!("Default Player Name: {}", plr.name);

    plr.load(&filepath, &prefixes, &items, &buffs)?;

    println!("Current Player Name: {}", plr.name);

    // Serde Serialization
    {
        let mut json_file = OpenOptions::new()
            .create(true)
            .write(true)
            .append(false)
            .open(player_dir.join(format!("{}.json", chara_name)))?;

        let mut json_writer = BufWriter::new(&mut json_file);

        serde_json::to_writer_pretty(&mut json_writer, &plr)?;
    }

    let outpath = player_dir.join(format!("{}_The_Second.plr", chara_name));
    plr.name = "Pogo The Second".to_string();

    println!("New Player Name: {}", plr.name);

    plr.save(&outpath)?;

    Ok(())
}
