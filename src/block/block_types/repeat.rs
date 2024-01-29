use either::Either;
use serde_json::Value;
use crate::types::*;
use crate::block::block_types::subactions::*;
pub enum Repeat {
    RepeatAdjacently { gets_the_current: Variable, center_block: Location },
    RepeatOnPath {
        gets_the_current: Variable,
        path_locations: Vec<Location>,
        point_spacing: Option<Number>,
    },
    RepeatMultipleTimes { gets_the: Option<Variable>, amount: Number },
    RepeatOnGrid {
        gets_the_current: Variable,
        start_of_region: Location,
        end_of_region: Location,
    },
    RepeatWhile { subaction: AllSubactions },
    Range {},
    RepeatForEachinList { gets_the_current: Variable, list_to_repeat_through: List },
    RepeatOnSphere {
        gets_the_current: Variable,
        sphere_center: Location,
        sphere_radius: Number,
        sphere_points: Option<Number>,
    },
    RepeatForever {},
    RepeatOnRange {
        gets_the_current: Option<Variable>,
        start_of_range: Number,
        end_of_range: Number,
        step: Option<Number>,
    },
    RepeatForEachDictionaryEntry {
        gets_the_current_bkey: Variable,
        gets_the_current_xffd47fvalue: Variable,
        dictionary_to: Dict,
    },
}
impl Repeat {
    pub fn compile(&self) -> Value {
        match self {
            Repeat::RepeatAdjacently { gets_the_current, center_block } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![gets_the_current.json(), center_block.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("Adjacent".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            Repeat::RepeatOnPath { gets_the_current, path_locations, point_spacing } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        gets_the_current.json(), path_locations.json(), point_spacing
                        .json()
                    ],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("Path".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            Repeat::RepeatMultipleTimes { gets_the, amount } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![gets_the.json(), amount.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("Multiple".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            Repeat::RepeatOnGrid {
                gets_the_current,
                start_of_region,
                end_of_region,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        gets_the_current.json(), start_of_region.json(), end_of_region
                        .json()
                    ],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("Grid".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            Repeat::RepeatWhile { subaction } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("While".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                let mut subaction = subaction.compile();
                let value = subaction.as_object_mut().unwrap();
                value.insert("subaction".to_string(), value["action"].clone());
                value
                    .insert(
                        "action".to_string(),
                        serde_json::Value::String("While".to_string()),
                    );
                drop(value);
                subaction
            }
            Repeat::Range {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("Range".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            Repeat::RepeatForEachinList { gets_the_current, list_to_repeat_through } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![gets_the_current.json(), list_to_repeat_through.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("ForEach".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            Repeat::RepeatOnSphere {
                gets_the_current,
                sphere_center,
                sphere_radius,
                sphere_points,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        gets_the_current.json(), sphere_center.json(), sphere_radius
                        .json(), sphere_points.json()
                    ],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("Sphere".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            Repeat::RepeatForever {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("Forever".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            Repeat::RepeatOnRange {
                gets_the_current,
                start_of_range,
                end_of_range,
                step,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        gets_the_current.json(), start_of_range.json(), end_of_range
                        .json(), step.json()
                    ],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String(" Range ".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            Repeat::RepeatForEachDictionaryEntry {
                gets_the_current_bkey,
                gets_the_current_xffd47fvalue,
                dictionary_to,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        gets_the_current_bkey.json(), gets_the_current_xffd47fvalue
                        .json(), dictionary_to.json()
                    ],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("ForEachEntry".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
        }
    }
}
