use crate::_prelude::*;

pub fn get_accuracy_bonus_of_attack(action_tag: &str) -> i64 {
    match action_tag {
        "attack" => attack::ACCURACY_BONUS,
        _ => unreachable!("Invalid action tag: {}", action_tag),
    }
}

pub fn get_evasion_bonus_of_dodge(action_tag: &str) -> i64 {
    match action_tag {
        "dodge" => dodge::EVASION_BONUS,
        _ => unreachable!("Invalid action tag: {}", action_tag),
    }
}

pub fn execute_attack<'a>(action_tag: &str, attacker: &mut Id, target: &mut Id) -> Blueprints<'a> {
    match action_tag {
        "attack" => attack::execute(attacker, target),
        _ => unreachable!("Invalid action tag: {}", action_tag),
    }
}
