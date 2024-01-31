use super::*;

pub enum BlockTag {
    Literal(Text),
    Variable(VariableLiteral),
}

impl Literal for BlockTag {
    fn json(&self) -> Vec<Map<String, Value>> {
        match self {
            BlockTag::Literal(literal) => literal.json(),
            BlockTag::Variable(variable) => variable.json(),
        }
    }
}