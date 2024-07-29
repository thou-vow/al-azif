use crate::_prelude::*;

pub fn execute_attack<'a>(action_tag: &str, attacker: &mut Id, target: &mut Id) -> Result<Blueprints<'a>> {
    use crate::commands::*;
    let blueprints = match action_tag {
        "attack" => attack::execute(attacker, target),
        "vital_trill" => vital_trill::execute(attacker, target),
        _ => return Err(PrefixError::InvalidActionTag { action_tag: FixedString::from_str_trunc(action_tag) }),
    };
    Ok(blueprints)
}
