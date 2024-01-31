use super::*;

// TODO: Implement sound literal generation from actiondump? Might be able to do this as a string literal
pub enum Sound {
    Variable(VariableLiteral),
}

impl Literal for Sound {
    fn json(&self) -> Vec<Map<String, Value>> {
        match self {
            Sound::Variable(v) => v.json(),
        }
    }
}