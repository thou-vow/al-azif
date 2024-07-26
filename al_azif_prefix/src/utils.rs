use crate::{_prelude::*, commands::*};

pub fn execute_attack<'a>(action_tag: &str, attacker: &mut Id, target: &mut Id) -> Blueprints<'a> {
    match action_tag {
        "attack" => attack::execute(attacker, target),
        _ => unreachable!("Invalid action tag: {}", action_tag),
    }
}
