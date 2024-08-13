use crate::_prelude::*;

pub const TAG: &str = "wave_attack";
pub const TAG_PT: &str = "ataque_onda";

pub async fn run_prefix(bot: &impl AsBot, msg: &Message, args: VecDeque<&str>) -> Result<Responses> {
    let setting = Setting::new(bot, args)
        .fetch_battle(msg.channel_id.to_string())
        .await?
        .require_primary_moment()
        .await?
        .fetch_user()
        .await?
        .fetch_targets([lang_diff!(bot,
            en: "You need to specify at least the main target.",
            pt: "Você precisa especificar pelo menos o alvo principal."
        )])
        .await?
        .fetch_optional_targets::<2>()
        .await?
        .unallow_any_self_any_target(lang_diff!(bot,
            en: "You cannot attack yourself.",
            pt: "Você não pode atacar a si mesmo(a)."
        ))?
        .unallow_duplicate_any_target(lang_diff!(bot,
            en: "You cannot specify the same target twice.",
            pt: "Você não pode especificar o mesmo alvo duas vezes."
        ))?;

    let mut blueprints = Vec::new();

    let mut battle = setting.get_battle_mirror().write().await;
    let user = setting.get_user_mirror().read().await;
    let main_target = setting.get_target_ms()[0].read().await;

    let mut target_names = vec![main_target.name.clone()];

    let optional_targets = setting.get_optional_target_ms();
    if let Some(optional_target_m) = optional_targets[0] {
        let optional_target = optional_target_m.read().await;
        target_names.push(optional_target.name.clone())
    }
    if let Some(optional_target_m) = optional_targets [0] {
        let optional_target = optional_target_m.read().await;
        target_names.push(optional_target.name.clone())
    }

    
    
    Ok(vec![Response::send(blueprints)])
}
