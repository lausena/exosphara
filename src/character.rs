use rand::RngExt;
use strum::EnumCount;

use crate::inventory_item::InventoryItem;
use crate::planet::Planet;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Character {
    pub name: String,
    pub age: u32,
    pub health: u32,
    pub inventory: Vec<InventoryItem>,
    pub origin: Planet,
}

impl Default for Character {
    fn default() -> Self {
        Self {
            name: "Unknown".to_string(),
            age: 0,
            health: 100,
            inventory: Vec::new(),
            origin: Planet::Earth,
        }
    }
}

#[derive(Debug)]
pub struct CharacterBuilder {
    name: Option<String>,
    age: u32,
    health: u32,
    inventory: Vec<InventoryItem>,
    origin: Planet,
}

impl CharacterBuilder {
    pub fn new() -> Self {
        Self {
            name: None,
            age: 0,
            health: 100,
            inventory: Vec::new(),
            origin: Planet::Earth,
        }
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn age(mut self, age: u32) -> Self {
        self.age = age;
        self
    }

    pub fn health(mut self, health: u32) -> Self {
        self.health = health;
        self
    }

    pub fn inventory(mut self, inventory: Vec<InventoryItem>) -> Self {
        self.inventory = inventory;
        self
    }

    pub fn add_item(mut self, item: InventoryItem) -> Self {
        self.inventory.push(item);
        self
    }

    pub fn origin(mut self, origin: Planet) -> Self {
        self.origin = origin;
        self
    }

    pub fn try_build(self) -> Result<Character, &'static str> {
        match self.name {
            Some(name) if !name.is_empty() => Ok(Character {
                name,
                age: self.age,
                health: self.health,
                inventory: self.inventory,
                origin: self.origin,
            }),
            _ => Err("Can't make a character without a name."),
        }
    }

    pub fn try_random_build() -> Character {
        let mut rng = rand::rng();
        let names = [
            "Kael", "Lyra", "Zeph", "Oryn", "Vela", "Dax", "Nera", "Cass", "Juno", "Riven",
        ];

        let name = names[rng.random_range(0..names.len())].to_string();
        let age = rng.random_range(18..=120);
        let health = rng.random_range(20..=100);

        // Add random inventory items
        let inventory_count = rng.random_range(0..=3);
        let mut inventory: Vec<InventoryItem> = Vec::new();
        for _ in 0..inventory_count {
            let inventory_item =
                InventoryItem::from_repr(rng.random_range(0..InventoryItem::COUNT))
                    .unwrap_or(InventoryItem::MedPack);

            if !inventory.contains(&inventory_item) {
                inventory.push(inventory_item);
            }
        }

        let planet =
            Planet::from_repr(rng.random_range(0..Planet::COUNT)).unwrap_or(Planet::Earth);

        Character {
            name,
            age,
            health,
            inventory,
            origin: planet,
        }
    }
}
