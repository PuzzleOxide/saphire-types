# Saphire-types

This crate is part of the saphire project. For more information see Saphire-rs.

The goal of Saphire-types is to provide rust types for serializing and deserializing [Diamondfire](mcdiamondfire.net) templates. To get started simply create a new `Template` and add any number of code blocks, than `.compile()` it.

```rust
use saphire-types::{
    block::{Block as B,
        prelude::*
    }
    template::Template,
    targets::PlayerTarget,
    types::*
}

fn main() -> () {
    // A simple template that sends a message to all players when a player joins.
    let template = Template::new("simple_join".to_string(), vec![
        B::PlayerEvent {
            event: PlayerEvent::PlayerJoinGameEvent {}
        },
        B::PlayerAction {
            action: PlayerAction::SendMessage {
                message_to_send:vec![MiniMessage::Literal(MiniMessageLiteral::new("Hello World".to_string()))],
                alignment_mode_tag: Default::default(),
                inherit_styles_tag: Default::default(),
                text_value_merging_tag: Default::default(),
        },
            target: PlayerTarget::All
        },
    ]);

    println!("{}", template.compile());
}
```

## Contributing

Any and all help on this project is welcome, but if you want your contributions dealt with faster making sure that they are well formated and documented helps. It is also important to remember that large portions of this crate are auto-generated using [Saphire-typegen](https://github.com/PuzzleOxide/saphire-typegen), so changes to any auto-generated modules should be made there.