use either::Either;

impl<L, R> Argument for Either<L, R> where L: Argument, R: Argument {
    fn json(&self) -> String {
        match self {
            Either::Left(l) => l.json(),
            Either::Right(r) => r.json(),
        }
    }
}

/// A trait for any litteral value that can be used as an argument item.
pub(crate) trait Argument {
    /// Must return a json string representing the argument.
    fn json(&self) -> String;

    fn compile(&self, slot: usize) -> String {
        format!("\"item\":{}, \"slot\":{}", self.json(), slot)
    }
}

pub struct Number {
    pub value: String,
}

pub struct Text {
    pub value: String,
}

pub struct MiniMessage {
    pub value: String,
}

pub struct Location {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub pitch: f64,
    pub yaw: f64,
}

pub struct Item {
    pub item: String,
}

pub struct Particle {
    pub particle: String,
}

pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub struct Sound {
    pub sound: String,
}

pub struct Block {
    pub block: String,
}

pub struct BlockTag {
    pub block_tag: String,
}

pub struct Projectile {
    pub projectile: String,
}

pub struct Potion {
    pub potion: String,
}

pub struct SpawnEgg {
    pub spawn_egg: String,
}

pub struct EntityType {
    pub entity_type: String,
}

pub struct Variable {
    pub variable: String,
}

pub struct AnyType {
    pub any_type: String,
}

pub struct Dict {
    pub dict: String,
}

pub struct List {
    pub list: String,
}

pub struct Vehicle {
    pub vehicle: String,
}