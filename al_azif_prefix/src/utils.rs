use crate::prelude::*;

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

pub fn execute_action(action_tag: &str, user: &Id, target: &mut Id) -> Result<Vec<ResponseBlueprint>> {
    match action_tag {
        "attack" => attack::execute(user, target),
        _ => unreachable!("Invalid action tag: {}", action_tag),
    }
}