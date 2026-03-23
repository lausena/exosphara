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
