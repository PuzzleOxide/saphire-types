use either::Either;
use serde_json::Value;
use crate::types::*;
use crate::block::block_types::subactions::*;
pub enum IfEntity {
    IsVehicle {},
    IsGrounded {},
    IsType { spawn_egg: Vec<EntityType> },
    IsProjectile {},
    IsMob {},
    HasCustomTag { tag_name: Text, tag_value: Option<Either<Number, Text>> },
    IsSheared {},
    IsItem {},
    IsRidingEntity { spawn_egg: Vec<Either<Either<EntityType, Text>, MiniMessage>> },
    Exists {},
    IsNearLocation {
        center_location: Vec<Location>,
        range: Option<Number>,
        shape_tag: ShapeIsNearLocation,
    },
    HasPotionEffect {
        effects: Vec<Potion>,
        check_properties_tag: CheckPropertiesHasPotionEffect,
        check_mode_tag: CheckModeHasPotionEffect,
    },
    IsRiding { compare_text_to_tag: CompareTextToIsRiding },
    StandingOn {},
    NameEquals { name_to_check_for: Vec<MiniMessage> },
    IsStandingonBlock { block_to_check_for: Vec<Either<Block, Location>> },
}
impl IfEntity {
    pub fn compile(&self) -> Value {
        match self {
            IfEntity::IsVehicle {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("IsVehicle".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            IfEntity::IsGrounded {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("IsGrounded".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            IfEntity::IsType { spawn_egg } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![spawn_egg.json()], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("IsType".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            IfEntity::IsProjectile {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("IsProj".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            IfEntity::IsMob {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("IsMob".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            IfEntity::HasCustomTag { tag_name, tag_value } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![tag_name.json(), tag_value.json()], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("HasCustomTag".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            IfEntity::IsSheared {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("IsSheared".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            IfEntity::IsItem {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("IsItem".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            IfEntity::IsRidingEntity { spawn_egg } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![spawn_egg.json()], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String(" IsRiding ".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            IfEntity::Exists {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("Exists".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            IfEntity::IsNearLocation { center_location, range, shape_tag } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![center_location.json(), range.json()],
                    vec![shape_tag.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("IsNear".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            IfEntity::HasPotionEffect {
                effects,
                check_properties_tag,
                check_mode_tag,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![effects.json()],
                    vec![check_properties_tag.json(), check_mode_tag.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("HasPotion".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            IfEntity::IsRiding { compare_text_to_tag } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![compare_text_to_tag.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("IsRiding".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            IfEntity::StandingOn {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("StandingOn".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            IfEntity::NameEquals { name_to_check_for } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![name_to_check_for.json()], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("NameEquals".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            IfEntity::IsStandingonBlock { block_to_check_for } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![block_to_check_for.json()], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String(" StandingOn ".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
        }
    }
}
#[derive(Debug, Clone)]
pub enum ShapeIsNearLocation {
    Sphere,
    Circle,
    Cube,
    Square,
}
impl ShapeIsNearLocation {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                ShapeIsNearLocation::Sphere => Value::String("Sphere".to_string()),
                ShapeIsNearLocation::Circle => Value::String("Circle".to_string()),
                ShapeIsNearLocation::Cube => Value::String("Cube".to_string()),
                ShapeIsNearLocation::Square => Value::String("Square".to_string()),
            },
        );
        data.insert("tag".to_string(), Value::String("Shape".to_string()));
        data.insert("action".to_string(), Value::String("IsNear".to_string()));
        data.insert("block".to_string(), Value::String("IsNear".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for ShapeIsNearLocation {
    fn default() -> Self {
        Self::Sphere
    }
}
#[derive(Debug, Clone)]
pub enum CheckPropertiesHasPotionEffect {
    None,
    Amplifier,
    Duration,
    Amplifierandduration,
}
impl CheckPropertiesHasPotionEffect {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                CheckPropertiesHasPotionEffect::None => Value::String("None".to_string()),
                CheckPropertiesHasPotionEffect::Amplifier => {
                    Value::String("Amplifier".to_string())
                }
                CheckPropertiesHasPotionEffect::Duration => {
                    Value::String("Duration".to_string())
                }
                CheckPropertiesHasPotionEffect::Amplifierandduration => {
                    Value::String("Amplifier and duration".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Check Properties".to_string()));
        data.insert("action".to_string(), Value::String("HasPotion".to_string()));
        data.insert("block".to_string(), Value::String("HasPotion".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for CheckPropertiesHasPotionEffect {
    fn default() -> Self {
        Self::None
    }
}
#[derive(Debug, Clone)]
pub enum CheckModeHasPotionEffect {
    Hasanyeffect,
    Hasalleffects,
}
impl CheckModeHasPotionEffect {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                CheckModeHasPotionEffect::Hasanyeffect => {
                    Value::String("Has any effect".to_string())
                }
                CheckModeHasPotionEffect::Hasalleffects => {
                    Value::String("Has all effects".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Check Mode".to_string()));
        data.insert("action".to_string(), Value::String("HasPotion".to_string()));
        data.insert("block".to_string(), Value::String("HasPotion".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for CheckModeHasPotionEffect {
    fn default() -> Self {
        Self::Hasanyeffect
    }
}
#[derive(Debug, Clone)]
pub enum CompareTextToIsRiding {
    Entitytype,
    NameorUUID,
}
impl CompareTextToIsRiding {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                CompareTextToIsRiding::Entitytype => {
                    Value::String("Entity type".to_string())
                }
                CompareTextToIsRiding::NameorUUID => {
                    Value::String("Name or UUID".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Compare Text To".to_string()));
        data.insert("action".to_string(), Value::String("IsRiding".to_string()));
        data.insert("block".to_string(), Value::String("IsRiding".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for CompareTextToIsRiding {
    fn default() -> Self {
        Self::Entitytype
    }
}
