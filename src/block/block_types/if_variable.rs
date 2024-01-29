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
    },
    StringStartsWith { string_to_check: Text, string_to_start_with: Vec<Text> },
    ListValueEquals {
        list_to_check_in: List,
        index_to_check_at: Number,
        variable_to: Vec<AnyType>,
    },
    ValueIsType { value_to_check: AnyType },
    TextMatches {},
    IsNear {},
    ValueIsWithinRange {
        check_value: AnyType,
        minimum_value: AnyType,
        maximum_value: AnyType,
    },
    VariableExists { variable_to_check: Variable },
    LegacyEq {},
    ItemEquals { item_to_check: Item, items_to_compare_to: Vec<Item> },
    ListContainsValue { list_to_check_in: List, value_to_find: Vec<AnyType> },
    LegacyEqN {},
    InRange {},
    LocationIsNear {
        location_to_check: Location,
        locations_to: Vec<Location>,
        radius: Number,
    },
    StringContains { string_to_check: Text, string_to_check_for: Vec<Text> },
    ValueEq { value_to_check: AnyType, values_to_compare_to: Vec<AnyType> },
    NumberLessThan { number_to_check: Number, number_to_compare_to: Number },
    Eq { value_to_check: AnyType, values_to_compare_to: Vec<AnyType> },
    NumberGreaterThan { number_to_check: Number, number_to_compare_to: Number },
    StringEndsWith { string_to_check: Text, string_to_end_with: Vec<Text> },
    GreaterThanEq { number_to_check: Number, number_to_compare_to: Number },
    DictionaryHasKey { dictionary_to_check: Dict, key_to_look_for: Text },
}
impl IfVariable {
    pub fn compile(&self) -> Value {
        match self {
            IfVariable::Number { number_to_check, number_to_compare_to } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![number_to_check.json(), number_to_compare_to.json()],
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
                let item_args = compile(
                    vec![
                        dictionary_to_check.json(), key_to_check.json(), values_to.json()
                    ],
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
                let item_args = compile(
                    vec![item_to_check.json(), tag_name.json(), tag_value.json()],
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
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        string_or_source_expression_to_match.json(), string_to_compare
                        .json()
                    ],
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
            IfVariable::StringStartsWith { string_to_check, string_to_start_with } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![string_to_check.json(), string_to_start_with.json()],
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
                let item_args = compile(
                    vec![
                        list_to_check_in.json(), index_to_check_at.json(), variable_to
                        .json()
                    ],
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
            IfVariable::ValueIsType { value_to_check } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![value_to_check.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("VarIsType".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            IfVariable::TextMatches {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
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
                let item_args = compile(vec![]);
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
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![check_value.json(), minimum_value.json(), maximum_value.json()],
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
                let item_args = compile(vec![variable_to_check.json()]);
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
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("Legacy !=".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            IfVariable::ItemEquals { item_to_check, items_to_compare_to } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![item_to_check.json(), items_to_compare_to.json()],
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
            IfVariable::ListContainsValue { list_to_check_in, value_to_find } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![list_to_check_in.json(), value_to_find.json()],
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
                let item_args = compile(vec![]);
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
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("InRange".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            IfVariable::LocationIsNear { location_to_check, locations_to, radius } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![location_to_check.json(), locations_to.json(), radius.json()],
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
            IfVariable::StringContains { string_to_check, string_to_check_for } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![string_to_check.json(), string_to_check_for.json()],
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
                let item_args = compile(
                    vec![value_to_check.json(), values_to_compare_to.json()],
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
                let item_args = compile(
                    vec![number_to_check.json(), number_to_compare_to.json()],
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
                let item_args = compile(
                    vec![value_to_check.json(), values_to_compare_to.json()],
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
                let item_args = compile(
                    vec![number_to_check.json(), number_to_compare_to.json()],
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
            IfVariable::StringEndsWith { string_to_check, string_to_end_with } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![string_to_check.json(), string_to_end_with.json()],
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
                let item_args = compile(
                    vec![number_to_check.json(), number_to_compare_to.json()],
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
                let item_args = compile(
                    vec![dictionary_to_check.json(), key_to_look_for.json()],
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
