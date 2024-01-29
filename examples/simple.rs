use saphire_types::block::Block as B;
use saphire_types::block::prelude::*;
use saphire_types::template::Template;
use saphire_types::targets::PlayerTarget;
use saphire_types::types::*;

fn main() -> () {
    // A simple template that sends a message to all players when a player joins.
    let template = Template::new("simple_join".to_string(), vec![
        B::PlayerEvent {
            event: PlayerEvent::PlayerJoinGameEvent {}
        },
        B::PlayerAction {
            action: PlayerAction::SendMessage {
                message_to_send: vec![
                        MiniMessage{ value: "Hello World".to_string()}
                    ]},
                target: PlayerTarget::All
            },
    ]);

    println!("{}", template.compile());
}