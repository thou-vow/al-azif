use crate::_prelude::*;

pub macro battle_exists_in_channel {
    ($bot:expr, $channel_id:expr) => {
        match Mirror::<Battle>::get($bot, $channel_id.to_string()).await {
            Ok(battle_m) => battle_m,
            Err(_) => {
                return Ok(response::simple_send_and_delete_with_original(
                    lang_diff!($bot, en: "There is no battle in this channel.",
                                     pt: "Não há batalha neste canal."),
                ));
            }
        }
    },
}

pub macro moment_none_in_battle {
    ($bot:expr, $battle:expr) => {
        match $battle.current_moment {
            Moment::None => (),
            _ => return Ok(response::simple_send_and_delete_with_original(
                lang_diff!($bot, en: "You can't use this command now.",
                                 pt: "Você não pode usar este comando agora."),
            )),
        }
    },
}

pub macro moment_primary_in_battle {
    ($bot:expr, $battle:expr) => {
        match &$battle.current_moment {
            Moment::PrimaryAction { primary_action_tag, attacker_tag, target_tag } => (primary_action_tag, attacker_tag, target_tag),
            _ => return Ok(response::simple_send_and_delete_with_original(
                lang_diff!($bot, en: "You can't use this command now.",
                                 pt: "Você não pode usar este comando agora."),
            )),
        }
    },
}

pub macro target_arg {
    ($bot:expr, $optional_arg:expr) => {
        match $optional_arg {
            Some(target_tag) => target_tag,
            None => {
                return Ok(response::simple_send_and_delete_with_original(
                    lang_diff!($bot, en: "The argument 'target' is required.",
                                     pt: "O argumento 'alvo' é obrigatório."),
                ));
            }
        }
    },
}

pub macro targeting_another {
    ($bot:expr, $battle:expr, $target_tag:expr) => {
        if $target_tag == $battle.current_turn_owner_tag {
            return Ok(response::simple_send_and_delete_with_original(
                lang_diff!($bot, en: "You can't target yourself.",
                                 pt: "O alvo não pode ser o próprio usuário."),
            ));
        }
    },
}

pub macro target_exists {
    ($bot:expr, $target_tag:expr) => {
        match Mirror::<Id>::get($bot, $target_tag).await {
            Ok(target_m) => target_m,
            Err(_) => {
                return Ok(response::simple_send_and_delete_with_original(
                    lang_diff!($bot, en: "The target doesn't exist.",
                                     pt: "O alvo não existe."),
                ));
            }
        }
    },
}

pub macro target_is_in_the_battle {
    ($bot:expr, $battle:expr, $target_tag:expr) => {
        if !$battle.opponents.contains_key(&FixedString::from_str_trunc($target_tag)) {
            return Ok(response::simple_send_and_delete_with_original(
                lang_diff!($bot, en: "The target is not in the battle.",
                                 pt: "O alvo não está na batalha."),
            ));
        }
    },
}
