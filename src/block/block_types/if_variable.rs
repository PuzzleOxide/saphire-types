use either::Either;
use serde_json::Value;
use crate::types::*;
use crate::block::block_types::subactions::*;
pub enum IfVariable {
    Number { number_to_check: Number, number_to_compare_to: Number },
    DictionaryValueEquals {
        dictionary_to_check: Dict,
        key_to_check: Text,
        values_to: Vec<AnyType>,
    },
    ItemHasCustomTag {
        item_to_check: Item,
        tag_name: Text,
        tag_value: Option<Either<Number, Text>>,
    },
    StringMatches {
        string_or_source_expression_to_match: Text,
        string_to_compare: Vec<Text>,
        regular_expressions_tag: RegularExpressionsStringMatches,
        ignore_case_tag: IgnoreCaseStringMatches,
    },
    StringStartsWith {
        string_to_check: Text,
        string_to_start_with: Vec<Text>,
        ignore_case_tag: IgnoreCaseStringStartsWith,
    },
    ListValueEquals {
        list_to_check_in: List,
        index_to_check_at: Number,
        variable_to: Vec<AnyType>,
    },
    ValueIsType { value_to_check: AnyType, variable_type__tag: VariableTypeValueIsType },
    TextMatches {
        regular_expressions_tag: RegularExpressionsTextMatches,
        ignore_case_tag: IgnoreCaseTextMatches,
    },
    IsNear {},
    ValueIsWithinRange {
        check_value: AnyType,
        minimum_value: AnyType,
        maximum_value: AnyType,
        location_handling_tag: LocationHandlingValueIsWithinRange,
    },
    VariableExists { variable_to_check: Variable },
    LegacyEq {},
    ItemEquals {
        item_to_check: Item,
        items_to_compare_to: Vec<Item>,
        comparison_mode_tag: ComparisonModeItemEquals,
    },
    ListContainsValue {
        list_to_check_in: List,
        value_to_find: Vec<AnyType>,
        check_mode_tag: CheckModeListContainsValue,
    },
    LegacyEqN {},
    InRange {},
    LocationIsNear {
        location_to_check: Location,
        locations_to: Vec<Location>,
        radius: Number,
        shape_tag: ShapeLocationIsNear,
    },
    StringContains {
        string_to_check: Text,
        string_to_check_for: Vec<Text>,
        ignore_case_tag: IgnoreCaseStringContains,
    },
    ValueEq { value_to_check: AnyType, values_to_compare_to: Vec<AnyType> },
    NumberLessThan { number_to_check: Number, number_to_compare_to: Number },
    Eq { value_to_check: AnyType, values_to_compare_to: Vec<AnyType> },
    NumberGreaterThan { number_to_check: Number, number_to_compare_to: Number },
    StringEndsWith {
        string_to_check: Text,
        string_to_end_with: Vec<Text>,
        ignore_case_tag: IgnoreCaseStringEndsWith,
    },
    GreaterThanEq { number_to_check: Number, number_to_compare_to: Number },
    DictionaryHasKey { dictionary_to_check: Dict, key_to_look_for: Text },
}
impl IfVariable {
    pub fn compile(&self) -> Value {
        match self {
            IfVariable::Number { number_to_check, number_to_compare_to } => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(
                    vec![number_to_check.json(), number_to_compare_to.json()],
                    vec![],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("<=".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            IfVariable::DictionaryValueEquals {
                dictionary_to_check,
                key_to_check,
                values_to,
            } => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(
                    vec![
                        dictionary_to_check.json(), key_to_check.json(), values_to.json()
                    ],
                    vec![],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("DictValueEquals".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            IfVariable::ItemHasCustomTag { item_to_check, tag_name, tag_value } => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(
                    vec![item_to_check.json(), tag_name.json(), tag_value.json()],
                    vec![],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("ItemHasTag".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            IfVariable::StringMatches {
                string_or_source_expression_to_match,
                string_to_compare,
                regular_expressions_tag,
                ignore_case_tag,
            } => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(
                    vec![
                        string_or_source_expression_to_match.json(), string_to_compare
                        .json()
                    ],
                    vec![regular_expressions_tag.json(), ignore_case_tag.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("StringMatches".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            IfVariable::StringStartsWith {
                string_to_check,
                string_to_start_with,
                ignore_case_tag,
            } => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(
                    vec![string_to_check.json(), string_to_start_with.json()],
                    vec![ignore_case_tag.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("StartsWith".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            IfVariable::ListValueEquals {
                list_to_check_in,
                index_to_check_at,
                variable_to,
            } => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(
                    vec![
                        list_to_check_in.json(), index_to_check_at.json(), variable_to
                        .json()
                    ],
                    vec![],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("ListValueEq".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            IfVariable::ValueIsType { value_to_check, variable_type__tag } => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(
                    vec![value_to_check.json()],
                    vec![variable_type__tag.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("VarIsType".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            IfVariable::TextMatches { regular_expressions_tag, ignore_case_tag } => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(
                    vec![],
                    vec![regular_expressions_tag.json(), ignore_case_tag.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("TextMatches".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            IfVariable::IsNear {} => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(vec![], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("IsNear".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            IfVariable::ValueIsWithinRange {
                check_value,
                minimum_value,
                maximum_value,
                location_handling_tag,
            } => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(
                    vec![check_value.json(), minimum_value.json(), maximum_value.json()],
                    vec![location_handling_tag.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String(" InRange ".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            IfVariable::VariableExists { variable_to_check } => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(vec![variable_to_check.json()], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("VarExists".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            IfVariable::LegacyEq {} => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(vec![], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("Legacy !=".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            IfVariable::ItemEquals {
                item_to_check,
                items_to_compare_to,
                comparison_mode_tag,
            } => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(
                    vec![item_to_check.json(), items_to_compare_to.json()],
                    vec![comparison_mode_tag.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("ItemEquals".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            IfVariable::ListContainsValue {
                list_to_check_in,
                value_to_find,
                check_mode_tag,
            } => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(
                    vec![list_to_check_in.json(), value_to_find.json()],
                    vec![check_mode_tag.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("ListContains".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            IfVariable::LegacyEqN {} => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(vec![], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("Legacy =".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            IfVariable::InRange {} => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(vec![], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("InRange".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            IfVariable::LocationIsNear {
                location_to_check,
                locations_to,
                radius,
                shape_tag,
            } => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(
                    vec![location_to_check.json(), locations_to.json(), radius.json()],
                    vec![shape_tag.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("LocIsNear".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            IfVariable::StringContains {
                string_to_check,
                string_to_check_for,
                ignore_case_tag,
            } => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(
                    vec![string_to_check.json(), string_to_check_for.json()],
                    vec![ignore_case_tag.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("Contains".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            IfVariable::ValueEq { value_to_check, values_to_compare_to } => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(
                    vec![value_to_check.json(), values_to_compare_to.json()],
                    vec![],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("!=".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            IfVariable::NumberLessThan { number_to_check, number_to_compare_to } => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(
                    vec![number_to_check.json(), number_to_compare_to.json()],
                    vec![],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("<".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            IfVariable::Eq { value_to_check, values_to_compare_to } => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(
                    vec![value_to_check.json(), values_to_compare_to.json()],
                    vec![],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("=".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            IfVariable::NumberGreaterThan { number_to_check, number_to_compare_to } => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(
                    vec![number_to_check.json(), number_to_compare_to.json()],
                    vec![],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String(">".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            IfVariable::StringEndsWith {
                string_to_check,
                string_to_end_with,
                ignore_case_tag,
            } => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(
                    vec![string_to_check.json(), string_to_end_with.json()],
                    vec![ignore_case_tag.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("EndsWith".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            IfVariable::GreaterThanEq { number_to_check, number_to_compare_to } => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(
                    vec![number_to_check.json(), number_to_compare_to.json()],
                    vec![],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String(">=".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            IfVariable::DictionaryHasKey { dictionary_to_check, key_to_look_for } => {
                let mut map = serde_json::Map::new();
                let mut item_args = compile(
                    vec![dictionary_to_check.json(), key_to_look_for.json()],
                    vec![],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("DictHasKey".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
        }
    }
}
#[derive(Debug, Clone)]
pub enum RegularExpressionsStringMatches {
    Enable,
    Disable,
}
impl RegularExpressionsStringMatches {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                RegularExpressionsStringMatches::Enable => {
                    Value::String("Enable".to_string())
                }
                RegularExpressionsStringMatches::Disable => {
                    Value::String("Disable".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Regular Expressions".to_string()));
        data.insert("action".to_string(), Value::String("StringMatches".to_string()));
        data.insert("block".to_string(), Value::String("StringMatches".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for RegularExpressionsStringMatches {
    fn default() -> Self {
        Self::Disable
    }
}
#[derive(Debug, Clone)]
pub enum IgnoreCaseStringMatches {
    True,
    False,
}
impl IgnoreCaseStringMatches {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                IgnoreCaseStringMatches::True => Value::String("True".to_string()),
                IgnoreCaseStringMatches::False => Value::String("False".to_string()),
            },
        );
        data.insert("tag".to_string(), Value::String("Ignore Case".to_string()));
        data.insert("action".to_string(), Value::String("StringMatches".to_string()));
        data.insert("block".to_string(), Value::String("StringMatches".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for IgnoreCaseStringMatches {
    fn default() -> Self {
        Self::False
    }
}
#[derive(Debug, Clone)]
pub enum IgnoreCaseStringStartsWith {
    True,
    False,
}
impl IgnoreCaseStringStartsWith {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                IgnoreCaseStringStartsWith::True => Value::String("True".to_string()),
                IgnoreCaseStringStartsWith::False => Value::String("False".to_string()),
            },
        );
        data.insert("tag".to_string(), Value::String("Ignore Case".to_string()));
        data.insert("action".to_string(), Value::String("StartsWith".to_string()));
        data.insert("block".to_string(), Value::String("StartsWith".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for IgnoreCaseStringStartsWith {
    fn default() -> Self {
        Self::False
    }
}
#[derive(Debug, Clone)]
pub enum VariableTypeValueIsType {
    Number,
    String,
    StyledText,
    Location,
    Item,
    List,
    Potioneffect,
    Sound,
    Particle,
    Vector,
    Dictionary,
}
impl VariableTypeValueIsType {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                VariableTypeValueIsType::Number => Value::String("Number".to_string()),
                VariableTypeValueIsType::String => Value::String("String".to_string()),
                VariableTypeValueIsType::StyledText => {
                    Value::String("Styled Text".to_string())
                }
                VariableTypeValueIsType::Location => {
                    Value::String("Location".to_string())
                }
                VariableTypeValueIsType::Item => Value::String("Item".to_string()),
                VariableTypeValueIsType::List => Value::String("List".to_string()),
                VariableTypeValueIsType::Potioneffect => {
                    Value::String("Potion effect".to_string())
                }
                VariableTypeValueIsType::Sound => Value::String("Sound".to_string()),
                VariableTypeValueIsType::Particle => {
                    Value::String("Particle".to_string())
                }
                VariableTypeValueIsType::Vector => Value::String("Vector".to_string()),
                VariableTypeValueIsType::Dictionary => {
                    Value::String("Dictionary".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Variable Type".to_string()));
        data.insert("action".to_string(), Value::String("VarIsType".to_string()));
        data.insert("block".to_string(), Value::String("VarIsType".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for VariableTypeValueIsType {
    fn default() -> Self {
        Self::Number
    }
}
#[derive(Debug, Clone)]
pub enum RegularExpressionsTextMatches {
    Enable,
    Disable,
}
impl RegularExpressionsTextMatches {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                RegularExpressionsTextMatches::Enable => {
                    Value::String("Enable".to_string())
                }
                RegularExpressionsTextMatches::Disable => {
                    Value::String("Disable".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Regular Expressions".to_string()));
        data.insert("action".to_string(), Value::String("TextMatches".to_string()));
        data.insert("block".to_string(), Value::String("TextMatches".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for RegularExpressionsTextMatches {
    fn default() -> Self {
        Self::Disable
    }
}
#[derive(Debug, Clone)]
pub enum IgnoreCaseTextMatches {
    True,
    False,
}
impl IgnoreCaseTextMatches {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                IgnoreCaseTextMatches::True => Value::String("True".to_string()),
                IgnoreCaseTextMatches::False => Value::String("False".to_string()),
            },
        );
        data.insert("tag".to_string(), Value::String("Ignore Case".to_string()));
        data.insert("action".to_string(), Value::String("TextMatches".to_string()));
        data.insert("block".to_string(), Value::String("TextMatches".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for IgnoreCaseTextMatches {
    fn default() -> Self {
        Self::False
    }
}
#[derive(Debug, Clone)]
pub enum LocationHandlingValueIsWithinRange {
    Block,
    Exact,
}
impl LocationHandlingValueIsWithinRange {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                LocationHandlingValueIsWithinRange::Block => {
                    Value::String("Block".to_string())
                }
                LocationHandlingValueIsWithinRange::Exact => {
                    Value::String("Exact".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Location Handling".to_string()));
        data.insert("action".to_string(), Value::String(" InRange ".to_string()));
        data.insert("block".to_string(), Value::String(" InRange ".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for LocationHandlingValueIsWithinRange {
    fn default() -> Self {
        Self::Exact
    }
}
#[derive(Debug, Clone)]
pub enum ComparisonModeItemEquals {
    Exactlyequals,
    Ignorestacksize,
    Ignoredurabilityandstacksize,
    Materialonly,
}
impl ComparisonModeItemEquals {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                ComparisonModeItemEquals::Exactlyequals => {
                    Value::String("Exactly equals".to_string())
                }
                ComparisonModeItemEquals::Ignorestacksize => {
                    Value::String("Ignore stack size".to_string())
                }
                ComparisonModeItemEquals::Ignoredurabilityandstacksize => {
                    Value::String("Ignore durability and stack size".to_string())
                }
                ComparisonModeItemEquals::Materialonly => {
                    Value::String("Material only".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Comparison Mode".to_string()));
        data.insert("action".to_string(), Value::String("ItemEquals".to_string()));
        data.insert("block".to_string(), Value::String("ItemEquals".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for ComparisonModeItemEquals {
    fn default() -> Self {
        Self::Exactlyequals
    }
}
#[derive(Debug, Clone)]
pub enum CheckModeListContainsValue {
    HasAnyValue,
    HasAllValues,
}
impl CheckModeListContainsValue {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                CheckModeListContainsValue::HasAnyValue => {
                    Value::String("Has Any Value".to_string())
                }
                CheckModeListContainsValue::HasAllValues => {
                    Value::String("Has All Values".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Check Mode".to_string()));
        data.insert("action".to_string(), Value::String("ListContains".to_string()));
        data.insert("block".to_string(), Value::String("ListContains".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for CheckModeListContainsValue {
    fn default() -> Self {
        Self::HasAnyValue
    }
}
#[derive(Debug, Clone)]
pub enum ShapeLocationIsNear {
    Sphere,
    Circle,
    Cube,
    Square,
}
impl ShapeLocationIsNear {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                ShapeLocationIsNear::Sphere => Value::String("Sphere".to_string()),
                ShapeLocationIsNear::Circle => Value::String("Circle".to_string()),
                ShapeLocationIsNear::Cube => Value::String("Cube".to_string()),
                ShapeLocationIsNear::Square => Value::String("Square".to_string()),
            },
        );
        data.insert("tag".to_string(), Value::String("Shape".to_string()));
        data.insert("action".to_string(), Value::String("LocIsNear".to_string()));
        data.insert("block".to_string(), Value::String("LocIsNear".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for ShapeLocationIsNear {
    fn default() -> Self {
        Self::Sphere
    }
}
#[derive(Debug, Clone)]
pub enum IgnoreCaseStringContains {
    True,
    False,
}
impl IgnoreCaseStringContains {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                IgnoreCaseStringContains::True => Value::String("True".to_string()),
                IgnoreCaseStringContains::False => Value::String("False".to_string()),
            },
        );
        data.insert("tag".to_string(), Value::String("Ignore Case".to_string()));
        data.insert("action".to_string(), Value::String("Contains".to_string()));
        data.insert("block".to_string(), Value::String("Contains".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for IgnoreCaseStringContains {
    fn default() -> Self {
        Self::False
    }
}
#[derive(Debug, Clone)]
pub enum IgnoreCaseStringEndsWith {
    True,
    False,
}
impl IgnoreCaseStringEndsWith {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                IgnoreCaseStringEndsWith::True => Value::String("True".to_string()),
                IgnoreCaseStringEndsWith::False => Value::String("False".to_string()),
            },
        );
        data.insert("tag".to_string(), Value::String("Ignore Case".to_string()));
        data.insert("action".to_string(), Value::String("EndsWith".to_string()));
        data.insert("block".to_string(), Value::String("EndsWith".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for IgnoreCaseStringEndsWith {
    fn default() -> Self {
        Self::False
    }
}
