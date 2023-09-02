use terra_core::{
    utils::{get_player_dir, to_hex, version_lookup},
    BuffMeta, Item, ItemMeta, Player, PrefixMeta, TICKS_PER_MICROSECOND, meta::Meta,
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

    let item_meta = ItemMeta::load().expect("Could not load items.");
    let buff_meta = BuffMeta::load().expect("Could not load buffs.");
    let prefix_meta = PrefixMeta::load().expect("Could not load prefixes.");

    let player_file_name: String = args.next().unwrap();

    println!("Item count: {}", item_meta.len());
    println!("Buff count: {}", buff_meta.len());
    println!("Prefix count: {}", prefix_meta.len());

    let filepath = player_dir.join(format!("{}.plr", &player_file_name));

    let mut player = Player::default();

    if let Err(err) = player.load(&item_meta, &filepath) {
        println!("Error loading player '{}': {}", player_file_name, err);
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
    println!("Hair Color: {}", to_hex(player.hair_color));
    println!("Skin Color: {}", to_hex(player.skin_color));
    println!("Eye Color: {}", to_hex(player.eye_color));
    println!("Shirt Color: {}", to_hex(player.shirt_color));
    println!("Undershirt Color: {}", to_hex(player.undershirt_color));
    println!("Pants Color: {}", to_hex(player.pants_color));
    println!("Shoe Color: {}", to_hex(player.shoe_color));
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

    println!("Armor:");
    print_items(&item_meta, &prefix_meta, &player.loadouts[0].armor);
    println!("Vanity Armor:");
    print_items(&item_meta, &prefix_meta, &player.loadouts[0].vanity_armor);
    println!("Accessories:");
    print_items(&item_meta, &prefix_meta, &player.loadouts[0].accessories);
    println!("Vanity Accessories:");
    print_items(
        &item_meta,
        &prefix_meta,
        &player.loadouts[0].vanity_accessories,
    );
    println!("Equipment:");
    print_items(&item_meta, &prefix_meta, &player.equipment);
}

fn print_items(item_meta: &Vec<ItemMeta>, prefix_meta: &Vec<PrefixMeta>, items: &[Item]) {
    items.iter().for_each(|item| {
        let item_meta = ItemMeta::get(item_meta, item.id);
        let name = item_meta.map_or("Unknown", |m| m.name.as_str());
        let prefix_meta = PrefixMeta::get(prefix_meta, item.prefix.id);
        let prefix_name = prefix_meta.map_or(String::new(), |p| format!(" {}", p.name));

        print!("  [{: >4}]", item.id);
        if item.stack > 1 {
            print!(" ({})", item.stack);
        }
        println!("{} {}", prefix_name, name);
    });
}
