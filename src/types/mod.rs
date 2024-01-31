use either::Either;
use serde_json::{Value, Map};

pub mod number;
pub mod text;
pub mod minimessage;
pub mod location;
pub mod vector;
pub mod item;
pub mod particle;
pub mod sound;
pub mod block;
pub mod block_tag;
pub mod projectile;
pub mod potion;
pub mod spawn_egg;
pub mod entity_type;
pub mod variable;
pub mod dict;
pub mod list;
pub mod vehicle;
pub mod any_type;

pub use number::*;
pub use text::*;
pub use minimessage::*;
pub use location::*;
pub use vector::*;
pub use item::*;
pub use particle::*;
pub use sound::*;
pub use block::*;
pub use block_tag::*;
pub use projectile::*;
pub use potion::*;
pub use spawn_egg::*;
pub use entity_type::*;
pub use variable::*;
pub use dict::*;
pub use list::*;
pub use vehicle::*;
pub use any_type::*;

/// turns a vector of arguments and tags into a vector of json objects
pub fn compile(args: Vec<Vec<Map<String, Value>>>, tags: Vec<Map<String, Value>>) -> Vec<Value> {
    let mut vec = Vec::new();
    for (i, arg)  in args.into_iter().flatten().enumerate() {
        let mut map = Map::new();
        map.insert("slot".to_string(), Value::Number(serde_json::Number::from(i)));
        map.insert("item".to_string(), Value::Object(arg));
        vec.push(Value::Object(map));
    };

    for (tag, i) in tags.iter().zip(26..0) {
        let mut map = Map::new();
        map.insert("slot".to_string(), Value::Number(serde_json::Number::from(i)));
        map.insert("item".to_string(), Value::Object(tag.clone()));
        vec.push(Value::Object(map));
    } 
    vec
}

/// A trait for any literal value that can be used as an argument item.
pub(crate) trait Literal {
    /// Must return a json string representing the argument.
    fn json(&self) -> Vec<Map<String, Value>>;
}

impl<T: Literal> Literal for Vec<T> {
    fn json(&self) -> Vec<Map<String, Value>> {
        let mut vec = Vec::new();
        for item in self {
            vec.extend(item.json());
        }
        vec
    }
}

impl<T: Literal> Literal for Option<T> {
    fn json(&self) -> Vec<Map<String, Value>> {
        match self {
            Some(item) => item.json(),
            None => Vec::new(),
        }
    }
}

impl<L, R> Literal for Either<L, R> where L: Literal, R: Literal {
    fn json(&self) -> Vec<Map<String, Value>> {
        match self {
            Either::Left(l) => l.json(),
            Either::Right(r) => r.json(),
        }
    }
}