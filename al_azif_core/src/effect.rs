#![allow(clippy::single_match, dead_code, unused_assignments, unused_variables, unused_mut)]

use crate::_prelude::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Effect {
    Bleed(BleedEffect),
    Block(BlockEffect),
    Faint(FaintEffect),
    HealingOverTime(HealingOverTimeEffect),
    Miracle(MiracleEffect),
    Rise(RiseEffect),
}
impl Effect {
    pub fn emoji(&self) -> &'static str {
        match self {
            Self::Bleed(_) => BleedEffect::EMOJI,
            Self::Block(_) => BlockEffect::EMOJI,
            Self::Faint(_) => FaintEffect::EMOJI,
            Self::HealingOverTime(_) => HealingOverTimeEffect::EMOJI,
            Self::Miracle(_) => MiracleEffect::EMOJI,
            Self::Rise(_) => RiseEffect::EMOJI,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::Bleed(_) => BleedEffect::NAME,
            Self::Block(_) => BlockEffect::NAME,
            Self::Faint(_) => FaintEffect::NAME,
            Self::HealingOverTime(_) => HealingOverTimeEffect::NAME,
            Self::Miracle(_) => MiracleEffect::NAME,
            Self::Rise(_) => RiseEffect::NAME,
        }
    }

    pub fn name_pt(&self) -> &'static str {
        match self {
            Self::Bleed(_) => BleedEffect::NAME_PT,
            Self::Block(_) => BlockEffect::NAME_PT,
            Self::Faint(_) => FaintEffect::NAME_PT,
            Self::HealingOverTime(_) => HealingOverTimeEffect::NAME_PT,
            Self::Miracle(_) => MiracleEffect::NAME_PT,
            Self::Rise(_) => RiseEffect::NAME_PT,
        }
    }

    pub fn acquire_effect_text(&self, bot: &impl AsBot, id_name: &str) -> Option<String> {
        match self {
            Self::Bleed(bleed) => bleed.acquire_effect_text(bot, id_name),
            Self::Block(block) => block.acquire_effect_text(bot, id_name),
            Self::Faint(faint) => faint.acquire_effect_text(bot, id_name),
            Self::HealingOverTime(healing) => healing.acquire_effect_text(bot, id_name),
            Self::Miracle(miracle) => miracle.acquire_effect_text(bot, id_name),
            Self::Rise(rise) => rise.acquire_effect_text(bot, id_name),
        }
    }

    pub fn lose_effect_text(&self, bot: &impl AsBot, id_name: &str) -> Option<String> {
        match self {
            Self::Bleed(bleed) => bleed.lose_effect_text(bot, id_name),
            Self::Block(block) => block.lose_effect_text(bot, id_name),
            Self::Faint(faint) => faint.lose_effect_text(bot, id_name),
            Self::HealingOverTime(healing) => healing.lose_effect_text(bot, id_name),
            Self::Miracle(miracle) => miracle.lose_effect_text(bot, id_name),
            Self::Rise(rise) => rise.lose_effect_text(bot, id_name),
        }
    }

    pub fn summary(&self, bot: &impl AsBot) -> Cow<'static, str> {
        match self {
            Self::Bleed(bleed) => bleed.summary(bot),
            Self::Block(block) => block.summary(bot),
            Self::Faint(faint) => faint.summary(bot),
            Self::HealingOverTime(healing) => healing.summary(bot),
            Self::Miracle(miracle) => miracle.summary(bot),
            Self::Rise(rise) => rise.summary(bot),
        }
    }
}
impl From<BleedEffect> for Effect {
    fn from(bleed: BleedEffect) -> Self { Self::Bleed(bleed) }
}
impl From<BlockEffect> for Effect {
    fn from(block: BlockEffect) -> Self { Self::Block(block) }
}
impl From<FaintEffect> for Effect {
    fn from(faint: FaintEffect) -> Self { Self::Faint(faint) }
}
impl From<HealingOverTimeEffect> for Effect {
    fn from(healing: HealingOverTimeEffect) -> Self { Self::HealingOverTime(healing) }
}
impl From<MiracleEffect> for Effect {
    fn from(miracle: MiracleEffect) -> Self { Self::Miracle(miracle) }
}
impl From<RiseEffect> for Effect {
    fn from(rise: RiseEffect) -> Self { Self::Rise(rise) }
}

pub trait AsEffect: Clone + Debug + Into<Effect> {
    const EMOJI: &'static str;
    const NAME: &'static str;
    const NAME_PT: &'static str;
    fn acquire_effect_text(&self, bot: &impl AsBot, id_name: &str) -> Option<String>;
    fn lose_effect_text(&self, bot: &impl AsBot, id_name: &str) -> Option<String>;
    fn summary(&self, bot: &impl AsBot) -> Cow<'static, str>;
}

pub mod all {
    pub use super::*;

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct BleedEffect {
        pub damage_over_turn: i64,
        pub turn_duration:    i64,
    }
    impl AsEffect for BleedEffect {
        const EMOJI: &'static str = "💔";
        const NAME: &'static str = "Bleed";
        const NAME_PT: &'static str = "Sangramento";

        fn acquire_effect_text(&self, bot: &impl AsBot, id_name: &str) -> Option<String> {
            Some(lang_diff!(bot,
                en: f!("{} | **{}** acquired the effect **{}**.",
                    Self::EMOJI,
                    id_name,
                    Self::NAME,
                ),
                pt: f!("{} | **{}** adquiriu o efeito **{}**.",
                    Self::EMOJI,
                    id_name,
                    Self::NAME_PT,
                )
            ))
        }

        fn lose_effect_text(&self, bot: &impl AsBot, id_name: &str) -> Option<String> {
            Some(lang_diff!(bot,
                en: f!("{} | **{}** recovered from **{}**.",
                    Self::EMOJI,
                    id_name,
                    Self::NAME,
                ),
                pt: f!("{} | **{}** recuperou-se de **{}**.",
                    Self::EMOJI,
                    id_name,
                    Self::NAME_PT,
                )
            ))
        }

        fn summary(&self, bot: &impl AsBot) -> Cow<'static, str> {
            lang_diff!(bot,
                en: f!("Receive **{}** damage after each action for **{}** turn{}.",
                    mark_thousands(self.damage_over_turn),
                    mark_thousands(self.turn_duration),
                    if self.turn_duration > 1 || self.turn_duration < -1 { "s" } else { "" },
                ),
                pt: f!("Recebe **{}** de dano após cada ação durante **{}** turno{}.",
                    mark_thousands(self.damage_over_turn),
                    mark_thousands(self.turn_duration),
                    if self.turn_duration > 1 || self.turn_duration < -1 { "s" } else { "" },
                )
            )
            .into()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct BlockEffect;
    impl AsEffect for BlockEffect {
        const EMOJI: &'static str = "🛡";
        const NAME: &'static str = "Block";
        const NAME_PT: &'static str = "Bloqueio";

        fn acquire_effect_text(&self, bot: &impl AsBot, id_name: &str) -> Option<String> {
            Some(lang_diff!(bot,
                en: f!("{} | **{}** is now defending.",
                    Self::EMOJI,
                    id_name,
                ),
                pt: f!("{} | **{}** está agora se defendendo.",
                    Self::EMOJI,
                    id_name,
                )
            ))
        }

        fn lose_effect_text(&self, _bot: &impl AsBot, _id_name: &str) -> Option<String> { None }

        fn summary(&self, bot: &impl AsBot) -> Cow<'static, str> {
            lang_diff!(bot,
                en: "Reduces the damage received from a Primary Action by **1/2** and then expires.",
                pt: "Reduz o dano recebido de uma Ação Primária em **1/2** e depois expira."
            )
            .into()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct FaintEffect;
    impl AsEffect for FaintEffect {
        const EMOJI: &'static str = "🪦";
        const NAME: &'static str = "Faint";
        const NAME_PT: &'static str = "Desmaio";

        fn acquire_effect_text(&self, bot: &impl AsBot, id_name: &str) -> Option<String> {
            Some(lang_diff!(bot,
                en: f!("{} | **{}** fainted.",
                    Self::EMOJI,
                    id_name,
                ),
                pt: f!("{} | **{}** desmaiou.",
                    Self::EMOJI,
                    id_name,
                )
            ))
        }

        fn lose_effect_text(&self, _bot: &impl AsBot, _id_name: &str) -> Option<String> { None }

        fn summary(&self, bot: &impl AsBot) -> Cow<'static, str> {
            lang_diff!(bot,
                en: "Unable to perform any action.",
                pt: "Incapaz de realizar nenhuma ação."
            )
            .into()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct HealingOverTimeEffect {
        pub healing_over_turn: i64,
        pub turn_duration:     i64,
    }
    impl AsEffect for HealingOverTimeEffect {
        const EMOJI: &'static str = "💊";
        const NAME: &'static str = "Healing Over Time";
        const NAME_PT: &'static str = "Cura Periódica";

        fn acquire_effect_text(&self, bot: &impl AsBot, id_name: &str) -> Option<String> {
            Some(lang_diff!(bot,
                en: f!("{} | **{}** acquired the effect **{}**.",
                    Self::EMOJI,
                    id_name,
                    Self::NAME,
                ),
                pt: f!("{} | **{}** adquiriu o efeito **{}**.",
                    Self::EMOJI,
                    id_name,
                    Self::NAME_PT,
                )
            ))
        }

        fn lose_effect_text(&self, bot: &impl AsBot, id_name: &str) -> Option<String> {
            Some(lang_diff!(bot,
                en: f!("{} | **{}** lost the effect **{}**.",
                    Self::EMOJI,
                    id_name,
                    Self::NAME,
                ),
                pt: f!("{} | **{}** perdeu o efeito **{}**.",
                    Self::EMOJI,
                    id_name,
                    Self::NAME_PT,
                )
            ))
        }

        fn summary(&self, bot: &impl AsBot) -> Cow<'static, str> {
            lang_diff!(bot,
                en: f!("Receive **{}** of healing after each turn for **{}** turn{}.",
                    mark_thousands(self.healing_over_turn),
                    mark_thousands(self.turn_duration),
                    if self.turn_duration > 1 || self.turn_duration < -1 { "s" } else { "" },
                ),
                pt: f!("Recebe **{}** de cura após cada turno durante **{}** turno{}.",
                    mark_thousands(self.healing_over_turn),
                    mark_thousands(self.turn_duration),
                    if self.turn_duration > 1 || self.turn_duration < -1 { "s" } else { "" },
                )
            )
            .into()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct MiracleEffect;
    impl AsEffect for MiracleEffect {
        const EMOJI: &'static str = "🌟";
        const NAME: &'static str = "Miracle";
        const NAME_PT: &'static str = "Milagre";

        fn acquire_effect_text(&self, bot: &impl AsBot, id_name: &str) -> Option<String> {
            Some(lang_diff!(bot,
                en: f!("{} | **{}** acquired the effect **{}**.",
                    Self::EMOJI,
                    id_name,
                    Self::NAME
                ),
                pt: f!("{} | **{}** adquiriu o efeito **{}**.",
                    Self::EMOJI,
                    id_name,
                    Self::NAME_PT
                )
            ))
        }

        fn lose_effect_text(&self, bot: &impl AsBot, id_name: &str) -> Option<String> {
            Some(lang_diff!(bot,
                en: f!("{} | **{}** lost the effect **Miracle**.",
                    Self::EMOJI,
                    id_name,
                ),
                pt: f!("{} | **{}** perdeu o efeito **Milagre**.",
                    Self::EMOJI,
                    id_name,
                )
            ))
        }

        fn summary(&self, bot: &impl AsBot) -> Cow<'static, str> {
            lang_diff!(bot,
                en: "Immediately receive **1** of healing when hit by decisive damage and then expires.",
                pt: "Recebe imediatamente **1** de cura quando atingido por um dano decisivo e depois expira."
            )
            .into()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct RiseEffect {
        pub might_bonus:   i64,
        pub turn_duration: i64,
    }
    impl AsEffect for RiseEffect {
        const EMOJI: &'static str = "⬆️";
        const NAME: &'static str = "Rise";
        const NAME_PT: &'static str = "Elevar";

        fn acquire_effect_text(&self, bot: &impl AsBot, id_name: &str) -> Option<String> {
            Some(lang_diff!(bot,
                en: f!("{} | **{}** acquired the effect **{}**.",
                    Self::EMOJI,
                    id_name,
                    Self::NAME
                ),
                pt: f!("{} | **{}** adquiriu o efeito **{}**.",
                    Self::EMOJI,
                    id_name,
                    Self::NAME_PT
                )
            ))
        }

        fn lose_effect_text(&self, bot: &impl AsBot, id_name: &str) -> Option<String> {
            Some(lang_diff!(bot,
                en: f!("{} | **{}** lost the effect **{}**.",
                    Self::EMOJI,
                    id_name,
                    Self::NAME
                ),
                pt: f!("{} | **{}** perdeu o efeito **{}**.",
                    Self::EMOJI,
                    id_name,
                    Self::NAME_PT
                )
            ))
        }

        fn summary(&self, bot: &impl AsBot) -> Cow<'static, str> {
            lang_diff!(bot,
                en: f!("Increases {MGT_LONG} by **{}** for **{}** turn{}",
                    mark_thousands(self.might_bonus),
                    mark_thousands(self.turn_duration),
                    if self.turn_duration > 1 || self.turn_duration < -1 { "s" } else { "" },
                ),
                pt: f!("Aumenta {MGT_LONG_PT} em **{}** por **{}** turno{}",
                mark_thousands(self.might_bonus),
                mark_thousands(self.turn_duration),
                    if self.turn_duration > 1 || self.turn_duration < -1 { "s" } else { "" },
                )
            )
            .into()
        }
    }
}

pub fn evaluate_might_bonuses(id: &Id) -> i64 {
    let mut bonuses = 0;

    for effect in id.effects.iter() {
        match effect {
            Effect::Rise(rise) => {
                bonuses += rise.might_bonus;
            },
            _ => (),
        }
    }

    bonuses
}

pub fn on_action_end(bot: &impl AsBot, id: &mut Id) -> Blueprints {
    let mut blueprints = Vec::new();

    let mut acting_effects = Vec::new();

    let mut i = 0;
    while i < id.effects.len() {
        let mut remove = false;

        match &mut id.effects[i] {
            Effect::Bleed(bleed) => {
                acting_effects.push(Effect::Bleed(bleed.clone()));
            },
            _ => (),
        }

        if remove {
            id.effects.remove(i);
        } else {
            i += 1;
        }
    }

    for effect in acting_effects {
        match effect {
            Effect::Bleed(bleed) => {
                blueprints.push(ResponseBlueprint::new().set_content(lang_diff!(bot,
                    en: f!("{} | **{}** received **{}** damage due to the **{}** effect.",
                        BleedEffect::EMOJI,
                        id.name,
                        mark_thousands(bleed.damage_over_turn),
                        BleedEffect::NAME,
                    ),
                    pt: f!("{} | **{}** recebeu **{}** de dano devido ao efeito **{}**.",
                        BleedEffect::EMOJI,
                        id.name,
                        mark_thousands(bleed.damage_over_turn),
                        BleedEffect::NAME_PT,
                    )
                )));
                blueprints.extend(id.receive_damage(bot, bleed.damage_over_turn));
            },
            _ => (),
        }
    }

    blueprints
}

pub fn on_faint(bot: &impl AsBot, id: &mut Id) -> Blueprints {
    let mut blueprints = Vec::new();

    let mut acting_effects = Vec::new();

    let mut i = 0;
    while i < id.effects.len() {
        let mut remove = false;

        match &mut id.effects[i] {
            Effect::Miracle(miracle) => {
                acting_effects.push(Effect::Miracle(miracle.clone()));
                remove = true;
            },
            _ => (),
        }

        if remove {
            id.effects.remove(i);
        } else {
            i += 1;
        }
    }

    for effect in acting_effects {
        match effect {
            Effect::Miracle(miracle) => {
                blueprints.push(ResponseBlueprint::new().set_content(lang_diff!(bot,
                    en: f!("{} | **{}** endured and received **1** of healing due to the **{}** effect.",
                        MiracleEffect::EMOJI,
                        id.name,
                        MiracleEffect::NAME,
                    ),
                    pt: f!("{} | **{}** suportou e recebeu **1** de cura devido ao efeito **{}**.",
                        MiracleEffect::EMOJI,
                        id.name,
                        MiracleEffect::NAME_PT,
                    )
                )));
                blueprints.extend(id.restore_health(bot, 1));
                blueprints.push(ResponseBlueprint::new().set_content(miracle.lose_effect_text(bot, &id.name).unwrap()));
            },
            _ => (),
        }
    }

    // TODO: Lose effects if still down

    blueprints
}

pub fn on_turn_end(bot: &impl AsBot, id: &mut Id) -> Blueprints {
    let mut blueprints = Vec::new();

    let mut acting_effects = Vec::new();

    let mut i = 0;
    while i < id.effects.len() {
        let mut remove = false;

        match &mut id.effects[i] {
            Effect::Bleed(bleed) => {
                acting_effects.push(Effect::Bleed(bleed.clone()));

                if bleed.turn_duration <= 0 {
                    remove = true;
                } else {
                    bleed.turn_duration -= 1;
                }
            },
            Effect::HealingOverTime(healing_over_time) => {
                acting_effects.push(Effect::HealingOverTime(healing_over_time.clone()));

                if healing_over_time.turn_duration <= 0 {
                    remove = true;
                } else {
                    healing_over_time.turn_duration -= 1;
                }
            },
            Effect::Rise(rise) => {
                acting_effects.push(Effect::Rise(rise.clone()));

                if rise.turn_duration <= 0 {
                    remove = true;
                } else {
                    rise.turn_duration -= 1;
                }
            },
            _ => (),
        }

        if remove {
            id.effects.remove(i);
        } else {
            i += 1;
        }
    }

    for effect in acting_effects {
        match effect {
            Effect::Bleed(bleed) => {
                if bleed.turn_duration <= 0 {
                    blueprints.push(ResponseBlueprint::new().set_content(bleed.lose_effect_text(bot, &id.name).unwrap()));
                }
            },
            Effect::HealingOverTime(healing) => {
                blueprints.push(ResponseBlueprint::new().set_content(lang_diff!(bot,
                    en: f!("{} | **{}** received **{}** of healing due to the **{}** effect.",
                        HealingOverTimeEffect::EMOJI,
                        id.name,
                        mark_thousands(healing.healing_over_turn),
                        HealingOverTimeEffect::NAME,
                    ),
                    pt: f!("{} | **{}** recebeu **{}** de cura devido ao efeito **{}**.",
                        HealingOverTimeEffect::EMOJI,
                        id.name,
                        mark_thousands(healing.healing_over_turn),
                        HealingOverTimeEffect::NAME_PT,
                    )
                )));
                blueprints.extend(id.restore_health(bot, healing.healing_over_turn));

                if healing.turn_duration <= 0 {
                    blueprints.push(ResponseBlueprint::new().set_content(healing.lose_effect_text(bot, &id.name).unwrap()));
                }
            },
            Effect::Rise(rise) => {
                if rise.turn_duration <= 0 {
                    blueprints.push(ResponseBlueprint::new().set_content(rise.lose_effect_text(bot, &id.name).unwrap()));
                }
            },
            _ => (),
        }
    }

    blueprints
}

pub fn at_primary_action_attack(bot: &impl AsBot, emitter: &mut Id, target: &mut Id, mut damage: i64) -> (i64, Blueprints) {
    let mut blueprints = Vec::new();

    let mut acting_effects = Vec::new();

    let mut i = 0;

    acting_effects = Vec::new();

    i = 0;
    while i < target.effects.len() {
        let mut remove = false;

        match &mut target.effects[i] {
            Effect::Block(block) => {
                acting_effects.push(Effect::Block(block.clone()));
                remove = true;
            },
            _ => (),
        }

        if remove {
            target.effects.remove(i);
        } else {
            i += 1;
        }
    }

    for effect in acting_effects {
        match effect {
            Effect::Block(block) => {
                damage /= 2;
            },
            _ => (),
        }
    }

    (damage, blueprints)
}
