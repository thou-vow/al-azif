use crate::_prelude::*;

pub const TAG: &str = "heal";
pub const TAG_PT: &str = "curar";

pub async fn run_prefix(bot: &impl AsBot, msg: &Message, args: VecDeque<&str>) -> Result<Responses> {
    let setting = Setting::new(bot, args)
        .fetch_battle(msg.channel_id.to_string())
        .await?
        .require_primary_moment()
        .await?
        .fetch_user()
        .await?
        .fetch_optional_targets::<1>()
        .await?;

    let mut blueprints = Vec::new();

    let mut battle = setting.get_battle_mirror().write().await;
    let mut user = setting.get_user_mirror().write().await;

    let heal_amount = user.constitution / 2;
    let new_effect = HealingOverTimeEffect { healing_over_turn: heal_amount / 5, turn_duration: 5 };

    match &setting.get_optional_target_tags_and_ms()[0] {
        Some((target_tag, target_m)) if *target_tag != user.tag => {
            let mut target = target_m.write().await;

            blueprints.push(ResponseBlueprint::new().set_content(lang_diff!(bot,
                en: f!("⚕️ | **{}** healed **{}** by **{}**.",
                    user.name, target.name, mark_thousands(heal_amount),
                ),
                pt: f!("⚕️ | **{}** curou **{}** em **{}**.",
                    user.name, target.name, mark_thousands(heal_amount)
                )
            )));

            blueprints.extend(target.restore_health(bot, heal_amount));
            blueprints.extend(target.acquire_effect(bot, HealingOverTimeEffect { healing_over_turn: heal_amount / 5, turn_duration: 5 }));
        },
        _ => {
            blueprints.push(ResponseBlueprint::new().set_content(lang_diff!(bot,
                en: f!("⚕️ | **{}** received **{}** of healing.",
                    user.name, mark_thousands(heal_amount),
                ),
                pt: f!("⚕️ | **{}** recebeu **{}** de cura.",
                    user.name, mark_thousands(heal_amount)
                )
            )));

            blueprints.extend(user.restore_health(bot, heal_amount));
            blueprints.extend(user.acquire_effect(bot, new_effect));
        },
    }

    user.unwrite();

    blueprints.extend(battle.advance(bot).await?);
    blueprints.push(battle.generate_turn_screen(bot).await?);

    Ok(vec![Response::send(blueprints)])
}
