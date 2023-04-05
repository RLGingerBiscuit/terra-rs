use std::{
    collections::HashMap,
    fs::{self, File, OpenOptions},
    io::{BufWriter, Read},
    path::PathBuf,
    str::FromStr,
};

use anyhow::Result;
// use image::{DynamicImage, GenericImage, ImageFormat};
use regex::{Captures, Regex};

use fs_extra::dir::{copy as copy_dir, create as create_dir, CopyOptions as DirCopyOptions};

use rlua::{Lua, Table};
use terra_core::{BuffMeta, BuffType, ItemMeta, ItemRarity, PrefixMeta};

const ITEM_DATA_URL: &str = "https://terraria.wiki.gg/api.php?action=query&prop=revisions&format=json&rvlimit=1&rvslots=*&rvprop=content&titles=Module:Iteminfo/data";
const BUFF_IDS_URL: &str = "https://terraria.wiki.gg/wiki/Buff_IDs";
const PREFIX_IDS_URL: &str = "https://terraria.wiki.gg/wiki/Prefix_IDs";
const ITEM_CSS_URL: &str = "https://yal.cc/r/terrasavr/plus/img/items.css";
const BUFF_CSS_URL: &str = "https://yal.cc/r/terrasavr/plus/img/buffs.css";

fn expand_templates(
    s: String,
    template: &Regex,
    game: &serde_json::Value,
    items: &serde_json::Value,
    npcs: &serde_json::Value,
) -> String {
    if s == "NOTHING" {
        return "".to_owned();
    }

    let expanded = template
        .replace_all(&s, |cap: &Captures| {
            if cap[1].starts_with("NPC") {
                if cap[2].to_owned() == "None" {
                    "".to_owned()
                } else {
                    npcs[&cap[1]][&cap[2]].as_str().unwrap().to_owned()
                }
            } else if cap[1].starts_with("Buff") {
                game[&cap[1]][&cap[2]]
                    .as_str()
                    .unwrap_or_else(|| {
                        println!("Warning: {} not found!", &cap[0]);
                        &cap[0][2..{ cap.len() - 1 }]
                    })
                    .to_owned()
            } else if cap[1].contains("Item") || cap[1].starts_with("PaintingArtist") {
                items[&cap[1]][&cap[2]].as_str().unwrap().to_owned()
            } else {
                println!("Warning: {} not found!", &cap[0]);
                "".to_owned()
            }
        })
        .to_string();

    if expanded.contains(r"{$}") {
        expand_templates(expanded, template, game, items, npcs)
    } else {
        expanded
            .replace("<right>", "Right Click")
            .replace("<left>", "Left Click")
    }
}

fn get_item_offsets() -> Result<HashMap<i32, [i32; 2]>> {
    let offset_regex = Regex::new(r"^.*id='(\d+)'.*ofs: -(\d+)px -(\d+)")?;

    let resp = reqwest::blocking::get(ITEM_CSS_URL)?;
    let text = resp.text()?;

    let mut offsets = HashMap::new();

    text.lines().for_each(|line| {
        let captures = offset_regex.captures(line).unwrap();
        let id = i32::from_str(captures.get(1).unwrap().as_str()).unwrap();
        let x = i32::from_str(captures.get(2).unwrap().as_str()).unwrap();
        let y = i32::from_str(captures.get(3).unwrap().as_str()).unwrap();

        offsets.insert(id, [x, y]);
    });

    Ok(offsets)
}

fn get_buff_offsets() -> Result<HashMap<i32, [i32; 2]>> {
    let offset_regex = Regex::new(r"^.*id='(\d+)'.*ofs: -(\d+)px -(\d+)")?;

    let resp = reqwest::blocking::get(BUFF_CSS_URL)?;
    let text = resp.text()?;

    let mut offsets = HashMap::new();

    text.lines().for_each(|line| {
        let captures = offset_regex.captures(line).unwrap();
        let id = i32::from_str(captures.get(1).unwrap().as_str()).unwrap();
        let x = i32::from_str(captures.get(2).unwrap().as_str()).unwrap();
        let y = i32::from_str(captures.get(3).unwrap().as_str()).unwrap();

        offsets.insert(id, [x, y]);
    });

    Ok(offsets)
}

fn get_item_meta(
    template: &Regex,
    game: &serde_json::Value,
    items: &serde_json::Value,
    npcs: &serde_json::Value,
) -> Result<Vec<ItemMeta>> {
    // https://terraria.wiki.gg/wiki/Module:Iteminfo/data
    // API: https://terraria.wiki.gg/api.php?action=query&prop=revisions&titles=Module:Iteminfo/data&rvlimit=1&rvslots=*&rvprop=ids|content&format=json
    // r[query][pages][18021][revisions][0][slots][main][*]
    // Split on "local cache = require 'mw.ext.LuaCache'"

    let lua_resp = reqwest::blocking::get(ITEM_DATA_URL)?;
    let lua_json: serde_json::Value = lua_resp.json()?;

    // Yeesh
    let lua_str = lua_json
        .as_object()
        .unwrap()
        .get("query")
        .unwrap()
        .get("pages")
        .unwrap()
        .get("18021")
        .unwrap()
        .get("revisions")
        .unwrap()
        .get(0)
        .unwrap()
        .get("slots")
        .unwrap()
        .get("main")
        .unwrap()
        .get("*")
        .unwrap()
        .as_str()
        .unwrap()
        .split(" require 'mw.ext.LuaCache")
        .next()
        .unwrap()
        .replace("local cache =", "return info");

    let mut item_meta: Vec<ItemMeta> = Vec::new();

    let offsets = get_item_offsets()?;

    let lua = Lua::new();

    lua.context(|ctx| -> Result<()> {
        let info: Table = ctx
            .load(lua_str.as_str())
            .set_name("Iteminfo_data")
            .unwrap()
            .eval()?;

        for pair in info.pairs::<rlua::String, rlua::Value>() {
            let (key, val) = pair?;

            if let rlua::Value::Table(lua_item) = val {
                let id = i32::from_str(key.to_str()?)?;
                let name = lua_item.get("name").unwrap_or(String::new());
                let internal_name = lua_item.get("internalName").unwrap_or(String::new());
                let max_stack = lua_item.get("maxStack").unwrap_or(1);
                let width = lua_item.get("width").unwrap_or(0);
                let height = lua_item.get("height").unwrap_or(0);
                let value = lua_item.get("value").unwrap_or(0);
                #[allow(unused)] // but it's not unused tho
                let rarity = ItemRarity::from(lua_item.get("rare").unwrap_or(0));
                let use_time: Option<i32> = lua_item.get("useTime").ok();
                let damage: Option<i32> = lua_item.get("damage").ok();
                let crit: Option<i32> = lua_item.get("crit").ok();
                let knockback: Option<f32> = lua_item.get("knockBack").ok();
                let defense: Option<i32> = lua_item.get("defense").ok();
                let use_ammo: Option<i32> = lua_item.get("useAmmo").ok();
                let mana_cost: Option<i32> = lua_item.get("mana").ok();
                let heal_life: Option<i32> = lua_item.get("healLife").ok();
                let heal_mana: Option<i32> = lua_item.get("healMana").ok();
                let pickaxe_power: Option<i32> = lua_item.get("pick").ok();
                let axe_power: Option<i32> = lua_item.get("axe").ok();
                let hammer_power: Option<i32> = lua_item.get("hammer").ok();
                let fishing_power: Option<i32> = lua_item.get("fishingPole").ok();
                let fishing_bait: Option<i32> = lua_item.get("bait").ok();
                let range_boost: Option<i32> = lua_item.get("tileBoost").ok();
                let sacrifices = lua_item.get("sacrifices").unwrap_or(1);

                let tooltip = items["ItemTooltip"][&internal_name].as_str().map(|tt| {
                    tt.lines()
                        .map(|s| expand_templates(s.to_owned(), &template, &game, &items, &npcs))
                        .collect::<Vec<_>>()
                });

                let consumable = lua_item.get("consumable").unwrap_or(false);
                let expert = lua_item.get("expert").unwrap_or(false);
                let rarity = ItemRarity::from(lua_item.get("rarity").unwrap_or(0));

                let [x, y] = offsets.get(&id).unwrap_or(&[-1, -1]);
                let x = x.to_owned();
                let y = y.to_owned();

                let item = ItemMeta {
                    id,
                    internal_name,
                    name,
                    width,
                    height,
                    x,
                    y,
                    max_stack,
                    value,
                    rarity,
                    use_time,
                    damage,
                    crit,
                    knockback,
                    defense,
                    use_ammo,
                    mana_cost,
                    heal_life,
                    heal_mana,
                    pickaxe_power,
                    axe_power,
                    hammer_power,
                    fishing_power,
                    fishing_bait,
                    range_boost,
                    sacrifices,
                    tooltip,
                    consumable,
                    expert,
                };

                item_meta.push(item);
            }
        }

        Ok(())
    })?;

    item_meta.sort_by_key(|i| i.id);

    Ok(item_meta)
}

fn get_buff_meta(
    template: &Regex,
    game: &serde_json::Value,
    items: &serde_json::Value,
    npcs: &serde_json::Value,
) -> Result<Vec<BuffMeta>> {
    let resp = reqwest::blocking::get(BUFF_IDS_URL)?;
    let text = resp.text()?;

    let doc = scraper::Html::parse_document(&text);

    let tbody_selector = scraper::Selector::parse("table.terraria.sortable").unwrap();
    let tr_selector = scraper::Selector::parse("tbody>tr").unwrap();
    let td_selector = scraper::Selector::parse("td").unwrap();

    let internal_name_selector = scraper::Selector::parse("code").unwrap();
    let name_selector = scraper::Selector::parse("span.i>span>span>a").unwrap();
    let image_selector = scraper::Selector::parse("span.i>a>img").unwrap();

    let offsets = get_buff_offsets()?;

    let mut buff_meta: Vec<BuffMeta> = doc
        .select(&tbody_selector)
        .next()
        .unwrap()
        .select(&tr_selector)
        .skip(1)
        .map(|tr| {
            let mut tds = tr.select(&td_selector);

            let id = i32::from_str(tds.next().unwrap().inner_html().trim()).unwrap();

            let image_text = tds
                .next()
                .unwrap()
                .select(&image_selector)
                .next()
                .unwrap()
                .value()
                .attr("alt")
                .unwrap()
                .to_owned();

            let name = match tds.next().unwrap().select(&name_selector).next() {
                Some(a) => a.inner_html().trim().to_owned(),
                None => image_text,
            };

            let [x, y] = offsets.get(&id).unwrap_or(&[-1, -1]);
            let x = x.to_owned();
            let y = y.to_owned();

            let internal_name = tds
                .next()
                .unwrap()
                .select(&internal_name_selector)
                .next()
                .unwrap()
                .inner_html()
                .trim()
                .to_owned();

            let buff_type = match tds.next().unwrap().inner_html().trim() {
                "Buff" => BuffType::Buff,
                "Debuff" => BuffType::Debuff,
                _ => panic!("TF THIS?"),
            };

            let tooltip = game["BuffDescription"][&internal_name].as_str().map(|tt| {
                tt.lines()
                    .map(|s| expand_templates(s.to_owned(), template, game, items, npcs))
                    .collect::<Vec<_>>()
            });

            BuffMeta {
                id,
                name,
                x,
                y,
                internal_name,
                buff_type,
                tooltip,
                ..Default::default()
            }
        })
        .collect();

    buff_meta.sort_by_key(|b| b.id);

    Ok(buff_meta)
}

fn get_prefix_meta(
    _template: &Regex,
    _game: &serde_json::Value,
    items: &serde_json::Value,
    _npcs: &serde_json::Value,
) -> Result<Vec<PrefixMeta>> {
    let resp = reqwest::blocking::get(PREFIX_IDS_URL)?;
    let text = resp.text()?;

    let doc = scraper::Html::parse_document(&text);

    let tbody_selector = scraper::Selector::parse("table.terraria.sortable").unwrap();
    let tr_selector = scraper::Selector::parse("tbody>tr").unwrap();
    let td_selector = scraper::Selector::parse("td").unwrap();

    let mut prefix_meta: Vec<PrefixMeta> = doc
        .select(&tbody_selector)
        .next()
        .unwrap()
        .select(&tr_selector)
        .skip(1)
        .map(|tr| {
            let mut tds = tr.select(&td_selector);

            let id = u8::from_str(tds.next().unwrap().inner_html().trim()).unwrap();

            let internal_name = match id {
                // Edge Cases
                20 => "Deadly2".to_owned(),
                75 => "Hasty2".to_owned(),
                76 => "Quick2".to_owned(),
                84 => "Legendary2".to_owned(),
                90 => "Piercing".to_owned(),
                _ => tds.next().unwrap().inner_html().trim().to_owned(),
            };

            // Quick hack because Piercing is mobile only
            let name = if internal_name == "Piercing" {
                internal_name.clone()
            } else {
                items["Prefix"][&internal_name].as_str().unwrap().to_owned()
            };

            PrefixMeta {
                id,
                internal_name,
                name,
            }
        })
        .collect();

    prefix_meta.sort_by_key(|p| p.id);

    Ok(prefix_meta)
}

// fn generate_spritesheet(items_fol: &PathBuf) -> Result<DynamicImage> {
//     let iter = fs::read_dir(items_fol)?;

//     let mut items: HashMap<u32, DynamicImage> = HashMap::new();

//     const ITEM_WIDTH: u32 = 50;
//     const ITEM_HEIGHT: u32 = 50;
//     const H_ITEM_COUNT: u32 = 64;

//     for (i, item) in iter.enumerate() {
//         let item = item?;
//         let file_name = item.file_name();
//         let file_name = file_name.to_string_lossy().to_owned();

//         if !file_name.ends_with(".png") {
//             continue;
//         }

//         let item_id = u32::from_str(
//             file_name
//                 .rsplit_once(".")
//                 .unwrap()
//                 .0
//                 .rsplit_once("_")
//                 .unwrap()
//                 .1,
//         )?;

//         if i % 1000 == 0 {
//             println!("loaded {} items", i);
//         }

//         let mut image = image::open(item.path())?;
//         if image.width() > ITEM_WIDTH || image.height() > ITEM_HEIGHT {
//             image = image.resize(
//                 ITEM_WIDTH,
//                 ITEM_HEIGHT,
//                 image::imageops::FilterType::CatmullRom,
//             );
//         }

//         items.insert(item_id, image);
//     }

//     let highest_id = items.keys().max_by(|a, b| a.cmp(b)).unwrap();

//     let width = ITEM_WIDTH * H_ITEM_COUNT;
//     let height = (highest_id / H_ITEM_COUNT + 1) * ITEM_HEIGHT;

//     let mut new_image = DynamicImage::new_rgba8(width, height);

//     let mut count = 0;
//     for (i, img) in items.iter() {
//         let x = i % H_ITEM_COUNT * ITEM_WIDTH + (ITEM_WIDTH - img.width()) / 2;
//         let y = i / H_ITEM_COUNT * ITEM_HEIGHT + (ITEM_HEIGHT - img.height()) / 2;
//         new_image.copy_from(img, x, y)?;

//         count += 1;
//         if count % 1000 == 0 {
//             println!("{} items", count);
//         }
//     }

//     Ok(new_image)
// }

fn main() -> Result<()> {
    let res_fol = PathBuf::from_str("./terra-res/resources")?;
    // let items_fol = res_fol.join("items");
    let gen_fol = PathBuf::from_str("./terra-res/generated")?;

    create_dir(&gen_fol, true)?;

    let mut item_file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(false)
        .open(gen_fol.join("items.json"))?;
    let item_writer = BufWriter::new(&mut item_file);

    let mut buff_file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(false)
        .open(gen_fol.join("buffs.json"))?;
    let buff_writer = BufWriter::new(&mut buff_file);

    let mut prefix_file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(false)
        .open(gen_fol.join("prefixes.json"))?;
    let prefix_writer = BufWriter::new(&mut prefix_file);

    // let mut spritesheet_file = File::create(gen_fol.join("items.png"))?;
    // let mut spritesheet_writer = BufWriter::new(&mut spritesheet_file);

    let item_localization_file = File::open(res_fol.join("Items.json"))?;
    let npc_localization_file = File::open(res_fol.join("NPCs.json"))?;
    let game_localization_file = File::open(res_fol.join("Game.json"))?;

    let item_localization: serde_json::Value = serde_json::from_reader(item_localization_file)?;
    let npc_localization: serde_json::Value = serde_json::from_reader(npc_localization_file)?;
    let game_localization: serde_json::Value = serde_json::from_reader(game_localization_file)?;

    let template = Regex::new(r"\{\$([A-z\d]+)\.([A-z\d]+)\}")?;

    let item_meta = get_item_meta(
        &template,
        &game_localization,
        &item_localization,
        &npc_localization,
    )?;
    let buff_meta = get_buff_meta(
        &template,
        &game_localization,
        &item_localization,
        &npc_localization,
    )?;
    let prefix_meta = get_prefix_meta(
        &template,
        &game_localization,
        &item_localization,
        &npc_localization,
    )?;
    // let spritesheet = generate_spritesheet(&items_fol)?;

    // Pretty scuffed but works for now
    let mut build_type = String::new();
    File::open("./terra-res/build_type.txt")?.read_to_string(&mut build_type)?;

    if build_type == "debug" {
        serde_json::to_writer_pretty(item_writer, &item_meta)?;
        serde_json::to_writer_pretty(buff_writer, &buff_meta)?;
        serde_json::to_writer_pretty(prefix_writer, &prefix_meta)?;
    } else {
        serde_json::to_writer(item_writer, &item_meta)?;
        serde_json::to_writer(buff_writer, &buff_meta)?;
        serde_json::to_writer(prefix_writer, &prefix_meta)?;
    }
    // spritesheet.write_to(&mut spritesheet_writer, ImageFormat::Png)?;

    let target_dir = PathBuf::from("./target").join(&build_type);
    let final_dir = target_dir.join("resources");

    if final_dir.exists() {
        fs::remove_dir_all(&final_dir)?;
    }

    let mut options = DirCopyOptions::new();
    options.overwrite = true;
    options.copy_inside = true;
    copy_dir(&gen_fol, &final_dir, &options)?;

    fs::copy(res_fol.join("items.png"), final_dir.join("items.png"))?;
    fs::copy(res_fol.join("buffs.png"), final_dir.join("buffs.png"))?;

    Ok(())
}
