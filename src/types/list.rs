use super::*;

pub struct List(VariableLiteral);

impl Literal for List {
    fn json(&self) -> Vec<Map<String, Value>> {
        self.0.json()
    }
}