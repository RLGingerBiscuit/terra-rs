use std::{
    collections::HashMap,
    fs::{self, File, OpenOptions},
    io::{BufWriter, LineWriter, Read, Write},
    path::{Path, PathBuf},
    str::FromStr,
};

use fs_extra::dir::{
    copy as copy_dir, create_all as create_dir_all, CopyOptions as DirCopyOptions,
};

use anyhow::Result;
use image::{DynamicImage, GenericImage, GenericImageView};
use itertools::Itertools;
use mlua::Lua;
use regex::{Captures, Regex};

use reqwest::{
    blocking::Client,
    header::{HeaderMap, HeaderValue},
};
use terra_core::{BuffMeta, BuffType, ItemMeta, ItemRarity, ItemType, PrefixMeta, SharedString};

mod truthy;
use truthy::TruthyOption;

const ITEM_DATA_URL: &str = "https://terraria.wiki.gg/api.php?action=query&prop=revisions&format=json&rvlimit=1&rvslots=*&rvprop=content&titles=Module:Iteminfo/luadata";
const BUFF_IDS_URL: &str = "https://terraria.wiki.gg/wiki/Buff_IDs";
const PREFIX_IDS_URL: &str = "https://terraria.wiki.gg/wiki/Prefix_IDs";
const TRAPPED_CHEST_DOT: &str = "trapped_chest_dot.png";

fn get_offsets(path: &Path) -> Result<HashMap<i32, [i32; 4]>> {
    let offset_regex =
        Regex::new(r"^.*id='(\d+)'.*ofs: -(\d+)px -(\d+)px; --w: (\d+)px; --h: (\d+)")?;

    let mut text: String = String::new();
    File::open(path).unwrap().read_to_string(&mut text)?;

    let mut offsets = HashMap::new();

    text.lines().for_each(|line| {
        let captures = offset_regex.captures(line).unwrap();
        let id = i32::from_str(captures.get(1).unwrap().as_str()).unwrap();
        let x = i32::from_str(captures.get(2).unwrap().as_str()).unwrap();
        let y = i32::from_str(captures.get(3).unwrap().as_str()).unwrap();
        let w = i32::from_str(captures.get(4).unwrap().as_str()).unwrap();
        let h = i32::from_str(captures.get(5).unwrap().as_str()).unwrap();

        offsets.insert(id, [x, y, w, h]);
    });

    Ok(offsets)
}

fn animated_items() -> HashMap<i32, [u32; 4]> {
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

    // These are halved, since Terraria's sprites' pixels are 2x2
    map.insert(75, [11, 12, 0, 0]);
    map.insert(353, [9, 9, 0, 0]);
    map.insert(357, [15, 11, 0, 0]);
    map.insert(520, [11, 11, 0, 1]);
    map.insert(521, [11, 11, 0, 1]);
    map.insert(547, [11, 11, 0, 1]);
    map.insert(548, [11, 11, 0, 1]);
    map.insert(549, [11, 11, 0, 1]);
    map.insert(575, [11, 11, 0, 1]);
    map.insert(967, [6, 7, 0, 0]);
    map.insert(969, [6, 7, 0, 0]);
    map.insert(1787, [19, 12, 0, 0]);
    map.insert(1911, [14, 15, 0, 0]);
    map.insert(1912, [14, 16, 0, 0]);
    map.insert(1919, [13, 13, 0, 0]);
    map.insert(1920, [14, 16, 0, 0]);
    map.insert(2266, [9, 15, 0, 0]);
    map.insert(2267, [15, 11, 0, 0]);
    map.insert(2268, [15, 14, 0, 0]);
    map.insert(2425, [15, 8, 0, 0]);
    map.insert(2426, [17, 8, 0, 0]);
    map.insert(2427, [15, 8, 0, 0]);
    map.insert(3195, [14, 12, 0, 0]);
    map.insert(3453, [7, 10, 1, 3]);
    map.insert(3454, [7, 10, 1, 3]);
    map.insert(3455, [7, 10, 1, 3]);
    map.insert(3532, [16, 15, 0, 0]);
    map.insert(3580, [11, 11, 0, 1]);
    map.insert(3581, [10, 10, 0, 2]);
    map.insert(4009, [12, 13, 0, 0]);
    map.insert(4010, [14, 10, 1, 0]);
    map.insert(4011, [17, 11, 0, 0]);
    map.insert(4012, [18, 14, 0, 0]);
    map.insert(4013, [15, 13, 0, 0]);
    map.insert(4014, [14, 12, 0, 0]);
    map.insert(4015, [14, 12, 0, 0]);
    map.insert(4016, [11, 12, 0, 0]);
    map.insert(4017, [15, 14, 0, 0]);
    map.insert(4018, [8, 14, 0, 0]);
    map.insert(4019, [15, 9, 0, 0]);
    map.insert(4020, [14, 8, 0, 0]);
    map.insert(4021, [12, 14, 0, 0]);
    map.insert(4022, [15, 14, 0, 0]);
    map.insert(4023, [11, 14, 0, 0]);
    map.insert(4024, [13, 12, 0, 0]);
    map.insert(4025, [16, 8, 0, 0]);
    map.insert(4026, [14, 17, 0, 0]);
    map.insert(4027, [12, 20, 0, 0]);
    map.insert(4028, [14, 14, 0, 0]);
    map.insert(4029, [14, 13, 0, 0]);
    map.insert(4030, [16, 16, 0, 0]);
    map.insert(4031, [15, 9, 0, 0]);
    map.insert(4032, [14, 11, 0, 0]);
    map.insert(4033, [15, 8, 0, 0]);
    map.insert(4034, [17, 10, 0, 0]);
    map.insert(4035, [17, 13, 0, 0]);
    map.insert(4036, [15, 8, 0, 1]);
    map.insert(4037, [17, 11, 0, 0]);
    map.insert(4068, [7, 9, 2, 1]);
    map.insert(4069, [9, 9, 1, 1]);
    map.insert(4070, [7, 9, 1, 1]);
    map.insert(4282, [11, 13, 0, 0]);
    map.insert(4283, [13, 15, 0, 0]);
    map.insert(4284, [10, 14, 0, 0]);
    map.insert(4285, [11, 12, 0, 0]);
    map.insert(4286, [11, 16, 0, 0]);
    map.insert(4287, [12, 12, 0, 0]);
    map.insert(4288, [13, 13, 0, 0]);
    map.insert(4289, [14, 15, 0, 0]);
    map.insert(4290, [13, 12, 0, 0]);
    map.insert(4291, [9, 14, 0, 0]);
    map.insert(4292, [14, 12, 0, 0]);
    map.insert(4293, [11, 12, 0, 0]);
    map.insert(4294, [10, 16, 0, 0]);
    map.insert(4295, [12, 14, 0, 0]);
    map.insert(4296, [15, 16, 0, 0]);
    map.insert(4297, [12, 12, 0, 0]);
    map.insert(4403, [15, 10, 0, 1]);
    map.insert(4411, [16, 11, 0, 0]);
    map.insert(4614, [7, 13, 0, 3]);
    map.insert(4615, [7, 13, 0, 3]);
    map.insert(4616, [8, 14, 0, 2]);
    map.insert(4617, [10, 15, 0, 1]);
    map.insert(4618, [10, 14, 0, 2]);
    map.insert(4619, [7, 15, 0, 1]);
    map.insert(4620, [9, 15, 0, 1]);
    map.insert(4621, [9, 16, 0, 0]);
    map.insert(4622, [9, 15, 0, 1]);
    map.insert(4623, [10, 18, 0, 2]);
    map.insert(4624, [9, 13, 0, 3]);
    map.insert(4625, [14, 13, 0, 1]);
    map.insert(5009, [17, 11, 0, 1]);
    map.insert(5041, [12, 19, 0, 1]);
    map.insert(5042, [16, 12, 0, 0]);
    map.insert(5092, [18, 16, 0, 0]);
    map.insert(5093, [18, 14, 0, 0]);
    map.insert(5275, [8, 14, 0, 0]);
    map.insert(5277, [10, 12, 0, 0]);
    map.insert(5278, [10, 11, 1, 1]);

    map
}

fn trapped_chests() -> HashMap<i32, i32> {
    let mut map = HashMap::new();
    map.insert(3665, 48);
    map.insert(3666, 306);
    map.insert(3667, 328);
    map.insert(3668, 625);
    map.insert(3669, 626);
    map.insert(3670, 627);
    map.insert(3671, 680);
    map.insert(3672, 681);
    map.insert(3673, 831);
    map.insert(3674, 838);
    map.insert(3675, 914);
    map.insert(3676, 952);
    map.insert(3677, 1142);
    map.insert(3678, 1298);
    map.insert(3679, 1528);
    map.insert(3680, 1529);
    map.insert(3681, 1530);
    map.insert(3682, 1531);
    map.insert(3683, 1532);
    map.insert(3684, 2230);
    map.insert(3685, 2249);
    map.insert(3686, 2250);
    map.insert(3687, 2526);
    map.insert(3688, 2544);
    map.insert(3689, 2559);
    map.insert(3690, 2574);
    map.insert(3691, 2612);
    map.insert(3692, 2613);
    map.insert(3693, 2614);
    map.insert(3694, 2615);
    map.insert(3695, 2616);
    map.insert(3696, 2617);
    map.insert(3697, 2618);
    map.insert(3698, 2619);
    map.insert(3699, 2620);
    map.insert(3700, 2748);
    map.insert(3701, 2814);
    map.insert(3702, 3125);
    map.insert(3703, 3180);
    map.insert(3704, 3181);
    map.insert(3705, 48); // Unobtainable
    map.insert(3706, 48); // Unobtainable
    map.insert(3886, 3884);
    map.insert(3887, 3885);
    map.insert(3950, 3939);
    map.insert(3976, 3965);
    map.insert(4164, 4153);
    map.insert(4185, 4174);
    map.insert(4206, 4195);
    map.insert(4227, 4216);
    map.insert(4266, 4265);
    map.insert(4268, 4267);
    map.insert(4585, 4574);
    map.insert(4713, 4712);
    map.insert(5167, 5156);
    map.insert(5188, 5177);
    map.insert(5209, 5198);
    map
}

fn forbidden_items() -> Vec<i32> {
    let mut forbidden_items = Vec::new();
    forbidden_items.push(0);
    forbidden_items.push(2772);
    forbidden_items.push(2773);
    forbidden_items.push(2775);
    forbidden_items.push(2777);
    forbidden_items.push(2778);
    forbidden_items.push(2780);
    forbidden_items.push(2782);
    forbidden_items.push(2783);
    forbidden_items.push(2785);
    forbidden_items.push(2881);
    forbidden_items.push(2903);
    forbidden_items.push(2989);
    forbidden_items.push(2990);
    forbidden_items.push(2991);
    forbidden_items.push(3331);
    forbidden_items.push(3398);
    forbidden_items.push(3404);
    forbidden_items.push(3462);
    forbidden_items.push(3463);
    forbidden_items.push(3465);
    forbidden_items.push(3705);
    forbidden_items.push(3706);
    forbidden_items.push(3847);
    forbidden_items.push(3848);
    forbidden_items.push(3849);
    forbidden_items.push(3850);
    forbidden_items.push(3851);
    forbidden_items.push(3853);
    forbidden_items.push(3861);
    forbidden_items.push(3862);
    forbidden_items.push(3978);
    forbidden_items.push(4058);
    forbidden_items.push(4143);
    forbidden_items.push(4722);
    forbidden_items.push(5013);
    forbidden_items
}

fn expand_templates(
    s: &str,
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
        expand_templates(&expanded, template, game, items, npcs)
    } else {
        expanded
            .replace("<right>", "Right Click")
            .replace("<left>", "Left Click")
    }
}

fn get_item_type(lua_item: &mlua::Table) -> Option<ItemType> {
    for (name, item_type) in [
        ("ammo", ItemType::Ammo),
        ("melee", ItemType::Melee),
        ("ranged", ItemType::Ranged),
        ("magic", ItemType::Magic),
        ("summon", ItemType::Summon),
        ("accessory", ItemType::Accessory),
        ("vanity", ItemType::Vanity),
        ("vanity", ItemType::Vanity),
    ] {
        if let Some(b) = lua_item.get(name).ok() {
            if b {
                return Some(item_type);
            }
        }
    }

    for (name, item_type) in [
        ("createTile", ItemType::Tile),
        ("createWall", ItemType::Wall),
        ("headSlot", ItemType::HeadArmor),
        ("bodySlot", ItemType::BodyArmor),
        ("legsSlot", ItemType::LegArmor),
    ] {
        if let Some(n) = lua_item.get::<i32>(name).ok() {
            if n >= 0 {
                return Some(item_type);
            }
        }
    }

    None
}

fn get_item_meta(
    client: &Client,
    template: &Regex,
    game: &serde_json::Value,
    items: &serde_json::Value,
    npcs: &serde_json::Value,
    offset_filepath: &Path,
) -> Result<Vec<ItemMeta>> {
    // https://terraria.wiki.gg/wiki/Module:Iteminfo/luadata
    // API: https://terraria.wiki.gg/api.php?action=query&prop=revisions&format=json&rvlimit=1&rvslots=*&rvprop=content&titles=Module:Iteminfo/luadata

    let lua_resp = client.get(ITEM_DATA_URL).send()?;

    let lua_resp = lua_resp.error_for_status()?;
    let lua_json: serde_json::Value = lua_resp.json()?;

    // Yeesh
    let lua_str = lua_json
        .as_object()
        .unwrap()
        .get("query")
        .unwrap()
        .get("pages")
        .unwrap()
        .get("65908")
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
        .unwrap();

    let mut item_meta: Vec<ItemMeta> = Vec::new();

    let offsets = get_offsets(offset_filepath)?;
    let forbidden_items = forbidden_items();

    let lua = Lua::new();
    let info: mlua::Table = lua.load(lua_str).set_name("Iteminfo_luadata").eval()?;

    for pair in info.pairs::<mlua::String, mlua::Value>() {
        let (key, val) = pair?;

        let mlua::Value::Table(lua_item) = val else {
            continue;
        };

        let id = i32::from_str(&key.to_str()?)?;
        let internal_name = lua_item.get("internalName").unwrap_or(String::new());
        let name = items["ItemName"][&internal_name]
            .as_str()
            .unwrap_or_else(|| &internal_name);
        let name = expand_templates(name, template, game, items, npcs);
        let max_stack = lua_item.get("maxStack").unwrap_or(1);
        let value = lua_item.get("value").unwrap_or(0);
        let use_time = lua_item.get("useTime").truthy_option();
        let damage = lua_item.get("damage").truthy_option();
        let crit_chance = lua_item.get("crit").truthy_option();
        let knockback = lua_item.get("knockBack").truthy_option();
        let defense = lua_item.get("defense").truthy_option();
        let use_ammo = lua_item.get("useAmmo").truthy_option();
        let mana_cost = lua_item.get("mana").truthy_option();
        let heal_life = lua_item.get("healLife").truthy_option();
        let heal_mana = lua_item.get("healMana").truthy_option();
        let pickaxe_power = lua_item.get("pick").truthy_option();
        let axe_power = lua_item.get("axe").truthy_option();
        let hammer_power = lua_item.get("hammer").truthy_option();
        let fishing_power = lua_item.get("fishingPole").truthy_option();
        let fishing_bait = lua_item.get("bait").truthy_option();
        let range_boost = lua_item.get("tileBoost").truthy_option();
        let sacrifices = lua_item.get("sacrifices").unwrap_or(1);

        let tooltip = items["ItemTooltip"][&internal_name].as_str().map(|tt| {
            tt.lines()
                .map(|s| expand_templates(&s, &template, &game, &items, &npcs))
                .collect::<Vec<_>>()
        });

        let forbidden = if forbidden_items.contains(&id) {
            Some(true)
        } else {
            None
        };

        let consumes_tile = lua_item.get("tileWand").truthy_option();

        let item_type = get_item_type(&lua_item);

        let is_material = lua_item.get("material").truthy_option();
        let is_consumable = lua_item.get("consumable").truthy_option();
        let is_quest_item = lua_item.get("questItem").truthy_option();
        let is_expert = lua_item.get("expert").truthy_option();

        let rarity = if is_expert.is_some_and(|e| e) {
            ItemRarity::Expert
        } else if let Ok(rarity) = lua_item.get::<i32>("rarity") {
            ItemRarity::from(rarity)
        } else {
            ItemRarity::from(lua_item.get("rare").unwrap_or(0))
        };

        let [x, y, width, height] = offsets.get(&id).unwrap_or(&[-1, -1, 0, 0]);
        let x = x.to_owned();
        let y = y.to_owned();
        let width = width.to_owned();
        let height = height.to_owned();

        let item = ItemMeta {
            id,
            internal_name: internal_name.into(),
            name: name.into(),
            width,
            height,
            x,
            y,
            max_stack,
            value,
            rarity,
            use_time,
            damage,
            crit_chance,
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
            tooltip: tooltip.map(|t| t.into_iter().map(|l| l.into()).collect_vec()),
            forbidden,
            consumes_tile,
            is_material,
            item_type,
            is_consumable,
            is_quest_item,
            is_expert,
        };

        item_meta.push(item);
    }

    item_meta.sort_by_key(|i| i.id);

    Ok(item_meta)
}

fn get_buff_meta(
    client: &Client,
    template: &Regex,
    game: &serde_json::Value,
    items: &serde_json::Value,
    npcs: &serde_json::Value,
    offset_filepath: &Path,
) -> Result<Vec<BuffMeta>> {
    let resp = client.get(BUFF_IDS_URL).send()?;
    let text = resp.text()?;

    let doc = scraper::Html::parse_document(&text);

    let tbody_selector = scraper::Selector::parse("table.terraria.sortable").unwrap();
    let tr_selector = scraper::Selector::parse("tbody>tr").unwrap();
    let td_selector = scraper::Selector::parse("td").unwrap();

    let internal_name_selector = scraper::Selector::parse("code").unwrap();
    let name_selector = scraper::Selector::parse("span.i>span>span>a").unwrap();
    let image_selector = scraper::Selector::parse("span.i>a>img").unwrap();

    let offsets = get_offsets(offset_filepath)?;

    let mut buff_meta = Vec::new();

    doc.select(&tbody_selector)
        .next()
        .unwrap()
        .select(&tr_selector)
        .skip(1)
        .for_each(|tr| {
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

            let [x, y, _, _] = offsets.get(&id).unwrap_or(&[-1, -1, 0, 0]);
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
                    .map(|s| expand_templates(&s, template, game, items, npcs))
                    .collect::<Vec<_>>()
            });

            buff_meta.push(BuffMeta {
                id,
                name: name.into(),
                x,
                y,
                internal_name: internal_name.into(),
                buff_type,
                tooltip: tooltip.map(|t| t.into_iter().map(|l| l.into()).collect_vec()),
            });
        });

    {
        let [x, y, _, _] = offsets.get(&0).unwrap_or(&[-1, -1, 0, 0]);
        let x = x.to_owned();
        let y = y.to_owned();
        buff_meta.push(BuffMeta {
            id: 0,
            name: SharedString::default(),
            internal_name: SharedString::new("None"),
            x,
            y,
            buff_type: BuffType::Buff,
            tooltip: None,
        });
    }

    buff_meta.sort_by_key(|b| b.id);

    Ok(buff_meta)
}

fn get_prefix_meta(
    client: &Client,
    _template: &Regex,
    _game: &serde_json::Value,
    items: &serde_json::Value,
    _npcs: &serde_json::Value,
) -> Result<Vec<PrefixMeta>> {
    let resp = client.get(PREFIX_IDS_URL).send()?;
    let text = resp.text()?;

    let doc = scraper::Html::parse_document(&text);

    let tbody_selector = scraper::Selector::parse("table.terraria.sortable").unwrap();
    let tr_selector = scraper::Selector::parse("tbody>tr").unwrap();
    let td_selector = scraper::Selector::parse("td").unwrap();

    let mut prefix_meta = Vec::new();

    doc.select(&tbody_selector)
        .next()
        .unwrap()
        .select(&tr_selector)
        .skip(1)
        .for_each(|tr| {
            let mut tds = tr.select(&td_selector);

            let id = u8::from_str(tds.next().unwrap().inner_html().trim()).unwrap();

            // NOTE: We're ignoring the mobile-only prefix 'Piercing' here
            if id == 90 {
                return;
            }

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

            prefix_meta.push(PrefixMeta {
                id,
                internal_name: internal_name.into(),
                name: name.into(),
            });
        });

    prefix_meta.push(PrefixMeta {
        id: 0,
        name: SharedString::default(),
        internal_name: SharedString::new("None"),
    });

    prefix_meta.sort_by_key(|p| p.id);

    Ok(prefix_meta)
}

fn generate_spritesheet(
    fol: &Path,
    res_fol: &Path,
    text: &str,
    max_width: u32,
    offset_filepath: &Path,
) -> Result<DynamicImage> {
    let animated_items = animated_items();
    let trapped_chests = trapped_chests();

    let iter = fs::read_dir(fol)?;

    let mut sprites: HashMap<i32, DynamicImage> = HashMap::new();

    let file = File::create(offset_filepath)?;
    let mut writer = LineWriter::new(file);

    const SPRITE_SPACING: u32 = 1;

    let last_sprite_id = iter
        .map(|z| {
            let z = z.unwrap().file_name();
            let z = z.to_string_lossy();
            if !z.ends_with(".png") {
                -1
            } else {
                let z = z
                    .rsplit_once(".")
                    .unwrap()
                    .0
                    .rsplit_once("_")
                    .unwrap_or_else(|| {
                        println!("File {} doesn't have an id, assuming 0", z);
                        ("", "0")
                    })
                    .1;

                i32::from_str(z).unwrap()
            }
        })
        .sorted()
        .last()
        .unwrap();

    let dot_sprite = image::open(res_fol.join(TRAPPED_CHEST_DOT))?;

    for i in -1..=last_sprite_id {
        let prefix = if text == "slot" { "Item" } else { "Buff" }.to_owned();

        let id = i.clamp(0, i32::MAX);

        let sprite_path = if i == -1 {
            prefix + ".png"
        } else {
            let sprite_id = trapped_chests.get(&id).unwrap_or(&id);
            format!("{prefix}_{sprite_id}.png")
        };
        let sprite_path = fol.join(sprite_path);

        if !Path::new(&sprite_path).exists() {
            continue;
        }

        let mut sprite = image::open(sprite_path)?;

        if trapped_chests.contains_key(&id) {
            let mut new_sprite = DynamicImage::new_rgba8(sprite.width() + 2, sprite.height() + 4);
            new_sprite.copy_from(&sprite, 0, 0)?;
            let start_x = new_sprite.width() - dot_sprite.width();
            let start_y = new_sprite.height() - dot_sprite.height();
            for (x, y, pixel) in dot_sprite.pixels() {
                let x = start_x + x;
                let y = start_y + y;
                // If pixel isn't transparent, replace it
                if pixel.0[3] != 0 {
                    new_sprite.put_pixel(x, y, pixel);
                }
            }
            sprite = new_sprite;
        }

        // Terraria's sprites' have 2x2 pixels, so we halve them in the name of size
        sprite = sprite.resize(
            sprite.width() / 2,
            sprite.height() / 2,
            image::imageops::FilterType::Nearest,
        );

        sprites.insert(id, sprite);
    }

    let sprite_iter = sprites
        .iter()
        .sorted_by(|(a, _), (b, _)| a.cmp(b))
        .sorted_by(|(i, a), (j, b)| {
            let (mut a, mut b) = if text == "slot" {
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

            if trapped_chests.contains_key(i) {
                a += 4;
            }
            if trapped_chests.contains_key(j) {
                b += 4;
            }

            a.cmp(&b)
        });

    let mut running_x = SPRITE_SPACING;
    let mut running_y = SPRITE_SPACING;

    let mut final_width = 0;
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
            if running_x > final_width {
                final_width = running_x;
            }
            running_x = 0;
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

    let mut final_image = DynamicImage::new_rgba8(final_width, final_height);

    largest_width = 0;
    largest_height = 0;
    running_x = SPRITE_SPACING;
    running_y = SPRITE_SPACING;

    let mut count = 0;

    sprite_iter
        .for_each(|(i, image)| {
            let (width, height, x, y) = if text == "slot" {
                match animated_items.get(i) {
                    Some([w,h,x,y]) => (w.to_owned(), h.to_owned(), x.to_owned(), y.to_owned()),
                    None => (image.width(), image.height(), 0, 0)
                }
            } else {
                (image.width(), image.height(), 0, 0)
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

            let view = image.view(x, y, width, height);

            final_image.copy_from(&*view, running_x, running_y).expect("Wut");

            running_x += SPRITE_SPACING + width + SPRITE_SPACING;

            count += 1;
            if count % 1000 == 0 {
                println!("{} sprites", count);
            }
        });

    Ok(final_image)
}

fn main() -> Result<()> {
    // Pretty scuffed but works for now
    let mut build_type = String::new();

    let terra_res_fol = PathBuf::from_str("crates/terra-res")?;

    File::open(terra_res_fol.join("build_type.txt"))?.read_to_string(&mut build_type)?;

    let res_fol = terra_res_fol.join("resources");
    let items_fol = res_fol.join("items");
    let buffs_fol = res_fol.join("buffs");
    let gen_fol = terra_res_fol.join("generated").join(&build_type);

    create_dir_all(&gen_fol, true)?;

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

    let item_localization_file = File::open(res_fol.join("Items.json"))?;
    let npc_localization_file = File::open(res_fol.join("NPCs.json"))?;
    let game_localization_file = File::open(res_fol.join("Game.json"))?;

    let item_localization: serde_json::Value = serde_json::from_reader(item_localization_file)?;
    let npc_localization: serde_json::Value = serde_json::from_reader(npc_localization_file)?;
    let game_localization: serde_json::Value = serde_json::from_reader(game_localization_file)?;

    let template = Regex::new(r"\{\$([A-z\d]+)\.([A-z\d]+)}")?;

    // We output to css so that they can also be used with Terrasavr (which as of August 3rd 2023 doesn't have correct sprites)
    let item_offset_filepath = gen_fol.join("items.css");
    let buff_offset_filepath = gen_fol.join("buffs.css");

    let client = {
        let mut map = HeaderMap::with_capacity(1);
        map.insert(reqwest::header::USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/132.0.0.0 Safari/537.3"));
        Client::builder().default_headers(map).build()?
    };

    println!("Generating item spritesheet");
    let item_spritesheet =
        generate_spritesheet(&items_fol, &res_fol, "slot", 2560, &item_offset_filepath)?;
    println!("Generating buff spritesheet");
    let buff_spritesheet =
        generate_spritesheet(&buffs_fol, &res_fol, "buff", 512, &buff_offset_filepath)?;
    println!("Generating icon spritesheet");
    let icon_spritesheet = {
        let sheet = image::open(res_fol.join("other").join("Extra_54.png"))?;
        sheet.resize(
            sheet.width() / 2,
            sheet.height() / 2,
            image::imageops::FilterType::Nearest,
        )
    };

    println!("Getting item meta");
    let item_meta = get_item_meta(
        &client,
        &template,
        &game_localization,
        &item_localization,
        &npc_localization,
        &item_offset_filepath,
    )?;
    println!("Getting buff meta");
    let buff_meta = get_buff_meta(
        &client,
        &template,
        &game_localization,
        &item_localization,
        &npc_localization,
        &buff_offset_filepath,
    )?;
    println!("Getting prefix meta");
    let prefix_meta = get_prefix_meta(
        &client,
        &template,
        &game_localization,
        &item_localization,
        &npc_localization,
    )?;

    println!("Writing to disk");

    if build_type == "debug" {
        serde_json::to_writer_pretty(item_writer, &item_meta)?;
        serde_json::to_writer_pretty(buff_writer, &buff_meta)?;
        serde_json::to_writer_pretty(prefix_writer, &prefix_meta)?;
    } else {
        serde_json::to_writer(item_writer, &item_meta)?;
        serde_json::to_writer(buff_writer, &buff_meta)?;
        serde_json::to_writer(prefix_writer, &prefix_meta)?;
    }
    item_spritesheet.save(gen_fol.join("items.png"))?;
    buff_spritesheet.save(gen_fol.join("buffs.png"))?;
    icon_spritesheet.save(gen_fol.join("icons.png"))?;

    let target_dir = PathBuf::from("./target").join(&build_type);
    let final_dir = target_dir.join("resources");

    if final_dir.exists() {
        fs::remove_dir_all(&final_dir)?;
    }

    let dir_options = DirCopyOptions::new().overwrite(true).copy_inside(true);
    copy_dir(&gen_fol, &final_dir, &dir_options)?;

    if build_type == "release" {
        let data_dir = PathBuf::from("./data/resources");
        if data_dir.exists() {
            fs::remove_dir_all(&data_dir)?;
        }
        copy_dir(&gen_fol, &data_dir, &dir_options)?;
    }

    Ok(())
}
