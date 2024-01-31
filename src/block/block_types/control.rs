use either::Either;
use serde_json::Value;
use crate::types::*;
use crate::block::block_types::subactions::*;
pub enum Control {
    StopRepeat {},
    ReturnFromFunction {},
    ReturnNTimes {},
    SkipIteration {},
    EndThread {},
    Wait { wait_duration: Option<Number>, time_unit_tag: TimeUnitWait },
}
impl Control {
    pub fn compile(&self) -> Value {
        match self {
            Control::StopRepeat {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("StopRepeat".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            Control::ReturnFromFunction {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("Return".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            Control::ReturnNTimes {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("ReturnNTimes".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            Control::SkipIteration {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("Skip".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            Control::EndThread {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("End".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            Control::Wait { wait_duration, time_unit_tag } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![wait_duration.json()],
                    vec![time_unit_tag.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("Wait".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
        }
    }
}
#[derive(Debug, Clone)]
pub enum TimeUnitWait {
    Ticks,
    Seconds,
    Minutes,
}
impl TimeUnitWait {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                TimeUnitWait::Ticks => Value::String("Ticks".to_string()),
                TimeUnitWait::Seconds => Value::String("Seconds".to_string()),
                TimeUnitWait::Minutes => Value::String("Minutes".to_string()),
            },
        );
        data.insert("tag".to_string(), Value::String("Time Unit".to_string()));
        data.insert("action".to_string(), Value::String("Wait".to_string()));
        data.insert("block".to_string(), Value::String("Wait".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for TimeUnitWait {
    fn default() -> Self {
        Self::Ticks
    }
}
