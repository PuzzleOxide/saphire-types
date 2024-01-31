use super::*;

// TODO: generate potion literal from actiondump
pub enum Potion {
    Variable(VariableLiteral)
}

impl Literal for Potion {
    fn json(&self) -> Vec<Map<String, Value>> {
        match self {
            Potion::Variable(v) => v.json()
        }
    }
}