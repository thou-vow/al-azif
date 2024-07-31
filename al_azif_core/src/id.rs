use crate::{_prelude::*, effect};

#[derive(Deserialize, Serialize)]
pub struct Id {
    pub tag:                  FixedString,
    pub name:                 FixedString,
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
    pub effects:              LinkedList<Effect>,
    pub color:                Option<u32>,
    pub emoji:                Option<FixedString>,
    pub current_battle_tag:   Option<FixedString>,
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
            effects:              LinkedList::new(),
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

        battle.opponents.insert(self.tag.clone(), opponent);

        self.current_battle_tag = Some(battle.tag.clone());
    }

    pub async fn start_turn<'a>(&mut self, _battle: &mut Battle) -> Result<Blueprints<'a>> {
        let mut blueprints = Vec::new();

        blueprints.push(ResponseBlueprint::new().set_content(f!("🕒 | Agora é a vez de **{}**.", self.name)));

        Ok(blueprints)
    }

    pub async fn end_turn<'a>(&mut self, bot: &impl AsBot, battle: &mut Battle) -> Result<Blueprints<'a>> {
        let mut blueprints = Vec::new();

        battle.opponents.get_mut(&self.tag).unwrap().sub_turn_value(battle.turn_value_cap);

        blueprints.push(ResponseBlueprint::new().set_content(f!("🏁 | Fim do turno de **{}**.", self.name)));
        blueprints.extend(self.effects_on_turn_end(bot, battle));

        Ok(blueprints)
    }

    pub fn acquire_effect<'a>(&mut self, bot: &impl AsBot, new_effect: impl AsEffect) -> Blueprints<'a> {
        let mut blueprints = Vec::new();

        if let Some(acquire_effect_text) = new_effect.acquire_effect_text(bot, &self.name) {
            blueprints.push(ResponseBlueprint::new().set_content(acquire_effect_text));
        }

        self.effects.push_back(new_effect.into());

        blueprints
    }

    pub fn receive_damage<'a>(&mut self, bot: &impl AsBot, damage: i64) -> Blueprints<'a> {
        let mut blueprints = Vec::new();
        let mut cursor = self.effects.cursor_front_mut();

        // TODO: for shield absorption

        blueprints.extend(self.lose_health(bot, damage));

        blueprints
    }

    pub fn lose_health<'a>(&mut self, bot: &impl AsBot, amount: i64) -> Blueprints<'a> {
        let mut blueprints = Vec::new();

        let previous_hp = self.hp;
        self.hp = (self.hp - amount).clamp(0, self.hp);

        blueprints.push(ResponseBlueprint::new().set_content(lang_diff!(bot,
            en: f!("{CON_EMOJI} | {HP_SHORT}: {previous_hp} → {}", self.hp),
            pt: f!("{CON_EMOJI} | {HP_SHORT_PT}: {previous_hp} → {}", self.hp)
        )));

        if self.hp == 0 {
            blueprints.extend(self.faint(bot));
        }

        // Todo: Effects after losing HP

        blueprints
    }

    pub fn restore_health<'a>(&mut self, bot: &impl AsBot, amount: i64) -> Blueprints<'a> {
        let mut blueprints = Vec::new();

        let previous_hp = self.hp;
        self.hp = (self.hp + amount).clamp(0, self.hp);

        blueprints.push(ResponseBlueprint::new().set_content(lang_diff!(bot,
            en: f!("{CON_EMOJI} | {HP_SHORT}: {previous_hp} → {}", self.hp),
            pt: f!("{CON_EMOJI} | {HP_SHORT_PT}: {previous_hp} → {}", self.hp)
        )));

        // Todo: Effects after restoring HP

        blueprints
    }

    pub fn faint<'a>(&mut self, bot: &impl AsBot) -> Blueprints<'a> {
        let mut blueprints = self.acquire_effect(bot, FaintEffect);

        blueprints.extend(self.effects_on_fainting(bot));

        // Lose Faint effect if alive somehow after effects
        if self.hp > 0 {
            let mut cursor = self.effects.cursor_back_mut();
            while let Some(effect) = cursor.current() {
                match effect {
                    Effect::Faint(_) => {
                        cursor.remove_current();
                        break;
                    },
                    _ => cursor.move_next(),
                }
            }
        }

        blueprints
    }
}
impl Id {
    pub fn evaluate_might_bonuses(&self) -> i64 {
        let mut bonuses = 0;

        for effect in self.effects.iter() {
            match effect {
                Effect::Rise(rise) => {
                    bonuses += rise.might_bonus;
                },
                _ => (),
            }
        }

        bonuses
    }

    pub fn effects_on_turn_end<'a>(&mut self, bot: &impl AsBot, _battle: &mut Battle) -> Blueprints<'a> {
        let mut blueprints = Vec::new();
        let mut cursor = self.effects.cursor_front_mut();

        let mut acting_effects = Vec::new();

        while let Some(effect) = cursor.current() {
            let mut remove_effect = false;

            match effect {
                Effect::Bleed(bleed) => {
                    acting_effects.push(Effect::Bleed(bleed.clone()));

                    if bleed.turn_duration <= 0 {
                        remove_effect = true;
                    } else {
                        bleed.turn_duration -= 1;
                    }
                },
                Effect::Rise(rise) => {
                    acting_effects.push(Effect::Rise(rise.clone()));

                    if rise.turn_duration <= 0 {
                        remove_effect = true;
                    } else {
                        rise.turn_duration -= 1;
                    }
                },
                _ => (),
            }

            if remove_effect {
                cursor.remove_current();
            } else {
                cursor.move_next();
            }
        }

        for effect in acting_effects {
            match effect {
                Effect::Bleed(bleed) => {
                    blueprints.push(ResponseBlueprint::new().set_content(lang_diff!(bot,
                        en: f!("{} | **{}** received **{}** damage due to the **{}** effect.",
                            BleedEffect::EMOJI,
                            self.name,
                            mark_thousands(bleed.damage_over_turn),
                            BleedEffect::NAME,
                        ),
                        pt: f!("{} | **{}** recebeu **{}** de dano devido ao efeito **{}**.",
                            BleedEffect::EMOJI,
                            self.name,
                            mark_thousands(bleed.damage_over_turn),
                            BleedEffect::NAME_PT,
                        )
                    )));
                    blueprints.extend(self.receive_damage(bot, bleed.damage_over_turn));

                    if bleed.turn_duration <= 0 {
                        blueprints.push(ResponseBlueprint::new().set_content(bleed.lose_effect_text(bot, &self.name).unwrap()));
                    }
                },
                Effect::Rise(rise) => {
                    if rise.turn_duration <= 0 {
                        blueprints.push(ResponseBlueprint::new().set_content(rise.lose_effect_text(bot, &self.name).unwrap()));
                    }
                },
                _ => (),
            }
        }

        blueprints
    }

    pub fn effects_when_attacking_with_primary_action<'a>(
        &mut self,
        bot: &impl AsBot,
        mut damage: i64,
        target: &mut Id,
    ) -> (i64, Blueprints<'a>) {
        let mut _blueprints = Vec::new();
        let mut _cursor = self.effects.cursor_front_mut();

        let mut cursor = target.effects.cursor_front_mut();
        let mut acting_effects = Vec::new();

        while let Some(effect) = cursor.current() {
            let mut remove_effect = false;

            match effect {
                Effect::Block(block) => {
                    acting_effects.push(Effect::Block(block.clone()));
                    remove_effect = true;
                },
                _ => (),
            }

            if remove_effect {
                cursor.remove_current();
            } else {
                cursor.move_next();
            }
        }

        for effect in acting_effects {
            match effect {
                Effect::Block(_) => {
                    damage /= 2;
                },
                _ => (),
            }
        }

        (damage, _blueprints)
    }

    pub fn effects_on_fainting<'a>(&mut self, bot: &impl AsBot) -> Blueprints<'a> {
        let mut blueprints = Vec::new();
        let mut cursor = self.effects.cursor_front_mut();

        let mut acting_effects = Vec::new();

        while let Some(effect) = cursor.current() {
            let mut remove_effect = false;

            match effect {
                Effect::Miracle(miracle) => {
                    acting_effects.push(miracle.clone());

                    remove_effect = true;
                },
                _ => (),
            }

            if remove_effect {
                cursor.remove_current();
            } else {
                cursor.move_next();
            }
        }

        for effect in acting_effects {
            blueprints.push(ResponseBlueprint::new().set_content(lang_diff!(bot,
                en: f!("{} | **{}** restored **1** {HP_SHORT} due to the **{}** effect.",
                    MiracleEffect::EMOJI,
                    self.name,
                    MiracleEffect::NAME,
                ),
                pt: f!("{} | **{}** restaurou **1** {HP_SHORT_PT} devido ao efeito **{}**.",
                    MiracleEffect::EMOJI,
                    self.name,
                    MiracleEffect::NAME_PT,
                )
            )));
            blueprints.extend(self.restore_health(bot, 1));
            blueprints.push(ResponseBlueprint::new().set_content(effect.lose_effect_text(bot, &self.name).unwrap()));
        }

        // Todo: Loose effects if still down

        blueprints
    }
}
impl Reflective for Id {
    const FOLDER_PATH: &'static str = "./database/ids";

    fn get_tag(&self) -> &str { self.tag.as_ref() }
}

#[derive(Deserialize, Serialize)]
pub enum Gender {
    Other,
    Female,
    Male,
}

#[derive(Deserialize, Serialize)]
pub enum Age {
    Child,
    Teen,
    Adult,
    Senior,
}
