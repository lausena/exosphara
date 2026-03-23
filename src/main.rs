mod character;
mod inventory_item;
mod planet;

use character::CharacterBuilder;
use inventory_item::InventoryItem;
use planet::Planet;

fn main() {
    use Planet::*;
    let planets = vec![Earth, Exo, Dextro, Indo];

    println!("Planets in the story: {:?}", planets);

    let luther = CharacterBuilder::new()
        .name("Luther")
        .age(37)
        .health(100)
        .add_item(InventoryItem::Excallibre)
        .origin(Exo)
        .try_build();

    println!("Main character: {:?}", luther);

    // Builder with multiple items via inventory()
    let medic = CharacterBuilder::new()
        .name("Voss")
        .age(25)
        .inventory(vec![InventoryItem::MedPack, InventoryItem::Shield])
        .origin(Dextro)
        .try_build();
    println!("Medic: {:?}", medic);

    // Builder with defaults (only name required)
    let rookie = CharacterBuilder::new()
        .name("Rookie")
        .try_build();
    println!("Rookie: {:?}", rookie);

    // Missing name should fail
    let nameless = CharacterBuilder::new()
        .age(50)
        .health(80)
        .try_build();
    println!("Nameless: {:?}", nameless);

    // Random build
    let my_random_character = CharacterBuilder::try_random_build();
    println!("My random character is: {:?}", my_random_character);
}
