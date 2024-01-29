use super::{
    if_player::IfPlayer,
    if_entity::IfEntity,
    if_variable::IfVariable,
    if_game::IfGame,
};
use serde_json::Value;

pub enum SelectPlayer {
    IfPlayer(IfPlayer),
    IfGame(IfGame),
    IfVariable(IfVariable),
}

impl SelectPlayer {
    pub fn compile(&self) -> Value {
        match self {
            SelectPlayer::IfPlayer(if_player) => if_player.compile(),
            SelectPlayer::IfGame(if_game) => if_game.compile(),
            SelectPlayer::IfVariable(if_variable) => if_variable.compile(),
        }
    }
}

pub enum SelectEntity {
    IfGame(IfGame),
    IfEntity(IfEntity),
    IfVariable(IfVariable),
}

impl SelectEntity {
    pub fn compile(&self) -> Value {
        match self {
            SelectEntity::IfGame(if_game) => if_game.compile(),
            SelectEntity::IfEntity(if_entity) => if_entity.compile(),
            SelectEntity::IfVariable(if_variable) => if_variable.compile(),
        }
    }
}

pub enum AllSubactions {
    IfGame(IfGame),
    IfVariable(IfVariable),
    IfPlayer(IfPlayer),
    IfEntity(IfEntity),
}

impl AllSubactions {
    pub fn compile(&self) -> Value {
        match self {
            AllSubactions::IfGame(if_game) => if_game.compile(),
            AllSubactions::IfVariable(if_variable) => if_variable.compile(),
            AllSubactions::IfPlayer(if_player) => if_player.compile(),
            AllSubactions::IfEntity(if_entity) => if_entity.compile(),
        }
    }
}