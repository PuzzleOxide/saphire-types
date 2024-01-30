use either::Either;
use serde_json::Value;
use crate::types::*;
use crate::block::block_types::subactions::*;
pub enum Repeat {
    RepeatAdjacently {
        gets_the_current: Variable,
        center_block: Location,
        change_location_rotation_tag: ChangeLocationRotationRepeatAdjacently,
        include_origin_block_tag: IncludeOriginBlockRepeatAdjacently,
        pattern_tag: PatternRepeatAdjacently,
    },
    RepeatOnPath {
        gets_the_current: Variable,
        path_locations: Vec<Location>,
        point_spacing: Option<Number>,
        rotate_location_tag: RotateLocationRepeatOnPath,
    },
    RepeatMultipleTimes { gets_the: Option<Variable>, amount: Number },
    RepeatOnGrid {
        gets_the_current: Variable,
        start_of_region: Location,
        end_of_region: Location,
    },
    RepeatWhile { subaction: AllSubactions },
    Range {},
    RepeatForEachinList {
        gets_the_current: Variable,
        list_to_repeat_through: List,
        allow_list_changes_tag: AllowListChangesRepeatForEachinList,
    },
    RepeatOnSphere {
        gets_the_current: Variable,
        sphere_center: Location,
        sphere_radius: Number,
        sphere_points: Option<Number>,
        point_locations_inwards_tag: PointLocationsInwardsRepeatOnSphere,
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
            Repeat::RepeatAdjacently {
                gets_the_current,
                center_block,
                change_location_rotation_tag,
                include_origin_block_tag,
                pattern_tag,
            } => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(
                    vec![gets_the_current.json(), center_block.json()],
                    vec![
                        change_location_rotation_tag.json(), include_origin_block_tag
                        .json(), pattern_tag.json()
                    ],
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
            Repeat::RepeatOnPath {
                gets_the_current,
                path_locations,
                point_spacing,
                rotate_location_tag,
            } => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(
                    vec![
                        gets_the_current.json(), path_locations.json(), point_spacing
                        .json()
                    ],
                    vec![rotate_location_tag.json()],
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
                let mut item_args = compile(
                    vec![gets_the.json(), amount.json()],
                    vec![],
                );
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
                let mut item_args = compile(
                    vec![
                        gets_the_current.json(), start_of_region.json(), end_of_region
                        .json()
                    ],
                    vec![],
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
                let mut item_args = compile(vec![], vec![]);
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
                let mut item_args = compile(vec![], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("Range".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            Repeat::RepeatForEachinList {
                gets_the_current,
                list_to_repeat_through,
                allow_list_changes_tag,
            } => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(
                    vec![gets_the_current.json(), list_to_repeat_through.json()],
                    vec![allow_list_changes_tag.json()],
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
                point_locations_inwards_tag,
            } => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(
                    vec![
                        gets_the_current.json(), sphere_center.json(), sphere_radius
                        .json(), sphere_points.json()
                    ],
                    vec![point_locations_inwards_tag.json()],
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
                let mut item_args = compile(vec![], vec![]);
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
                let mut item_args = compile(
                    vec![
                        gets_the_current.json(), start_of_range.json(), end_of_range
                        .json(), step.json()
                    ],
                    vec![],
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
                let mut item_args = compile(
                    vec![
                        gets_the_current_bkey.json(), gets_the_current_xffd47fvalue
                        .json(), dictionary_to.json()
                    ],
                    vec![],
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
#[derive(Debug, Clone)]
pub enum ChangeLocationRotationRepeatAdjacently {
    True,
    False,
}
impl ChangeLocationRotationRepeatAdjacently {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                ChangeLocationRotationRepeatAdjacently::True => {
                    Value::String("True".to_string())
                }
                ChangeLocationRotationRepeatAdjacently::False => {
                    Value::String("False".to_string())
                }
            },
        );
        data.insert(
            "tag".to_string(),
            Value::String("Change Location Rotation".to_string()),
        );
        data.insert("action".to_string(), Value::String("Adjacent".to_string()));
        data.insert("block".to_string(), Value::String("Adjacent".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for ChangeLocationRotationRepeatAdjacently {
    fn default() -> Self {
        Self::False
    }
}
#[derive(Debug, Clone)]
pub enum IncludeOriginBlockRepeatAdjacently {
    True,
    False,
}
impl IncludeOriginBlockRepeatAdjacently {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                IncludeOriginBlockRepeatAdjacently::True => {
                    Value::String("True".to_string())
                }
                IncludeOriginBlockRepeatAdjacently::False => {
                    Value::String("False".to_string())
                }
            },
        );
        data.insert(
            "tag".to_string(),
            Value::String("Include Origin Block".to_string()),
        );
        data.insert("action".to_string(), Value::String("Adjacent".to_string()));
        data.insert("block".to_string(), Value::String("Adjacent".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for IncludeOriginBlockRepeatAdjacently {
    fn default() -> Self {
        Self::False
    }
}
#[derive(Debug, Clone)]
pub enum PatternRepeatAdjacently {
    CardinalFourblocks,
    SquareEightblocks,
    AdjacentSixblocks,
    CubeTwoSixblocks,
}
impl PatternRepeatAdjacently {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                PatternRepeatAdjacently::CardinalFourblocks => {
                    Value::String("Cardinal (4 blocks)".to_string())
                }
                PatternRepeatAdjacently::SquareEightblocks => {
                    Value::String("Square (8 blocks)".to_string())
                }
                PatternRepeatAdjacently::AdjacentSixblocks => {
                    Value::String("Adjacent (6 blocks)".to_string())
                }
                PatternRepeatAdjacently::CubeTwoSixblocks => {
                    Value::String("Cube (26 blocks)".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Pattern".to_string()));
        data.insert("action".to_string(), Value::String("Adjacent".to_string()));
        data.insert("block".to_string(), Value::String("Adjacent".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for PatternRepeatAdjacently {
    fn default() -> Self {
        Self::AdjacentSixblocks
    }
}
#[derive(Debug, Clone)]
pub enum RotateLocationRepeatOnPath {
    True,
    False,
}
impl RotateLocationRepeatOnPath {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                RotateLocationRepeatOnPath::True => Value::String("True".to_string()),
                RotateLocationRepeatOnPath::False => Value::String("False".to_string()),
            },
        );
        data.insert("tag".to_string(), Value::String("Rotate Location".to_string()));
        data.insert("action".to_string(), Value::String("Path".to_string()));
        data.insert("block".to_string(), Value::String("Path".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for RotateLocationRepeatOnPath {
    fn default() -> Self {
        Self::False
    }
}
#[derive(Debug, Clone)]
pub enum AllowListChangesRepeatForEachinList {
    True,
    Falsecopylist,
}
impl AllowListChangesRepeatForEachinList {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                AllowListChangesRepeatForEachinList::True => {
                    Value::String("True".to_string())
                }
                AllowListChangesRepeatForEachinList::Falsecopylist => {
                    Value::String("False (copy list)".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Allow List Changes".to_string()));
        data.insert("action".to_string(), Value::String("ForEach".to_string()));
        data.insert("block".to_string(), Value::String("ForEach".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for AllowListChangesRepeatForEachinList {
    fn default() -> Self {
        Self::True
    }
}
#[derive(Debug, Clone)]
pub enum PointLocationsInwardsRepeatOnSphere {
    True,
    False,
}
impl PointLocationsInwardsRepeatOnSphere {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                PointLocationsInwardsRepeatOnSphere::True => {
                    Value::String("True".to_string())
                }
                PointLocationsInwardsRepeatOnSphere::False => {
                    Value::String("False".to_string())
                }
            },
        );
        data.insert(
            "tag".to_string(),
            Value::String("Point Locations Inwards".to_string()),
        );
        data.insert("action".to_string(), Value::String("Sphere".to_string()));
        data.insert("block".to_string(), Value::String("Sphere".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for PointLocationsInwardsRepeatOnSphere {
    fn default() -> Self {
        Self::False
    }
}
