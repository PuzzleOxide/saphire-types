use super::*;

pub struct Dict(VariableLiteral);

impl Literal for Dict {
    fn json(&self) -> Vec<Map<String, Value>> {
        self.0.json()
    }
}