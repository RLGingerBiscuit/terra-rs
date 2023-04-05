use serde_repr::{Deserialize_repr, Serialize_repr};

mod item;
mod item_meta;

pub use item::{Item, ItemError};
pub use item_meta::ItemMeta;

#[derive(thiserror::Error, Debug)]
pub enum ItemError {
    #[error("Either id or internal_name need to be true to correctly load/save an item.")]
    NoIdOrInternalName,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Item {
    pub id: i32,
    pub name: String,
    pub internal_name: String,
    pub max_stack: i32,
    #[serde(skip)]
    pub stack: i32,
    #[serde(skip)]
    pub prefix: Prefix,
    pub sacrifices: i32,
    pub tooltip: String,
    pub favourited: bool,
}

impl Default for Item {
    fn default() -> Self {
        Self {
            id: 0,
            name: "".to_owned(),
            internal_name: "".to_owned(),
            max_stack: 0,
            stack: 0,
            prefix: Prefix::default(),
            sacrifices: 0,
            tooltip: "".to_owned(),
            favourited: false,
        }
    }
}

impl Item {
    pub fn load_items() -> Result<Vec<Self>> {
        let items_file = File::open(
            std::env::current_exe()?
                .parent()
                .unwrap()
                .join("resources")
                .join("items.json"),
        )?;

        let items_reader = BufReader::new(items_file);

        let items: Vec<Self> = serde_json::from_reader(items_reader)?;

        Ok(items)
    }

    fn legacy_lookup(version: i32, legacy_name: &str) -> String {
        let legacy_name = legacy_name.into();

        if version <= 4 {
            if legacy_name == "Cobalt Helmet" {
                "Jungle Hat".to_owned()
            } else if legacy_name == "Cobalt Breastplate" {
                "Jungle Shirt".to_owned()
            } else if legacy_name == "Cobalt Greaves" {
                "Jungle Pants".to_owned()
            } else {
                legacy_name
            }
        } else if version <= 20 {
            if legacy_name == "Gills potion" {
                "Gills Potion".to_owned()
            } else if legacy_name == "Thorn Chakrum" {
                "Thorm Chakram".to_owned()
            } else if legacy_name == "Ball 'O Hurt" {
                "Ball O' Hurt".to_owned()
            } else {
                legacy_name
            }
        } else if version <= 41 && legacy_name == "Iron Chain" {
            "Chain".to_owned()
        } else if version <= 44 && legacy_name == "Orb of Light" {
            "Shadow Orb".to_owned()
        } else if version <= 46 {
            if legacy_name == "Black Dye" {
                "Black Thread".to_owned()
            } else if legacy_name == "Green Dye" {
                "Green Thread".to_owned()
            } else {
                legacy_name
            }
        } else {
            legacy_name
        }
    }

    fn reverse_legacy_lookup(version: i32, name: &str) -> String {
        if version <= 4 {
            if name == "Jungle Hat" {
                "Cobalt Helmet".to_owned()
            } else if name == "Jungle Shirt" {
                "Cobalt Breastplate".to_owned()
            } else if name == "Jungle Pants" {
                "Cobalt Greaves".to_owned()
            } else {
                name.to_owned()
            }
        } else if version <= 20 {
            if name == "Gills Potion" {
                "Gills potion".to_owned()
            } else if name == "Thorn Chakram" {
                "Thork Chakrum".to_owned()
            } else if name == "Ball O' Hurt" {
                "Ball 'O Hurt".to_owned()
            } else {
                name.to_owned()
            }
        } else if version <= 41 && name == "Chain" {
            "Iron Chain".to_owned()
        } else if version <= 44 && name == "Shadow Orb" {
            "Orb of Light".to_owned()
        } else if version <= 46 {
            if name == "Black Thread" {
                "Black Dye".to_owned()
            } else if name == "Green Thread" {
                "Green Dye".to_owned()
            } else {
                name.to_owned()
            }
        } else {
            name.to_owned()
        }
    }

    fn copy(&mut self, item: &Self) {
        self.id = item.id.clone();
        self.internal_name = item.internal_name.clone();
        self.name = item.name.clone();
        self.max_stack = item.max_stack.clone();
        self.sacrifices = item.sacrifices.clone();
        self.tooltip = item.tooltip.clone();
    }

    pub fn load(
        &mut self,
        reader: &mut dyn Read,
        items: &Vec<Self>,
        prefixes: &Vec<Prefix>,
        id: bool,
        internal_name: bool,
        stack: bool,
        prefix: bool,
        favourited: bool,
    ) -> Result<()> {
        if !id && !internal_name {
            return Err(ItemError::NoIdOrInternalName.into());
        }

        let mut internal_name_string = "".to_owned();

        if id {
            self.id = reader.read_i32::<LE>()?
        }
        if internal_name {
            internal_name_string = reader.read_lpstring()?;
        }
        if stack {
            self.stack = reader.read_i32::<LE>()?
        }
    }
}
