use super::*;

pub enum Vehicle {
    Item(Item),
    Variable(VariableLiteral),
}

impl Literal for Vehicle {
    fn json(&self) -> Vec<Map<String, Value>> {
        match self {
            Vehicle::Item(i) => i.json(),
            Vehicle::Variable(v) => v.json(),
        }
    }
}