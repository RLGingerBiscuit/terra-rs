use std::{
    fs::{File, OpenOptions},
    io::{BufWriter, Read},
    path::PathBuf,
    str::FromStr,
};

use anyhow::Result;
use regex::{Captures, Regex};

use fs_extra::dir::{copy as copy_dir, create as create_dir, CopyOptions};

use rlua::{Lua, Table};
use terra_core::{buff::Buff, buff::BuffType, item::Item, prefix::Prefix};

const ITEM_INFO_URL: &str = "https://terraria.wiki.gg/api.php?action=query&prop=revisions&format=json&rvlimit=1&rvslots=*&rvprop=content&titles=Module:Iteminfo/data";
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
        return "".to_string();
    }

    let expanded = template
        .replace_all(&s, |cap: &Captures| {
            if cap[1].starts_with("NPC") {
                if cap[2].to_owned() == "None" {
                    "".to_string()
                } else {
                    npcs[&cap[1]][&cap[2]].as_str().unwrap().to_string()
                }
            } else if cap[1].starts_with("Buff") {
                game[&cap[1]][&cap[2]]
                    .as_str()
                    .unwrap_or_else(|| {
                        println!("Warning: {} not found!", &cap[0]);
                        &cap[0][2..{ cap.len() - 1 }]
                    })
                    .to_string()
            } else if cap[1].contains("Item") || cap[1].starts_with("PaintingArtist") {
                items[&cap[1]][&cap[2]].as_str().unwrap().to_string()
            } else {
                println!("Warning: {} not found!", &cap[0]);
                "".to_string()
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

fn get_item_info(
    template: &Regex,
    game: &serde_json::Value,
    items: &serde_json::Value,
    npcs: &serde_json::Value,
) -> Result<Vec<Item>> {
    // https://terraria.wiki.gg/wiki/Module:Iteminfo/data
    // API: https://terraria.wiki.gg/api.php?action=query&prop=revisions&titles=Module:Iteminfo/data&rvlimit=1&rvslots=*&rvprop=ids|content&format=json
    // r[query][pages][18021][revisions][0][slots][main][*]
    // Split on "local cache = require 'mw.ext.LuaCache'"

    let lua_resp = reqwest::blocking::get(ITEM_INFO_URL)?;
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

    let mut item_info: Vec<Item> = Vec::new();

    let lua = Lua::new();

    lua.context(|ctx| -> Result<()> {
        let info: Table = ctx
            .load(lua_str.as_str())
            .set_name("Iteminfo_data")
            .unwrap()
            .eval()?;

        for pair in info.pairs::<rlua::String, rlua::Value>() {
            let (_, val) = pair?;

            if let rlua::Value::Table(lua_item) = val {
                let id = lua_item.get("netID").unwrap_or(0);
                let internal_name = lua_item.get("internalName").unwrap_or("".to_string());
                let name = lua_item.get("name").unwrap_or("".to_string());
                let max_stack = lua_item.get("maxStack").unwrap_or(1);
                let sacrifices = lua_item.get("sacrifices").unwrap_or(1);

                let tooltip = match items["ItemTooltip"][&internal_name].as_str() {
                    Some(tt) => tt
                        .lines()
                        .map(|s| expand_templates(s.to_string(), &template, &game, &items, &npcs))
                        .collect::<Vec<_>>()
                        .join("\n"),
                    None => "".to_string(),
                };

                let item = Item {
                    id,
                    internal_name,
                    name,
                    max_stack,
                    sacrifices,
                    tooltip,
                    ..Default::default()
                };

                item_info.push(item);
            }
        }

        Ok(())
    })?;

    item_info.sort_by_key(|i| i.id);

    Ok(item_info)
}

fn get_buff_info(
    template: &Regex,
    game: &serde_json::Value,
    items: &serde_json::Value,
    npcs: &serde_json::Value,
) -> Result<Vec<Buff>> {
    let resp = reqwest::blocking::get(BUFF_IDS_URL)?;
    let text = resp.text()?;

    let doc = scraper::Html::parse_document(&text);

    let tbody_selector = scraper::Selector::parse("table.terraria.sortable").unwrap();
    let tr_selector = scraper::Selector::parse("tbody>tr").unwrap();
    let td_selector = scraper::Selector::parse("td").unwrap();

    let internal_name_selector = scraper::Selector::parse("code").unwrap();
    let name_selector = scraper::Selector::parse("span.i>span>span>a").unwrap();
    let image_selector = scraper::Selector::parse("span.i>a>img").unwrap();

    let mut buff_info: Vec<Buff> = doc
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
                .to_string();

            let name = match tds.next().unwrap().select(&name_selector).next() {
                Some(a) => a.inner_html().trim().to_string(),
                None => image_text,
            };

            let internal_name = tds
                .next()
                .unwrap()
                .select(&internal_name_selector)
                .next()
                .unwrap()
                .inner_html()
                .trim()
                .to_string();

            let buff_type = match tds.next().unwrap().inner_html().trim() {
                "Buff" => BuffType::Buff,
                "Debuff" => BuffType::Debuff,
                _ => panic!("TF THIS?"),
            };

            let tooltip = match &game["BuffDescription"][&internal_name].as_str() {
                Some(tt) => tt
                    .lines()
                    .map(|s| expand_templates(s.to_string(), template, game, items, npcs))
                    .collect::<Vec<_>>()
                    .join("\n"),
                None => "".to_string(),
            };

            Buff {
                id,
                name,
                internal_name,
                buff_type,
                tooltip,
                ..Default::default()
            }
        })
        .collect();

    buff_info.sort_by_key(|b| b.id);

    Ok(buff_info)
}

fn get_prefix_info(
    _template: &Regex,
    _game: &serde_json::Value,
    items: &serde_json::Value,
    _npcs: &serde_json::Value,
) -> Result<Vec<Prefix>> {
    let resp = reqwest::blocking::get(PREFIX_IDS_URL)?;
    let text = resp.text()?;

    let doc = scraper::Html::parse_document(&text);

    // Edge cases:
    // 20 = Deadly2
    // 75 = Hasty2
    // 76 = Quick2
    // 84 = Legendary2
    // 90 = Piercing (mobile only)

    let tbody_selector = scraper::Selector::parse("table.terraria.sortable").unwrap();
    let tr_selector = scraper::Selector::parse("tbody>tr").unwrap();
    let td_selector = scraper::Selector::parse("td").unwrap();

    let mut prefix_info: Vec<Prefix> = doc
        .select(&tbody_selector)
        .next()
        .unwrap()
        .select(&tr_selector)
        .skip(1)
        .map(|tr| {
            let mut tds = tr.select(&td_selector);

            let id = i32::from_str(tds.next().unwrap().inner_html().trim()).unwrap();

            let internal_name = match id {
                20 => "Deadly2".to_string(),
                75 => "Hasty2".to_string(),
                76 => "Quick2".to_string(),
                84 => "Legendary2".to_string(),
                90 => "Piercing".to_string(),
                _ => tds.next().unwrap().inner_html().trim().to_string(),
            };

            // Quick hack because Piercing is mobile only
            let name = if internal_name == "Piercing" {
                internal_name.clone()
            } else {
                items["Prefix"][&internal_name]
                    .as_str()
                    .unwrap()
                    .to_string()
            };

            Prefix {
                id,
                internal_name,
                name,
            }
        })
        .collect();

    prefix_info.sort_by_key(|p| p.id);

    Ok(prefix_info)
}

fn main() -> Result<()> {
    let res_fol = PathBuf::from_str("./terra-res/resources")?;
    let gen_fol = PathBuf::from_str("./terra-res/generated")?;

    create_dir(&gen_fol, true)?;

    let mut json_item_file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(false)
        .open(gen_fol.join("items.json"))?;
    let json_item_writer = BufWriter::new(&mut json_item_file);

    let mut json_buff_file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(false)
        .open(gen_fol.join("buffs.json"))?;
    let json_buff_writer = BufWriter::new(&mut json_buff_file);

    let mut json_prefix_file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(false)
        .open(gen_fol.join("prefixes.json"))?;
    let json_prefix_writer = BufWriter::new(&mut json_prefix_file);

    let item_localization_file = File::open(res_fol.join("Items.json"))?;
    let npc_localization_file = File::open(res_fol.join("NPCs.json"))?;
    let game_localization_file = File::open(res_fol.join("Game.json"))?;

    let item_localization: serde_json::Value = serde_json::from_reader(item_localization_file)?;
    let npc_localization: serde_json::Value = serde_json::from_reader(npc_localization_file)?;
    let game_localization: serde_json::Value = serde_json::from_reader(game_localization_file)?;

    let template = Regex::new(r"\{\$([A-z\d]+)\.([A-z\d]+)\}")?;

    let item_info = get_item_info(
        &template,
        &game_localization,
        &item_localization,
        &npc_localization,
    )?;

    let buff_info = get_buff_info(
        &template,
        &game_localization,
        &item_localization,
        &npc_localization,
    )?;
    let prefix_info = get_prefix_info(
        &template,
        &game_localization,
        &item_localization,
        &npc_localization,
    )?;

    serde_json::to_writer_pretty(json_item_writer, &item_info)?;
    serde_json::to_writer_pretty(json_buff_writer, &buff_info)?;
    serde_json::to_writer_pretty(json_prefix_writer, &prefix_info)?;

    // Pretty scuffed but works for now
    let mut build_type = String::new();
    File::open("./terra-res/build_type.txt")?.read_to_string(&mut build_type)?;

    let target_dir = PathBuf::from("./target").join(&build_type);
    let final_dir = target_dir.join("resources");

    let mut options = CopyOptions::new();
    options.overwrite = true;
    options.copy_inside = true;

    copy_dir(&gen_fol, &final_dir, &options)?;

    Ok(())
}
