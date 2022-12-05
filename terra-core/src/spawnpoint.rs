#[derive(Clone, Debug)]
pub struct Spawnpoint {
    pub id: i32,
    pub x: i32,
    pub y: i32,
    pub name: String,
}

impl Default for Spawnpoint {
    fn default() -> Self {
        Self {
            id: 0,
            x: 0,
            y: 0,
            name: "".to_string(),
        }
    }
}
