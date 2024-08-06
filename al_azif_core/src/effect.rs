use crate::_prelude::*;

#[derive(Deserialize, Serialize)]
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

pub trait AsEffect: Clone + Into<Effect> {
    const EMOJI: &'static str;
    const NAME: &'static str;
    const NAME_PT: &'static str;
    fn acquire_effect_text(&self, bot: &impl AsBot, id_name: &str) -> Option<String>;
    fn lose_effect_text(&self, bot: &impl AsBot, id_name: &str) -> Option<String>;
    fn summary(&self, bot: &impl AsBot) -> Cow<'static, str>;
}

#[derive(Clone, Deserialize, Serialize)]
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

#[derive(Clone, Deserialize, Serialize)]
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

#[derive(Clone, Deserialize, Serialize)]
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

#[derive(Clone, Deserialize, Serialize)]
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

#[derive(Clone, Deserialize, Serialize)]
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

#[derive(Clone, Deserialize, Serialize)]
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
