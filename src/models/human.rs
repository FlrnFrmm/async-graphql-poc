use async_graphql::{SimpleObject, Enum};

#[derive(SimpleObject)]
pub struct Human {
    id: String,
    name: String,
    appears_in: Vec<Episode>,
    home_planet: String,
}
#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum Episode {
    NewHope,
    Empire,
    Jedi,
}