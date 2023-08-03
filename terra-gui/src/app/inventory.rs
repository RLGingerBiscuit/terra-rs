use egui::{pos2, vec2, Align2, Image, Rect, Response, RichText, TextStyle, TextureHandle, Ui};

use terra_core::{
    utils, BuffMeta, ItemMeta, ItemType, PrefixMeta, BUFF_SPRITE_SIZE as CORE_BUFF_SPRITE_SIZE,
    STRANGE_BREW_ID, STRANGE_BREW_MAX_HEAL,
};

use crate::ui::{ClickableFrame, UiExt};

use super::{App, Message};

pub const ITEM_SLOT_SIZE: f32 = 40.;
pub const ITEM_SPRITE_SCALE: f32 = 2.;

pub const BUFF_SLOT_SIZE: f32 = 32.;
pub const BUFF_SPRITE_SIZE: f32 = CORE_BUFF_SPRITE_SIZE as f32;
pub const BUFF_SPRITE_SCALE: f32 = 2.;

#[macro_export]
macro_rules! meta_or_default {
    ($meta:expr,$id:expr) => {
        $meta
            .get($id as usize)
            // .iter().filter(|m| m.id == id).next()
            .unwrap_or($meta.get(0).expect("We really should have a zeroth meta"))
    };
}

#[macro_export]
macro_rules! selected_item {
    ($selected_item:expr,$selected_loadout:expr,$player:expr) => {
        match $selected_item.0 {
            ItemTab::Inventory => &mut $player.inventory[$selected_item.1],
            ItemTab::Coins => &mut $player.coins[$selected_item.1],
            ItemTab::Ammo => &mut $player.ammo[$selected_item.1],
            ItemTab::Bank => &mut $player.piggy_bank[$selected_item.1],
            ItemTab::Safe => &mut $player.safe[$selected_item.1],
            ItemTab::Forge => &mut $player.defenders_forge[$selected_item.1],
            ItemTab::Void => &mut $player.void_vault[$selected_item.1],
            ItemTab::Equipment => &mut $player.equipment[$selected_item.1],
            ItemTab::EquipmentDyes => &mut $player.equipment_dyes[$selected_item.1],
            // TODO: The only case where a change will happen immediately without clicking on another item is changing loadouts, do I want to keep this?
            ItemTab::VanityArmor => {
                &mut $player.loadouts[$selected_loadout.0].vanity_armor[$selected_item.1]
            }
            ItemTab::Armor => {
                &mut $player.loadouts[$selected_loadout.0].armor[$selected_item.1]
            }
            ItemTab::ArmorDyes => {
                &mut $player.loadouts[$selected_loadout.0].armor_dyes[$selected_item.1]
            }
            ItemTab::VanityAccessories => {
                &mut $player.loadouts[$selected_loadout.0].vanity_accessories
                    [$selected_item.1]
            }
            ItemTab::Accessories => {
                &mut $player.loadouts[$selected_loadout.0].accessories[$selected_item.1]
            }
            ItemTab::AccessoryDyes => {
                &mut $player.loadouts[$selected_loadout.0].accessory_dyes[$selected_item.1]
            }
        }
    };
}

#[macro_export]
macro_rules! selected_buff {
    ($selected_buff:expr,$player:expr) => {
        &mut $player.buffs[$selected_buff.0]
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ItemTab {
    Inventory,
    Coins,
    Ammo,
    Bank,
    Safe,
    Forge,
    Void,
    Equipment,
    EquipmentDyes,
    Armor,
    VanityArmor,
    ArmorDyes,
    Accessories,
    VanityAccessories,
    AccessoryDyes,
}

#[derive(Debug)]
pub struct SelectedItem(pub ItemTab, pub usize);

#[derive(Debug)]
pub struct SelectedBuff(pub usize);

#[derive(Debug)]
pub struct SelectedLoadout(pub usize);

impl App {
    // TODO: split sprite into render_icon (or something)
    // TODO: render slot icons
    // TODO: render coloured slots
    pub fn render_slot(
        &self,
        ui: &mut Ui,
        slot_size: f32,
        scale: f32,
        with_margin: bool,
        rect: Rect,
        selected: bool,
        spritesheet: Option<&TextureHandle>,
        stack_size: Option<i32>,
    ) -> Response {
        let mut padding_x = slot_size;
        let mut padding_y = slot_size;

        let [width, height]: [f32; 2] = rect.size().into();
        let [x, y]: [f32; 2] = rect.left_top().into();

        let group = if selected {
            let mut frame = ClickableFrame::group(ui.style());
            frame.fill = ui.visuals().code_bg_color;
            frame
        } else {
            ClickableFrame::group(ui.style())
        };

        let group = if with_margin {
            group
        } else {
            group.inner_margin(0.)
        };

        let response = group
            .show(ui, |ui| {
                ui.spacing_mut().item_spacing = [0., 0.].into();

                if let Some(spritesheet) = spritesheet {
                    let [spritesheet_width, spritesheet_height] =
                        spritesheet.size().map(|s| s as f32);

                    let mut final_width = width * scale;
                    let mut final_height = height * scale;

                    let scale = if final_width > slot_size || final_height > slot_size {
                        if final_width <= final_height {
                            slot_size / final_height
                        } else {
                            slot_size / final_width
                        }
                    } else {
                        1.
                    };

                    final_width *= scale;
                    final_height *= scale;

                    padding_x = if final_width < slot_size {
                        (slot_size - final_width) / 2.
                    } else {
                        0.
                    };
                    padding_y = if final_height < slot_size {
                        (slot_size - final_height) / 2.
                    } else {
                        0.
                    };

                    let min = pos2(x / spritesheet_width, y / spritesheet_height);
                    let sprite_size = vec2(width / spritesheet_width, height / spritesheet_height);
                    let uv = Rect::from_min_size(min, sprite_size);
                    let size = vec2(final_width, final_height);

                    ui.add_space(padding_x);
                    ui.vertical(|ui| {
                        ui.add_space(padding_y);
                        ui.add(Image::new(spritesheet, size).uv(uv));
                        ui.add_space(padding_y);
                    });
                    ui.add_space(padding_x);
                } else {
                    ui.add_space(padding_x);
                    ui.vertical(|ui| {
                        ui.add_space(padding_y);
                    });
                }
            })
            .response;

        if let Some(stack) = stack_size {
            let max = response.rect.max;
            let icon_spacing = ui.style().spacing.icon_spacing;

            ui.painter().text(
                pos2(max.x - icon_spacing, max.y - icon_spacing),
                Align2::RIGHT_BOTTOM,
                stack.to_string(),
                TextStyle::Body.resolve(ui.style()),
                ui.style().visuals.text_color(),
            );
        }

        response
    }

    pub fn render_item_slot(
        &self,
        ui: &mut Ui,
        item_id: i32,
        prefix_meta: Option<&PrefixMeta>,
        favourited: bool,
        selected: bool,
        stack_size: Option<i32>,
        tooltip_on_hover: bool,
    ) -> Response {
        let spritesheet = self.item_spritesheet.read();

        if spritesheet.is_none() && !self.busy {
            self.do_update(Message::LoadItemSpritesheet);
        }

        let item_meta = &*self.item_meta.read();
        let meta = meta_or_default!(item_meta, item_id);

        let min = pos2(meta.x as f32, meta.y as f32);
        let size = vec2(meta.width as f32, meta.height as f32);
        let rect = Rect::from_min_size(min, size);

        let response = self.render_slot(
            ui,
            ITEM_SLOT_SIZE,
            ITEM_SPRITE_SCALE,
            true,
            rect,
            selected,
            spritesheet.as_ref(),
            stack_size,
        );

        if meta.id != 0 && tooltip_on_hover {
            response.on_hover_ui(|ui| {
                self.render_item_tooltip(ui, meta, prefix_meta, favourited);
            })
        } else {
            response
        }
    }

    // TODO: Take in an Iterator?
    pub fn render_item_slots_special(
        &self,
        ui: &mut Ui,
        tooltip_on_hover: bool,
        items: &[(i32, Option<&PrefixMeta>, bool, Option<i32>, ItemTab, usize)],
    ) {
        for (item_id, prefix_meta, favourited, stack_size, tab, index) in items {
            let selected = &self.selected_item.0 == tab && &self.selected_item.1 == index;

            if self
                .render_item_slot(
                    ui,
                    *item_id,
                    *prefix_meta,
                    *favourited,
                    selected,
                    *stack_size,
                    tooltip_on_hover,
                )
                .clicked()
            {
                self.do_update(Message::SelectItem(SelectedItem(*tab, *index)));
            }
        }
    }

    pub fn item_name(&self, item_name: &str, prefix_meta: Option<&PrefixMeta>) -> String {
        if let Some(prefix_meta) = prefix_meta {
            if prefix_meta.id != 0 {
                return format!("{} {}", &prefix_meta.name, item_name);
            }
        }

        item_name.to_owned()
    }

    pub fn item_name_from_id(&self, item_id: i32, prefix_meta: Option<&PrefixMeta>) -> String {
        let item_meta = &*self.item_meta.read();
        let item_meta = meta_or_default!(item_meta, item_id);

        if let Some(prefix_meta) = prefix_meta {
            if prefix_meta.id != 0 {
                return format!("{} {}", prefix_meta.name, item_meta.name);
            }
        }

        item_meta.name.to_owned()
    }

    pub fn render_selected_item(&mut self, ui: &mut Ui) {
        let mut player = self.player.write();

        let item = selected_item!(self.selected_item, self.selected_loadout, player);

        let item_meta = &*self.item_meta.read();
        let prefix_meta = &*self.prefix_meta.read();

        let current_item_meta = meta_or_default!(item_meta, item.id);
        let current_prefix_meta = meta_or_default!(prefix_meta, item.prefix.id);

        let largest_item_id = item_meta
            .last()
            .expect("We really should have at least one item")
            .id;
        let largest_prefix_id = prefix_meta
            .last()
            .expect("We really should have at least one prefix")
            .id;

        ui.label(self.item_name(&current_item_meta.name, Some(current_prefix_meta)));

        egui::Grid::new("selected_item")
            .num_columns(3)
            .show(ui, |ui| {
                ui.label("Id: ");
                ui.drag_value_with_buttons(&mut item.id, 1., 0..=largest_item_id);
                if ui.button("\u{1f50e}").clicked() {
                    self.do_update(Message::OpenItemBrowser);
                }
                ui.end_row();

                ui.label("Stack: ");
                ui.drag_value_with_buttons(&mut item.stack, 1., 0..=current_item_meta.max_stack);
                if ui.button("Max").clicked() {
                    item.stack = current_item_meta.max_stack;
                }
                ui.end_row();

                ui.label("Prefix: ");
                ui.drag_value_with_buttons(&mut item.prefix.id, 1., 0..=largest_prefix_id);
                if ui.button("\u{1f50e}").clicked() {
                    self.do_update(Message::OpenPrefixBrowser);
                }
                ui.end_row();
            });
    }

    pub fn render_item_tooltip(
        &self,
        ui: &mut Ui,
        item_meta: &ItemMeta,
        prefix_meta: Option<&PrefixMeta>,
        favourited: bool,
    ) {
        if item_meta.id == 0 {
            return;
        }

        ui.heading(self.item_name(&item_meta.name, prefix_meta));
        if item_meta.forbidden.is_some_and(|f| f) {
            ui.small(
                RichText::new("Forbidden")
                    .small()
                    .color(ui.style().visuals.error_fg_color),
            );
        }

        ui.small(format!("Id: {}", item_meta.id));
        if let Some(prefix_meta) = prefix_meta {
            if prefix_meta.id != 0 {
                ui.small(format!("Prefix Id: {}", prefix_meta.id));
            }
        }

        if favourited {
            ui.label("Marked as favorite");
            ui.label("Quick trash, stacking, and selling will be blocked");
        }

        if let Some(damage) = item_meta.damage {
            let mut string = damage.to_string();

            if let Some(item_type) = &item_meta.item_type {
                match item_type {
                    ItemType::Melee => string += " melee",
                    ItemType::Ranged => string += " ranged",
                    ItemType::Magic => string += " magic",
                    ItemType::Summon => string += " summon",
                    _ => {}
                }
            }

            string += " damage";

            if let Some(use_time) = item_meta.use_time {
                string += &format!(" (~{:.0} DPS)", (damage as f32) * (60. / (use_time) as f32));
            }

            ui.label(string);
        }

        // NOTE: Inaccuracy here: crit chance is only displayed if melee, ranged, or magic, not always
        if let Some(crit_chance) = item_meta.crit_chance {
            ui.label(format!("{}% critical strike chance", crit_chance));
        }

        if let Some(use_time) = item_meta.use_time {
            ui.label(format!(
                "Use time {} ({:.02}/s, {})",
                use_time,
                (60. / (use_time) as f32),
                utils::use_time_lookup(use_time)
            ));
        }

        if let Some(knockback) = item_meta.knockback {
            ui.label(format!(
                "Knockback {} ({})",
                knockback,
                utils::knockback_lookup(knockback)
            ));
        }

        if let Some(fishing_power) = item_meta.fishing_power {
            ui.label(format!("{}% fishing power", fishing_power));
        }

        if let Some(fishing_bait) = item_meta.fishing_bait {
            ui.label(format!("{}% fishing bait", fishing_bait));
        }

        if let Some(consumes_tile) = item_meta.consumes_tile {
            ui.label(format!(
                "Consumes {}",
                self.item_name_from_id(consumes_tile, prefix_meta)
            ));
        }

        if item_meta
            .is_quest_item
            .is_some_and(|is_quest_item| is_quest_item)
        {
            ui.label("Quest Item");
        }

        if let Some(ItemType::Vanity) = &item_meta.item_type {
            ui.label("Vanity Item");
        }

        if let Some(defense) = item_meta.defense {
            if defense > 0 {
                ui.label(format!("{} defense", defense));
            }
        }

        if let Some(pickaxe_power) = item_meta.pickaxe_power {
            if pickaxe_power > 0 {
                ui.label(format!("{}% pickaxe power", pickaxe_power));
            }
        }

        if let Some(axe_power) = item_meta.axe_power {
            if axe_power > 0 {
                ui.label(format!("{}% axe power", axe_power * 5));
            }
        }

        if let Some(hammer_power) = item_meta.hammer_power {
            if hammer_power > 0 {
                ui.label(format!("{}% hammer power", hammer_power));
            }
        }

        if let Some(range_boost) = item_meta.range_boost {
            ui.label(format!(
                "{}{} range",
                if range_boost.is_positive() { "+" } else { "" },
                range_boost
            ));
        }

        if let Some(heal_life) = item_meta.heal_life {
            // Strange brew is strange
            if item_meta.id == STRANGE_BREW_ID {
                ui.label(format!(
                    "Restores from {} to {} life",
                    heal_life, STRANGE_BREW_MAX_HEAL
                ));
            } else {
                ui.label(format!("Restores {} life", heal_life));
            }
        }

        if let Some(heal_mana) = item_meta.heal_mana {
            ui.label(format!("Restores {} mana", heal_mana));
        }

        if let Some(mana_cost) = item_meta.mana_cost {
            ui.label(format!("Uses {} mana", mana_cost));
        }

        // NOTE: Not ingame
        if let Some(item_type) = &item_meta.item_type {
            match item_type {
                ItemType::HeadArmor => {
                    ui.label("Equippable (head slot)");
                }
                ItemType::BodyArmor => {
                    ui.label("Equippable (body slot)");
                }
                ItemType::LegArmor => {
                    ui.label("Equippable (legs slot)");
                }
                ItemType::Accessory => {
                    ui.label("Equippable (accessory)");
                }
                _ => {}
            }
        }

        if let Some(item_type) = &item_meta.item_type {
            match item_type {
                ItemType::Wall => {
                    ui.label("Can be placed (wall)");
                }
                ItemType::Tile => {
                    ui.label("Can be placed (tile)");
                }
                ItemType::Ammo => {
                    ui.label("Ammo");
                }
                _ => {}
            }
        }

        if item_meta
            .is_consumable
            .is_some_and(|is_consumable| is_consumable)
        {
            ui.label("Consumable");
        }

        if item_meta.is_material.is_some_and(|is_material| is_material) {
            ui.label("Material");
        }

        if let Some(tooltip) = &item_meta.tooltip {
            for line in tooltip {
                ui.label(line);
            }
        }

        ui.label(format!("{} Max Stack", item_meta.max_stack));

        ui.label(format!("Worth {}", utils::coins_lookup(item_meta.value)));

        // TODO: Maybe prefix values?
    }

    pub fn render_buff_slot(
        &self,
        ui: &mut Ui,
        buff_id: i32,
        buff_time: Option<i32>,
        selected: bool,
        tooltip_on_hover: bool,
    ) -> Response {
        let spritesheet = self.buff_spritesheet.read();

        if spritesheet.is_none() && !self.busy {
            self.do_update(Message::LoadBuffSpritesheet);
        }

        let buff_meta = &*self.buff_meta.read();
        let meta = meta_or_default!(buff_meta, buff_id);

        let min = pos2(meta.x as f32, meta.y as f32);
        let size = vec2(BUFF_SPRITE_SIZE, BUFF_SPRITE_SIZE);
        let rect = Rect::from_min_size(min, size);

        let response = self.render_slot(
            ui,
            BUFF_SLOT_SIZE,
            BUFF_SPRITE_SCALE,
            false,
            rect,
            selected,
            spritesheet.as_ref(),
            None,
        );

        if meta.id != 0 && tooltip_on_hover {
            response.on_hover_ui(|ui| {
                self.render_buff_tooltip(ui, meta, buff_time);
            })
        } else {
            response
        }
    }

    // TODO: Take in an Iterator?
    pub fn render_buff_slots_special(
        &self,
        ui: &mut Ui,
        tooltip_on_hover: bool,
        buffs: &[(i32, Option<i32>, usize)],
    ) {
        for (buff_id, buff_time, index) in buffs {
            let selected = &self.selected_buff.0 == index;
            if self
                .render_buff_slot(ui, *buff_id, *buff_time, selected, tooltip_on_hover)
                .clicked()
            {
                self.do_update(Message::SelectBuff(SelectedBuff(*index)));
            }
        }
    }

    pub fn buff_name(&self, buff_name: &str, buff_time: Option<i32>) -> String {
        const FRAMES_PER_SECOND: i32 = 60;
        const FRAMES_PER_MINUTE: i32 = FRAMES_PER_SECOND * 60;
        const FRAMES_PER_HOUR: i32 = FRAMES_PER_MINUTE * 60;
        const FRAMES_PER_THOUSAND_HOURS: i32 = FRAMES_PER_HOUR * 1000;

        if let Some(time) = buff_time {
            if time == 0 {
                buff_name.to_owned()
            } else {
                let time = if time < FRAMES_PER_SECOND {
                    format!("({}f)", time)
                } else if time < FRAMES_PER_MINUTE {
                    format!("({}s)", time / FRAMES_PER_SECOND)
                } else if time < FRAMES_PER_HOUR {
                    format!("({}m)", time / FRAMES_PER_MINUTE)
                } else if time < FRAMES_PER_THOUSAND_HOURS {
                    format!("({}h)", time / FRAMES_PER_HOUR)
                } else {
                    "(∞)".to_owned()
                };

                format!("{} {}", buff_name, time)
            }
        } else {
            buff_name.to_owned()
        }
    }

    pub fn render_selected_buff(&mut self, ui: &mut Ui) {
        let mut player = self.player.write();

        let buff = selected_buff!(self.selected_buff, player);

        let buff_meta = &*self.buff_meta.read();
        let meta = meta_or_default!(buff_meta, buff.id);

        let largest_buff_id = buff_meta
            .last()
            .expect("we really should have at least one buff")
            .id;

        ui.label(self.buff_name(&meta.name, Some(buff.time)));

        egui::Grid::new("selected_buff")
            .num_columns(3)
            .show(ui, |ui| {
                ui.label("Id: ");
                ui.drag_value_with_buttons(&mut buff.id, 1., 0..=largest_buff_id);
                if ui.button("\u{1f50e}").clicked() {
                    self.do_update(Message::OpenBuffBrowser);
                }
                ui.end_row();

                ui.label("Duration: ");
                ui.drag_value_with_buttons(&mut buff.time, 1., 0..=i32::MAX);
                if ui.button("Max").clicked() {
                    buff.time = i32::MAX;
                }
                ui.end_row();
            });
    }

    pub fn render_buff_tooltip(&self, ui: &mut Ui, buff_meta: &BuffMeta, buff_time: Option<i32>) {
        if buff_meta.id == 0 {
            return;
        }

        ui.heading(self.buff_name(&buff_meta.name, buff_time));

        ui.small(format!("ID: {}", buff_meta.id));

        if let Some(tooltip) = &buff_meta.tooltip {
            for line in tooltip {
                ui.label(line);
            }
        }
    }

    pub fn render_prefix_tooltip(&self, ui: &mut Ui, prefix_meta: &PrefixMeta) {
        if prefix_meta.id == 0 {
            return;
        }

        ui.heading(&prefix_meta.name);
        ui.small(format!("Id: {}", prefix_meta.id));
    }
}
