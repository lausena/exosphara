use strum_macros::{EnumCount, FromRepr};

#[derive(Debug, FromRepr, EnumCount, PartialEq)]
pub enum Planet {
    Earth,
    Exo,
    Dextro,
    Indo,
}
