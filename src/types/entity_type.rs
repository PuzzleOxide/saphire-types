use super::*;

pub enum EntityType {
    SpawnEgg(SpawnEgg),
    Variable(VariableLiteral),
}

impl Literal for EntityType {
    fn json(&self) -> Vec<Map<String, Value>> {
        match self {
            EntityType::SpawnEgg(spawn_egg) => spawn_egg.json(),
            EntityType::Variable(variable) => variable.json(),
        }
    }
}