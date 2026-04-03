# Exosphara

A Rust-based narrative RPG world-building framework set in the year 2182. Earth has discovered three previously hidden planets — **Exosphara (Exo)**, **Dextronamous (Dextro)**, and **Indo** — and the player must navigate the political fallout through an allegiance-driven storyline.

> *"Dextro, Indo, Exo are the powers that be — always remember these, or D.I.E."*

## Overview

Exosphara models a sci-fi universe with factions, characters, inventories, and an allegiance system. The project explores Rust patterns like enums, builder types, and random generation in the context of interactive fiction.

### Core Mechanics

- **Allegiance System (D.I.E.)** — Four factions (Earth, Exosphara, Dextronamous, Indo) with trust meters shaped by player choices
- **Character System** — Builder-pattern character creation with names, stats, inventory, and planetary origin
- **Inventory & Artifacts** — Items ranging from shields and med packs to the legendary Excallibre
- **Random Generation** — Procedural character creation for NPCs

## Project Structure

```
src/
├── main.rs             # Entry point demonstrating character creation
├── character.rs        # Character struct and CharacterBuilder
├── inventory_item.rs   # Inventory item enum
└── planet.rs           # Planet/faction enum
```

## Getting Started

```sh
# Build
cargo build

# Run
cargo run
```

Requires Rust 2024 edition.

## Dependencies

| Crate | Purpose |
|-------|---------|
| `rand` | Random number generation |
| `random` | Randomization utilities |
| `strum` / `strum_macros` | Enum derive macros (FromRepr, EnumCount) |

## Example

```rust
use exosphara::character::CharacterBuilder;
use exosphara::inventory_item::InventoryItem;
use exosphara::planet::Planet;

let luther = CharacterBuilder::new()
    .name("Luther")
    .age(37)
    .health(100)
    .origin(Planet::Exo)
    .add_item(InventoryItem::Excallibre)
    .try_build()
    .unwrap();
```

