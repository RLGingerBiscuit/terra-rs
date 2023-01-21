use terra_core::{
    utils::{get_player_dir, version_lookup},
    Buff, Item, Player, Prefix, TICKS_PER_MICROSECOND,
};

fn usage() {
    println!("Usage: terra-tui PLAYER_FILENAME");
}

fn main() {
    let player_dir = get_player_dir();

    let mut args = std::env::args().skip(1);

    if args.len() == 0 {
        usage();
        return;
    }

    let items = Item::load_items().expect("Could not load items.");
    let buffs = Buff::load_buffs().expect("Could not load buffs.");
    let prefixes = Prefix::load_prefixes().expect("Could not load prefixes.");

    let player_filename: String = args.next().unwrap();

    println!("Items count: {}", items.len());
    println!("Buffs count: {}", buffs.len());
    println!("Prefixes count: {}", prefixes.len());

    let filepath = player_dir.join(format!("{}.plr", &player_filename));

    let mut player = Player::default();

    if let Err(err) = player.load(&filepath, &prefixes, &items, &buffs) {
        println!("Error loading player '{}': {}", player_filename, err);
        return;
    }

    println!("------");

    println!("Name: {}", player.name);
    println!(
        "Version: {} ({})",
        version_lookup(player.version),
        player.version
    );
    {
        let playtime = player.playtime / (TICKS_PER_MICROSECOND as i64) / 1000000;
        println!(
            "Playtime: {:02}:{:02}:{:02}",
            playtime / 3600,
            (playtime / 60) % 60,
            playtime % 60
        );
    }
    println!("Difficulty: {}", player.difficulty);
    println!("Health: {}/{}", player.life, player.max_life);
    println!("Mana: {}/{}", player.mana, player.max_mana);
    println!("Demon Heart: {}", player.demon_heart);
    println!("Artisan Loaf: {}", player.artisan_loaf);
    println!("Vital Crystal: {}", player.vital_crystal);
    println!("Aegis Fruit: {}", player.aegis_fruit);
    println!("Arcane Crystal: {}", player.arcane_crystal);
    println!("Galaxy Pearl: {}", player.galaxy_pearl);
    println!("Gummy Worm: {}", player.gummy_worm);
    println!("Ambrosia: {}", player.ambrosia);
    println!("Defeated Torch God: {}", player.biome_torches);
    println!("Defeated Old One's Army: {}", player.defeated_ooa);
    println!("PVE Deaths: {}", player.pve_deaths);
    println!("PVP Deaths: {}", player.pvp_deaths);
    println!("Hair Color: #{:x}", player.hair_color);
    println!("Skin Color: #{:x}", player.skin_color);
    println!("Eye Color: #{:x}", player.eye_color);
    println!("Shirt Color: #{:x}", player.shirt_color);
    println!("Undershirt Color: #{:x}", player.undershirt_color);
    println!("Pants Color: #{:x}", player.pants_color);
    println!("Shoe Color: #{:x}", player.shoe_color);
    println!("Angler Quests: {}", player.angler_quests);
    if player.dead {
        println!(
            "Is Dead: {} ({} seconds left)",
            player.dead,
            player.respawn_timer / 60
        );
    } else {
        println!("Is Dead: {}", player.dead);
    }
    println!("Golf Highscore: {}", player.golfer_score);
    println!("Upgraded Minecarts: {}", player.super_cart);
}
