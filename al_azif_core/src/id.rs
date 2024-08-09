use crate::_prelude::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct Id {
    pub tag:                  FixedString<u8>,
    pub name:                 FixedString<u8>,
    pub gender:               Gender,
    pub age:                  Age,
    pub lvl:                  i64,
    pub xp:                   i64,
    pub hp:                   i64,
    pub sp:                   i64,
    pub points_to_distribute: i64,
    pub constitution:         i64,
    pub spirit:               i64,
    pub might:                i64,
    pub movement:             i64,
    pub dexterity:            i64,
    pub cognition:            i64,
    pub charisma:             i64,
    pub effects:              Vec<Effect>,
    pub color:                Option<u32>,
    pub emoji:                Option<FixedString<u8>>,
    pub current_battle_tag:   Option<FixedString<u8>>,
}
impl Id {
    pub fn new(tag: impl AsRef<str>) -> Self { Id::_new(tag.as_ref()) }

    fn _new(tag: &str) -> Self {
        Self {
            tag:                  FixedString::from_str_trunc(tag),
            name:                 FixedString::from_static_trunc("Unknown"),
            gender:               Gender::Other,
            age:                  Age::Child,
            lvl:                  0,
            xp:                   0,
            hp:                   50,
            sp:                   50,
            points_to_distribute: 200,
            constitution:         5,
            spirit:               5,
            might:                5,
            movement:             5,
            dexterity:            5,
            cognition:            5,
            charisma:             5,
            effects:              Vec::new(),
            color:                None,
            emoji:                None,
            current_battle_tag:   None,
        }
    }

    pub fn assign_name(mut self, tag: impl AsRef<str>) -> Self {
        self.name = FixedString::from_str_trunc(tag.as_ref());
        self
    }
}
impl Id {
    pub async fn join_battle(&mut self, battle: &mut Battle) {
        let opponent = Opponent {
            tag:                                    self.tag.clone(),
            turn_value:                             self.movement,
            last_total_increased_turn_value_amount: self.movement,
        };

        battle.opponents.push(opponent);

        self.current_battle_tag = Some(battle.tag.clone());
    }

    pub async fn start_turn(&mut self, bot: &impl AsBot, _battle: &mut Battle) -> Result<Blueprints> {
        let mut blueprints = Vec::new();

        blueprints.push(ResponseBlueprint::new().set_content(lang_diff!(bot,
            en: f!("🕒 | It's now **{}**'s turn.", self.name),
            pt: f!("🕒 | Agora é a vez de **{}**.", self.name),
        )));

        Ok(blueprints)
    }

    pub async fn end_turn(&mut self, bot: &impl AsBot, battle: &mut Battle) -> Result<Blueprints> {
        let mut blueprints = Vec::new();

        battle
            .opponents
            .iter_mut()
            .find(|opponent| opponent.tag == self.tag)
            .unwrap()
            .sub_turn_value(battle.turn_value_cap);

        blueprints.push(ResponseBlueprint::new().set_content(lang_diff!(bot,
            en: f!("🏁 | End of **{}**'s turn.", self.name),
            pt: f!("🏁 | Fim do turno de **{}**.", self.name),
        )));
        blueprints.extend(effect::on_turn_end(bot, self));

        Ok(blueprints)
    }

    pub fn acquire_effect(&mut self, bot: &impl AsBot, new_effect: impl AsEffect) -> Blueprints {
        let mut blueprints = Vec::new();

        if let Some(acquire_effect_text) = new_effect.acquire_effect_text(bot, &self.name) {
            blueprints.push(ResponseBlueprint::new().set_content(acquire_effect_text));
        }

        self.effects.push(new_effect.into());

        blueprints
    }

    pub fn receive_damage(&mut self, bot: &impl AsBot, damage: i64) -> Blueprints {
        let mut blueprints = Vec::new();

        // TODO: for shield absorption

        blueprints.extend(self.lose_health(bot, damage));

        blueprints
    }

    pub fn lose_health(&mut self, bot: &impl AsBot, amount: i64) -> Blueprints {
        let mut blueprints = Vec::new();

        let previous_hp = self.hp;
        self.hp = (self.hp - amount).clamp(0, self.constitution * 10);

        if previous_hp != self.hp {
            blueprints.push(ResponseBlueprint::new().set_content(f!(
                "```ansi\n\u{001b}[0;40m{}: \u{001b}[0;40;{}m{}\u{001b}[0;40m → \u{001b}[0;40;{}m{}```",
                lang_diff!(bot, en: HP_SHORT, pt: HP_SHORT_PT),
                match previous_hp * 100 / (self.constitution * 10) {
                    .. 33 => 31,
                    33 .. 66 => 33,
                    _ => 32,
                },
                mark_thousands(previous_hp),
                match self.hp * 100 / (self.constitution * 10) {
                    .. 33 => 31,
                    33 .. 66 => 33,
                    _ => 32,
                },
                mark_thousands(self.hp),
            )));
        } else {
            blueprints.push(ResponseBlueprint::new().set_content(f!(
                "```ansi\n\u{001b}[0;40m{}: \u{001b}[0;40;{}m{}```",
                lang_diff!(bot, en: HP_SHORT, pt: HP_SHORT_PT),
                match self.hp * 100 / (self.constitution * 10) {
                    .. 33 => 31,
                    33 .. 66 => 33,
                    _ => 32,
                },
                mark_thousands(self.hp),
            )));
        }

        if self.hp == 0 {
            blueprints.extend(self.faint(bot));
        }

        // Todo: Effects after losing HP

        blueprints
    }

    pub fn restore_health(&mut self, bot: &impl AsBot, amount: i64) -> Blueprints {
        let mut blueprints = Vec::new();

        let previous_hp = self.hp;
        self.hp = (self.hp + amount).clamp(0, self.constitution * 10);

        let mut new_content = f!(
            "```ansi\n\u{001b}[0;40m{}: \u{001b}[0;40;{}m{}",
            lang_diff!(bot, en: HP_SHORT, pt: HP_SHORT_PT),
            match previous_hp * 100 / (self.constitution * 10) {
                .. 33 => 31,
                33 .. 66 => 33,
                _ => 32,
            },
            mark_thousands(previous_hp)
        );
        if previous_hp != self.hp {
            new_content.push_str(&f!(
                "\u{001b}[0;40m → \u{001b}[0;40;{}m{}",
                match self.hp * 100 / (self.constitution * 10) {
                    .. 33 => 31,
                    33 .. 66 => 33,
                    _ => 32,
                },
                mark_thousands(self.hp),
            ));
        }
        new_content.push_str("```");
        blueprints.push(ResponseBlueprint::new().set_content(new_content));

        // Todo: Effects after restoring HP

        blueprints
    }

    pub fn faint(&mut self, bot: &impl AsBot) -> Blueprints {
        let mut blueprints = self.acquire_effect(bot, FaintEffect);

        blueprints.extend(effect::on_faint(bot, self));

        // Lose Faint effect if alive somehow after effects
        if self.hp > 0 {
            for i in 0 .. self.effects.len() {
                if let Effect::Faint(FaintEffect) = self.effects[i] {
                    self.effects.remove(i);
                    break;
                }
            }
        }

        blueprints
    }
}
impl Id {
    pub fn evaluate_total_might(&self) -> i64 { self.might + effect::evaluate_might_bonuses(self) }
}
impl Reflective for Id {
    const FOLDER_PATH: &'static str = "./database/ids";

    fn get_tag(&self) -> &str { self.tag.as_ref() }
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Gender {
    Other,
    Female,
    Male,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Age {
    Child,
    Teen,
    Adult,
    Senior,
}
