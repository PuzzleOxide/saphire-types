use super::*;

// TODO: Implement particle literal generation from actiondump
pub enum Particle {
    Variable(VariableLiteral),
}

impl Literal for Particle {
    fn json(&self) -> Vec<Map<String, Value>> {
        match self {
            Particle::Variable(v) => v.json(),
        }
    }
}