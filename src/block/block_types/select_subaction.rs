use super::{
    if_player::IfPlayer,
    if_entity::IfEntity,
    if_variable::IfVariable,
};

pub enum SelectSubAction {
    IfPlayer(IfPlayer),
    IfEntity(IfEntity),
    IfVariable(IfVariable),
}