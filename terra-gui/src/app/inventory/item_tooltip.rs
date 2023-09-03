use egui::{Rect, Response, RichText, Sense, Ui, Widget};
use terra_core::{utils, ItemMeta, ItemType, PrefixMeta, STRANGE_BREW_ID, STRANGE_BREW_MAX_HEAL};

use super::{item_name, item_slot::ItemSlotOptions};

#[derive(Debug, Clone, Copy)]
pub struct ItemTooltipOptions<'a> {
    pub id: i32,
    pub prefix_meta: Option<&'a PrefixMeta>,
    pub favourited: bool,
}

#[allow(dead_code)]
impl<'a> ItemTooltipOptions<'a> {
    pub fn new() -> Self {
        Self {
            id: 0,
            prefix_meta: None,
            favourited: false,
        }
    }

    pub fn from_slot_options(options: &'a ItemSlotOptions) -> Self {
        Self {
            id: options.id,
            prefix_meta: options.prefix_meta,
            favourited: options.favourited,
        }
    }

    pub fn id(mut self, id: i32) -> Self {
        self.id = id;
        self
    }

    pub fn prefix_meta(mut self, meta: Option<&'a PrefixMeta>) -> Self {
        self.prefix_meta = meta;
        self
    }

    pub fn favourited(mut self, favourited: bool) -> Self {
        self.favourited = favourited;
        self
    }
}

pub(super) struct ItemTooltip<'a> {
    options: ItemTooltipOptions<'a>,
    meta: &'a ItemMeta,
}

impl<'a> ItemTooltip<'a> {
    pub fn new(options: ItemTooltipOptions<'a>, meta: &'a ItemMeta) -> Self {
        Self { options, meta }
    }
}

impl<'a> Widget for ItemTooltip<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        if self.options.id == 0 {
            return ui.allocate_rect(Rect::NOTHING, Sense::hover());
        }

        let response = ui.allocate_rect(Rect::NOTHING, Sense::hover());

        let item = self.meta;
        let prefix = self.options.prefix_meta;

        if item.id == 0 {
            return response;
        }

        response.union(ui.heading(item_name(&item.name, prefix)));
        if item.forbidden.is_some_and(|f| f) {
            response.union(
                ui.small(
                    RichText::new("Forbidden")
                        .small()
                        .color(ui.style().visuals.error_fg_color),
                ),
            );
        }

        response.union(ui.small(format!("Id: {}", item.id)));
        if let Some(prefix) = prefix {
            if prefix.id != 0 {
                response.union(ui.small(format!("Prefix Id: {}", prefix.id)));
            }
        }

        if self.options.favourited {
            response.union(ui.label("Marked as favorite"));
            response.union(ui.label("Quick trash, stacking, and selling will be blocked"));
        }

        if let Some(damage) = item.damage {
            let mut string = damage.to_string();

            if let Some(item_type) = item.item_type.as_ref() {
                match item_type {
                    ItemType::Melee => string += " melee",
                    ItemType::Ranged => string += " ranged",
                    ItemType::Magic => string += " magic",
                    ItemType::Summon => string += " summon",
                    _ => {}
                }
            }

            string += " damage";

            if let Some(use_time) = item.use_time {
                string += &format!(" (~{:.0} DPS)", (damage as f32) * (60. / (use_time) as f32));
            }

            response.union(ui.label(string));
        }

        // NOTE: Inaccuracy here: crit chance is only displayed if melee, ranged, or magic, not always
        if let Some(crit_chance) = item.crit_chance {
            response.union(ui.label(format!("{}% critical strike chance", crit_chance)));
        }

        if let Some(use_time) = item.use_time {
            response.union(ui.label(format!(
                "Use time {} ({:.02}/s, {})",
                use_time,
                (60. / (use_time) as f32),
                utils::use_time_lookup(use_time)
            )));
        }

        if let Some(knockback) = item.knockback {
            response.union(ui.label(format!(
                "Knockback {} ({})",
                knockback,
                utils::knockback_lookup(knockback)
            )));
        }

        if let Some(fishing_power) = item.fishing_power {
            response.union(ui.label(format!("{}% fishing power", fishing_power)));
        }

        if let Some(fishing_bait) = item.fishing_bait {
            response.union(ui.label(format!("{}% fishing bait", fishing_bait)));
        }

        if let Some(tile) = item.consumes_tile {
            // TODO: Display the item name again
            response.union(ui.label(format!("Consumes {}", tile)));
        }

        if item
            .is_quest_item
            .is_some_and(|is_quest_item| is_quest_item)
        {
            response.union(ui.label("Quest Item"));
        }

        if let Some(ItemType::Vanity) = &item.item_type {
            response.union(ui.label("Vanity Item"));
        }

        if let Some(defense) = item.defense {
            if defense > 0 {
                response.union(ui.label(format!("{} defense", defense)));
            }
        }

        if let Some(pickaxe_power) = item.pickaxe_power {
            if pickaxe_power > 0 {
                response.union(ui.label(format!("{}% pickaxe power", pickaxe_power)));
            }
        }

        if let Some(axe_power) = item.axe_power {
            if axe_power > 0 {
                response.union(ui.label(format!("{}% axe power", axe_power * 5)));
            }
        }

        if let Some(hammer_power) = item.hammer_power {
            if hammer_power > 0 {
                response.union(ui.label(format!("{}% hammer power", hammer_power)));
            }
        }

        if let Some(range_boost) = item.range_boost {
            response.union(ui.label(format!(
                "{}{} range",
                if range_boost.is_positive() { "+" } else { "-" },
                range_boost
            )));
        }

        if let Some(heal_life) = item.heal_life {
            // Strange brew is strange
            if item.id == STRANGE_BREW_ID {
                response.union(ui.label(format!(
                    "Restores from {} to {} life",
                    heal_life, STRANGE_BREW_MAX_HEAL
                )));
            } else {
                response.union(ui.label(format!("Restores {} life", heal_life)));
            }
        }

        if let Some(heal_mana) = item.heal_mana {
            response.union(ui.label(format!("Restores {} mana", heal_mana)));
        }

        if let Some(mana_cost) = item.mana_cost {
            response.union(ui.label(format!("Uses {} mana", mana_cost)));
        }

        // NOTE: Not ingame
        if let Some(item_type) = &item.item_type {
            match item_type {
                ItemType::HeadArmor => {
                    response.union(ui.label("Equippable (head slot)"));
                }
                ItemType::BodyArmor => {
                    response.union(ui.label("Equippable (body slot)"));
                }
                ItemType::LegArmor => {
                    response.union(ui.label("Equippable (legs slot)"));
                }
                ItemType::Accessory => {
                    response.union(ui.label("Equippable (accessory)"));
                }
                ItemType::Wall => {
                    response.union(ui.label("Can be placed (wall)"));
                }
                ItemType::Tile => {
                    response.union(ui.label("Can be placed (tile)"));
                }
                ItemType::Ammo => {
                    response.union(ui.label("Ammo"));
                }
                _ => {}
            }
        }

        if item
            .is_consumable
            .is_some_and(|is_consumable| is_consumable)
        {
            response.union(ui.label("Consumable"));
        }

        if item.is_material.is_some_and(|is_material| is_material) {
            response.union(ui.label("Material"));
        }

        if let Some(tooltip) = &item.tooltip {
            for line in tooltip {
                response.union(ui.label(line));
            }
        }

        // Add already researched/research X more
        response.union(ui.label(format!(
            "Research {} to unlock duplication",
            item.sacrifices
        )));

        response.union(ui.label(format!("{} Max Stack", item.max_stack)));

        response.union(ui.label(format!("Worth {}", utils::coins_lookup(item.value))));

        // TODO: Maybe prefix values?

        response
    }
}
