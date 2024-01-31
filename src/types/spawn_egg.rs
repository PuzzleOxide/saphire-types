use super::*;

pub enum SpawnEgg {
    Item(Item),
    Variable(VariableLiteral),
}

impl Literal for SpawnEgg {
    fn json(&self) -> Vec<Map<String, Value>> {
        match self {
            SpawnEgg::Item(i) => i.json(),
            SpawnEgg::Variable(v) => v.json(),
        }
    }
}