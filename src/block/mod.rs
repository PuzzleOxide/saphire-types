pub mod block_types;
pub mod prelude;

use serde_json::{Value, Map, json};

use prelude::*;
use crate::targets::*;

/// A literal Diamond Fire Block, Has a one to one mapping with Diamond Fire Blocks.
// TOOD: Change Args to an Item struct/enum
pub enum Block {
    /// A Player Event, used to run code when a player does something.
    /// Has a one to one mapping with Diamond Fire Player Events.
    PlayerEvent { 
        event: PlayerEvent,
    },
    /// A Player Action, used to interact with players.
    /// Has a one to one mapping with Diamond Fire Player Actions.
    PlayerAction { 
        action: PlayerAction,
        target: PlayerTarget,
    },
    /// An If Player, Used for if statements involving players.
    /// Has a one to one mapping with Diamond Fire If Player.
    IfPlayer { 
        action: IfPlayer,
        target: PlayerTarget,
    },
    /// An Entity Event, used to run code when an entity does something.
    /// Has a one to one mapping with Diamond Fire Entity Events.
    EntityEvent { 
        event: EntityEvent,
    },
    /// An Entity Action, used to interact with entities.
    /// Has a one to one mapping with Diamond Fire Entity Actions.
    EntityAction { 
        action: EntityAction,
        target: EntityTarget,
    },
    /// An If Entity, Used for if statements involving entities.
    /// Has a one to one mapping with Diamond Fire If Entity.
    IfEntity { 
        action: IfEntity,
        target: EntityTarget,
    },
    /// A Game Action, used to interact with Events and the World.
    /// Has a one to one mapping with Diamond Fire Game Actions.
    GameAction { 
        action: GameAction,
    },
    /// An If Game, Used for if statements involving the game.
    /// Has a one to one mapping with Diamond Fire If Game.
    IfGame {
        action: IfGame,
    },
    /// A Set Variable, Used to manipulate variables.
    /// Has a one to one mapping with Diamond Fire Set Variable.
    SetVariable {
        action: SetVariable,
    },
    /// An If Variable, Used for if statements involving variables.
    /// Has a one to one mapping with Diamond Fire If Variable.
    IfVariable {
        action: IfVariable,
    },
    /// An Else. Used for else statements.
    /// Has a one to one mapping with Diamond Fire Else.
    Else,
    /// A Control, Used for control statements.
    /// Has a one to one mapping with Diamond Fire Control.
    Control {
        action: Control,
    },
    /// A Select, Used to change the current selection.
    /// Has a one to one mapping with Diamond Fire Select.
    Select {
        action: SelectObject,
    },
    /// A Repeat, Used to repeat code.
    /// Has a one to one mapping with Diamond Fire Repeat.
    Repeat {
        action: Repeat,
    },
    /// A Function definition, Used to define functions.
    /// Has a one to one mapping with Diamond Fire Function.
    Function {
        data: String,
        args: Vec<String>,
        //bl_tags: FunctionBlockTags,
    },
    /// A Function call, Used to call functions.
    /// Has a one to one mapping with Diamond Fire Call Function.
    CallFunction {
        data: String,
        args: Vec<String>, // TODO: Look at changing to a function call struct
    },
    /// A Process definition, Used to define processes.
    /// Has a one to one mapping with Diamond Fire Process.
    Process {
        data: String,
        args: Vec<String>,
        //bl_tags: ProcessBlockTags,
    },
    /// A Process call, Used to call processes.
    /// Has a one to one mapping with Diamond Fire Call Process.
    CallProcess {
        data: String,
        args: Vec<String>,
        //bl_tags: CallProcessBlockTags,
    },
    /// A Bracket block, Used for loops and if statements.
    Bracket { 
        direct: BracketDirection,
        typ: BracketType,
    }
}

impl Block {
    /// Compile the block into a string.
    pub fn compile(&self) -> String {
        
        match self {
            Block::PlayerEvent { event } => event.compile(),
            Block::PlayerAction { action, target } => {
                let mut value = action.compile();
                value.as_object_mut().unwrap().insert("target".to_string(), Value::String(target.to_string()));
                value
            },
            Block::IfPlayer { action, target } => {
                let mut value = action.compile();
                value.as_object_mut().unwrap().insert("target".to_string(), Value::String(target.to_string()));
                value
            },
            Block::EntityEvent { event } => event.compile(),
            Block::EntityAction { action, target } => {
                let mut value = action.compile();
                value.as_object_mut().unwrap().insert("target".to_string(), Value::String(target.to_string()));
                value
            },
            Block::IfEntity { action, target } => {
                let mut value = action.compile();
                value.as_object_mut().unwrap().insert("target".to_string(), Value::String(target.to_string()));
                value
            },
            Block::GameAction { action } => action.compile(),
            Block::IfGame { action } => action.compile(),
            Block::SetVariable { action } => action.compile(),
            Block::IfVariable { action } => action.compile(),
            Block::Else => Value::Null,
            Block::Control { action } => action.compile(),
            Block::Select { action} => action.compile(),
            Block::Repeat { action } => action.compile(),
            Block::Function { data, args } => {
                todo!("Add block tags")
            },
            Block::CallFunction { data, args } => Value::Null,
            Block::Process { data, args } => {
                todo!("Add block tags")
            },
            Block::CallProcess { data, args } => {
                todo!("Add block tags")
            },
            Block::Bracket { direct, typ } => json!(
                {
                    "id": "bracket",
                    "direct": match direct {
                        BracketDirection::Open => "open",
                        BracketDirection::Close => "close",
                    },
                    "type": match typ {
                        BracketType::Norm => "norm",
                        BracketType::Reapeat => "repeat",
                    },
                }
            ),
        }.to_string()
    }
}

/// A bracket block direction
pub enum BracketDirection {
    Open,
    Close,
}

/// A bracket block type
pub enum BracketType {
    Norm,
    Reapeat,
}