pub mod block_types;
pub mod prelude;

use serde_json::Value;

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
        sub_action: SelectSubAction,
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
        // match self {
        //     RawBlock::PlayerEvent { event } => event.compile(),
        //     RawBlock::PlayerAction { action, target } => action.compile(target),
        //     RawBlock::IfPlayer { action, target } => action.compile(target),
        //     RawBlock::EntityEvent { event } => event.compile(),
        //     RawBlock::EntityAction { action, target } => action.compile(target),
        //     RawBlock::IfEntity { action, target } => action.compile(target),
        //     RawBlock::GameAction { action } => action.compile(),
        //     RawBlock::IfGame { action } => action.compile(),
        //     RawBlock::SetVariable { action } => action.compile(),
        //     RawBlock::IfVariable { action } => action.compile(),
        //     RawBlock::Else => Value::Null,
        //     RawBlock::Control { action } => action.compile(),
        //     RawBlock::Select { action, sub_action } => action.compile(sub_action),
        //     RawBlock::Function { data, args, bl_tags } => bl_tags.compile(data, args),
        //     RawBlock::CallFunction { data, args } => Value::Null,
        //     RawBlock::Process { data, args, bl_tags } => bl_tags.compile(data, args),
        //     RawBlock::CallProcess { data, args, bl_tags } => bl_tags.compile(data, args),
        //     RawBlock::Bracket { direct, typ } => Value::Null,
        // }
        todo!("Compile RawBlock")
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