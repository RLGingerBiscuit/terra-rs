use std::{
    collections::HashMap,
    fs::{self, File, OpenOptions},
    io::{BufWriter, LineWriter, Read, Write},
    path::{Path, PathBuf},
    str::FromStr,
};

use anyhow::Result;
use image::{DynamicImage, GenericImage, ImageFormat};
use itertools::Itertools;
use regex::{Captures, Regex};

use fs_extra::dir::{copy as copy_dir, create as create_dir, CopyOptions as DirCopyOptions};

use rlua::{Lua, Table};
use terra_core::{BuffMeta, BuffType, ItemMeta, ItemRarity, PrefixMeta};

const ITEM_DATA_URL: &str = "https://terraria.wiki.gg/api.php?action=query&prop=revisions&format=json&rvlimit=1&rvslots=*&rvprop=content&titles=Module:Iteminfo/data";
const BUFF_IDS_URL: &str = "https://terraria.wiki.gg/wiki/Buff_IDs";
const PREFIX_IDS_URL: &str = "https://terraria.wiki.gg/wiki/Prefix_IDs";

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

fn get_offsets(path: &Path) -> Result<HashMap<i32, [i32; 2]>> {
    let offset_regex = Regex::new(r"^.*id='(\d+)'.*ofs: -(\d+)px -(\d+)")?;

    let mut text = String::new();
    File::open(path).unwrap().read_to_string(&mut text)?;

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
    offset_filepath: &Path,
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

    let offsets = get_offsets(offset_filepath)?;

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
    offset_filepath: &Path,
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

    let offsets = get_offsets(offset_filepath)?;

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

fn animated_items() -> HashMap<u32, [u32; 4]> {
    let mut map = HashMap::new();
    // TODO: This kinda sucks, but I just want to get this working, so here's an algo for doing this a little better
    // iterate through all the rows, taking note of the bounds for each row (first & last pixel)
    // and the first row where a non-transparent pixel was encountered, let that be miny
    // stop once all pixels are transparent or at the end, let the index of the final row be maxy
    // Find minx and maxx using the bounds
    // w = maxx - minx
    // h = maxy - miny
    // x = minx
    // y = miny
    map.insert(75, [22, 24, 0, 0]);
    map.insert(353, [18, 18, 0, 0]);
    map.insert(357, [30, 22, 0, 0]);
    map.insert(520, [22, 22, 0, 1]);
    map.insert(521, [22, 22, 0, 1]);
    map.insert(547, [22, 22, 0, 1]);
    map.insert(548, [22, 22, 0, 1]);
    map.insert(549, [22, 22, 0, 1]);
    map.insert(575, [22, 22, 0, 1]);
    map.insert(967, [12, 14, 0, 0]);
    map.insert(969, [12, 14, 0, 0]);
    map.insert(1787, [38, 25, 0, 0]);
    map.insert(1911, [28, 30, 0, 0]);
    map.insert(1912, [28, 32, 0, 0]);
    map.insert(1919, [26, 26, 0, 0]);
    map.insert(1920, [28, 32, 0, 0]);
    map.insert(2266, [18, 30, 0, 0]);
    map.insert(2267, [30, 22, 0, 0]);
    map.insert(2268, [30, 28, 0, 0]);
    map.insert(3195, [28, 24, 0, 0]);
    map.insert(3453, [16, 22, 0, 32]);
    map.insert(3454, [16, 22, 0, 32]);
    map.insert(3455, [16, 22, 0, 32]);
    map.insert(3532, [32, 30, 0, 0]);
    map.insert(3580, [22, 22, 0, 2]);
    map.insert(3581, [21, 20, 0, 4]);
    map.insert(4009, [24, 26, 0, 0]);
    map.insert(4010, [29, 20, 2, 0]);
    map.insert(4011, [34, 22, 0, 0]);
    map.insert(4012, [36, 28, 0, 0]);
    map.insert(4013, [30, 26, 0, 0]);
    map.insert(4014, [28, 24, 0, 0]);
    map.insert(4015, [28, 24, 0, 0]);
    map.insert(4016, [22, 24, 0, 0]);
    map.insert(4017, [30, 28, 0, 0]);
    map.insert(4018, [16, 28, 0, 0]);
    map.insert(4019, [30, 18, 0, 0]);
    map.insert(4020, [28, 16, 0, 0]);
    map.insert(4021, [24, 28, 0, 0]);
    map.insert(4022, [30, 28, 0, 0]);
    map.insert(4023, [22, 28, 0, 0]);
    map.insert(4024, [26, 24, 0, 0]);
    map.insert(4025, [32, 16, 0, 0]);
    map.insert(4026, [28, 34, 0, 0]);
    map.insert(4027, [24, 40, 0, 0]);
    map.insert(4028, [29, 28, 0, 0]);
    map.insert(4029, [28, 26, 0, 0]);
    map.insert(4030, [32, 32, 0, 0]);
    map.insert(4031, [30, 18, 0, 0]);
    map.insert(4032, [28, 22, 0, 0]);
    map.insert(4033, [30, 16, 0, 0]);
    map.insert(4034, [34, 20, 0, 0]);
    map.insert(4035, [34, 26, 0, 0]);
    map.insert(4036, [30, 18, 0, 0]);
    map.insert(4037, [34, 22, 0, 0]);
    map.insert(4068, [14, 18, 2, 4]);
    map.insert(4069, [18, 18, 2, 2]);
    map.insert(4070, [14, 18, 2, 2]);
    map.insert(4282, [22, 26, 0, 0]);
    map.insert(4283, [26, 30, 0, 0]);
    map.insert(4284, [20, 28, 0, 0]);
    map.insert(4285, [22, 24, 0, 0]);
    map.insert(4286, [22, 32, 0, 0]);
    map.insert(4287, [24, 24, 0, 0]);
    map.insert(4288, [26, 26, 0, 0]);
    map.insert(4289, [28, 30, 0, 0]);
    map.insert(4290, [26, 24, 0, 0]);
    map.insert(4291, [18, 28, 0, 0]);
    map.insert(4292, [28, 24, 0, 0]);
    map.insert(4293, [22, 24, 0, 0]);
    map.insert(4294, [20, 32, 0, 0]);
    map.insert(4295, [24, 28, 0, 0]);
    map.insert(4296, [30, 32, 0, 0]);
    map.insert(4297, [24, 24, 0, 0]);
    map.insert(4403, [30, 20, 0, 2]);
    map.insert(4411, [32, 22, 0, 0]);
    map.insert(4614, [14, 26, 0, 6]);
    map.insert(4615, [14, 26, 0, 6]);
    map.insert(4616, [16, 28, 0, 4]);
    map.insert(4617, [20, 32, 0, 2]);
    map.insert(4618, [20, 28, 0, 4]);
    map.insert(4619, [14, 30, 0, 2]);
    map.insert(4620, [10, 30, 0, 2]);
    map.insert(4621, [18, 32, 0, 0]);
    map.insert(4622, [18, 30, 0, 2]);
    map.insert(4623, [20, 36, 0, 4]);
    map.insert(4624, [18, 26, 0, 6]);
    map.insert(4625, [18, 26, 0, 2]);
    map.insert(5009, [34, 22, 0, 2]);
    map.insert(5041, [24, 38, 0, 2]);
    map.insert(5042, [32, 34, 0, 0]);
    map.insert(5092, [36, 32, 0, 0]);
    map.insert(5093, [36, 28, 0, 0]);
    map.insert(5275, [16, 28, 0, 0]);
    map.insert(5277, [20, 24, 0, 0]);
    map.insert(5278, [20, 22, 2, 2]);
    map
}

fn generate_spritesheet(
    fol: &Path,
    text: &str,
    max_width: u32,
    offset_filepath: &Path,
) -> Result<DynamicImage> {
    // TODO: Scale sprites by 1/2
    let animated_items = animated_items();

    let iter = fs::read_dir(fol)?;

    let mut sprites: HashMap<u32, DynamicImage> = HashMap::new();

    let file = File::create(offset_filepath)?;
    let mut writer = LineWriter::new(file);

    const SPRITE_SPACING: u32 = 2;

    for (i, sprite) in iter.enumerate() {
        let item = sprite?;
        let filename = item.file_name();
        let filename = filename.to_string_lossy().to_owned();

        if !filename.ends_with(".png") {
            continue;
        }

        let id = filename.rsplit_once(".").unwrap().0;
        let id = id
            .rsplit_once("_")
            .unwrap_or_else(|| {
                println!("File {} doesn't have an id, assuming 0", filename);
                ("", "0")
            })
            .1;
        let id = u32::from_str(id)?;

        if i % 1000 == 0 {
            println!("loaded {} sprites", i);
        }

        let image = image::open(item.path())?;

        sprites.insert(id, image);
    }

    let sprite_iter = sprites
        .iter()
        .sorted_by(|(a, _), (b, _)| a.cmp(b))
        .sorted_by(|(i, a), (j, b)| {
            let (a, b) = if text == "slot" {
                (
                    if let Some([_, a, _, _]) = animated_items.get(i) {
                        a.to_owned()
                    } else {
                        a.height()
                    },
                    if let Some([_, b, _, _]) = animated_items.get(j) {
                        b.to_owned()
                    } else {
                        b.height()
                    },
                )
            } else {
                (a.height(), b.height())
            };

            a.cmp(&b)
        });

    let mut running_x = SPRITE_SPACING;
    let mut running_y = SPRITE_SPACING;

    let mut final_height = 0;
    let mut largest_width = 0;
    let mut largest_height = 0;

    for (i, image) in sprite_iter.as_ref() {
        let (width, height) = if text == "slot" {
            if let Some([width, height, _, _]) = animated_items.get(i) {
                (width.to_owned(), height.to_owned())
            } else {
                (image.width(), image.height())
            }
        } else {
            (image.width(), image.height())
        };

        if running_x + width > max_width {
            running_x = SPRITE_SPACING;
            running_y += largest_height + SPRITE_SPACING * 2;
            final_height = running_y + height + SPRITE_SPACING;
        }

        if width > largest_width {
            largest_width = width;
        }
        if height > largest_height {
            largest_height = height;
            final_height = running_y + height + SPRITE_SPACING;
        }

        running_x += SPRITE_SPACING + width + SPRITE_SPACING;
    }

    let mut new_image = DynamicImage::new_rgba8(max_width, final_height);

    largest_width = 0;
    largest_height = 0;
    running_x = SPRITE_SPACING;
    running_y = SPRITE_SPACING;

    let mut count = 0;

    sprite_iter
        .for_each(|(i, image)| {
            let (width, height) = if text == "slot" {
                if let Some([width, height, _, _]) = animated_items.get(i) {
                    (width.to_owned(), height.to_owned())
                } else {
                    (image.width(), image.height())
                }
            } else {
                (image.width(), image.height())
            };

            if running_x + width > max_width {
                running_y += largest_height + SPRITE_SPACING * 2;
                running_x = SPRITE_SPACING;
            }

            if width > largest_width {
                largest_width = width;
            }
            if height > largest_height {
                largest_height = height;
            }

            writeln!(writer, ".{text}[data-id='{i}'] {{ --ofs: -{running_x}px -{running_y}px; --w: {width}px; --h: {height}px; }}").expect("Wut");

            new_image.copy_from(image, running_x, running_y).expect("Wut");

            running_x += SPRITE_SPACING + width + SPRITE_SPACING;

            count += 1;
            if count % 1000 == 0 {
                println!("{} sprites", count);
            }
        });

    Ok(new_image)
}

fn main() -> Result<()> {
    let res_fol = PathBuf::from_str("./terra-res/resources")?;
    let items_fol = res_fol.join("items");
    let buffs_fol = res_fol.join("buffs");
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

    let mut item_spritesheet_file = File::create(gen_fol.join("items.png"))?;
    let mut item_spritesheet_writer = BufWriter::new(&mut item_spritesheet_file);

    let mut buff_spritesheet_file = File::create(gen_fol.join("buffs.png"))?;
    let mut buff_spritesheet_writer = BufWriter::new(&mut buff_spritesheet_file);

    let item_localization_file = File::open(res_fol.join("Items.json"))?;
    let npc_localization_file = File::open(res_fol.join("NPCs.json"))?;
    let game_localization_file = File::open(res_fol.join("Game.json"))?;

    let item_localization: serde_json::Value = serde_json::from_reader(item_localization_file)?;
    let npc_localization: serde_json::Value = serde_json::from_reader(npc_localization_file)?;
    let game_localization: serde_json::Value = serde_json::from_reader(game_localization_file)?;

    let template = Regex::new(r"\{\$([A-z\d]+)\.([A-z\d]+)\}")?;

    let item_offset_filepath = gen_fol.join("items.css");
    let buff_offset_filepath = gen_fol.join("buffs.css");

    let item_spritesheet = generate_spritesheet(&items_fol, "slot", 2560, &item_offset_filepath)?;
    let buff_spritesheet = generate_spritesheet(&buffs_fol, "buff", 512, &buff_offset_filepath)?;

    let item_meta = get_item_meta(
        &template,
        &game_localization,
        &item_localization,
        &npc_localization,
        &item_offset_filepath,
    )?;
    let buff_meta = get_buff_meta(
        &template,
        &game_localization,
        &item_localization,
        &npc_localization,
        &buff_offset_filepath,
    )?;
    let prefix_meta = get_prefix_meta(
        &template,
        &game_localization,
        &item_localization,
        &npc_localization,
    )?;

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
    item_spritesheet.write_to(&mut item_spritesheet_writer, ImageFormat::Png)?;
    buff_spritesheet.write_to(&mut buff_spritesheet_writer, ImageFormat::Png)?;

    let target_dir = PathBuf::from("./target").join(&build_type);
    let final_dir = target_dir.join("resources");

    if final_dir.exists() {
        fs::remove_dir_all(&final_dir)?;
    }

    let mut options = DirCopyOptions::new();
    options.overwrite = true;
    options.copy_inside = true;
    copy_dir(&gen_fol, &final_dir, &options)?;

    Ok(())
}
