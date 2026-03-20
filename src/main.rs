use rand::RngExt;
use strum::EnumCount;
use strum_macros::{EnumCount, FromRepr};

#[derive(Debug, FromRepr, EnumCount, PartialEq)]
pub enum InventoryItem {
    Shield,
    MedPack,
    RationPack,
    GrappleHook,
    EmpGrenade,
    WorldMap,
    Excallibre,
}

#[derive(Debug, FromRepr, EnumCount, PartialEq)]
pub enum Planet {
    Earth,
    Exo,
    Dextro,
    Indo,
}

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
    pub name: String,
    pub age: u32,
    pub health: u32,
    pub inventory: Vec<InventoryItem>,
    pub origin: Planet,
}

impl CharacterBuilder {
    fn new(
        name: String,
        age: u32,
        health: u32,
        inventory: Vec<InventoryItem>,
        origin: Planet,
    ) -> Self {
        Self {
            name,
            age,
            health,
            inventory,
            origin,
        }
    }

    fn try_build(self) -> Result<Character, &'static str> {
        if !self.name.is_empty() {
            Ok(Character {
                name: self.name,
                age: self.age,
                health: self.health,
                inventory: self.inventory,
                origin: self.origin,
            })
        } else {
            Err("Can't make a character without a name.")
        }
    }

    fn try_random_build() -> Character {
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

        let planet = Planet::from_repr(rng.random_range(0..Planet::COUNT)).unwrap_or(Planet::Earth);

        Character {
            name,
            age,
            health,
            inventory,
            origin: planet,
        }
    }
}

fn main() {
    use Planet::*;
    let planets = vec![Earth, Exo, Dextro, Indo];

    println!("Planets in the story: {:?}", planets);

    let luther = CharacterBuilder::new(
        "Luther".to_string(),
        37,
        100,
        vec![InventoryItem::Excallibre],
        Exo,
    )
    .try_build();

    println!("Main character: {:?}", luther);

    let my_random_character = CharacterBuilder::try_random_build();
    println!("My random character is: {:?}", my_random_character);
}
