use crate::_prelude::*;

#[derive(Deserialize, Serialize)]
pub struct Id {
    pub tag: FixedString,
    pub name: FixedString,
    pub gender: Gender,
    pub age: Age,
    pub lvl: i64,
    pub xp: i64,
    pub hp: i64,
    pub sp: i64,
    pub points_to_distribute: i64,
    pub constitution: i64,
    pub spirit: i64,
    pub might: i64,
    pub movement: i64,
    pub dexterity: i64,
    pub cognition: i64,
    pub charisma: i64,
    pub effects: Vec<Effect>,
    pub color: Option<u32>,
    pub emoji: Option<FixedString>,
    pub current_battle_tag: Option<FixedString>,
}
impl Id {
    pub fn new(tag: impl AsRef<str>) -> Self {
        Id::_new(tag.as_ref())
    }
    fn _new(tag: &str) -> Self {
        Self {
            tag: FixedString::from_str_trunc(tag),
            name: FixedString::from_static_trunc("Unknown"),
            gender: Gender::Other,
            age: Age::Child,
            lvl: 0,
            xp: 0,
            hp: 50,
            sp: 50,
            points_to_distribute: 200,
            constitution: 5,
            spirit: 5,
            might: 5,
            movement: 5,
            dexterity: 5,
            cognition: 5,
            charisma: 5,
            effects: Vec::new(),
            color: None,
            emoji: None,
            current_battle_tag: None,
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
            tag: self.tag.clone(),
            turn_value: self.movement,
            last_total_increased_turn_value_amount: self.movement,
        };

        battle.opponents.insert(self.tag.clone(), opponent);

        self.current_battle_tag = Some(battle.tag.clone());
    }
    pub async fn start_turn<'a>(&mut self, _battle: &mut Battle) -> Result<Blueprints<'a>> {
        let mut blueprints = Vec::new();

        blueprints.push(
            ResponseBlueprint::default()
                .set_content(f!("🕒 | Agora é a vez de **{}**.", self.name)),
        );

        Ok(blueprints)
    }
    pub async fn end_turn<'a>(&mut self, battle: &mut Battle) -> Result<Blueprints<'a>> {
        let mut blueprints = Vec::new();

        battle
            .opponents
            .get_mut(&self.tag)
            .unwrap()
            .sub_turn_value(battle.turn_value_cap);

        blueprints.push(
            ResponseBlueprint::default().set_content(f!("🏁 | Fim do turno de **{}**.", self.name)),
        );
        blueprints.extend(self.effects_on_turn_end(battle));

        Ok(blueprints)
    }
    pub fn take_damage<'a>(&mut self, mut damage: i64) -> Blueprints<'a> {
        let mut blueprints = Vec::new();
        (damage, blueprints) = self.effects_on_take_damage(damage);

        let previous_hp = self.hp;
        self.hp = (self.hp - damage).clamp(0, self.hp);

        blueprints.push(ResponseBlueprint::default().set_content(f!(
            "{STRIKE_EMOJI} | **{}** recebeu **{}** de dano. [{HP_SHORT}: **{}** → **{}**]",
            self.name,
            mark_thousands(damage),
            mark_thousands(previous_hp),
            mark_thousands(self.hp),
        )));

        blueprints
    }
    pub fn acquire_effect<'a>(&mut self, new_effect: Effect) -> Blueprints<'a> {
        let mut blueprints = Vec::new();

        'already_acquired_or_not: {
            for effect in self.effects.iter_mut() {
                match (effect, &new_effect) {
                    (Effect::Block, Effect::Block) => break 'already_acquired_or_not,
                    (
                        Effect::Rise {
                            might_bonus,
                            turn_duration,
                        },
                        Effect::Rise {
                            might_bonus: new_might_bonus,
                            turn_duration: new_turn_duration,
                        },
                    ) => {
                        *might_bonus = *new_might_bonus;
                        *turn_duration = *new_turn_duration;
                        break 'already_acquired_or_not;
                    }
                    _ => (),
                }
            }
            self.effects.push(new_effect);
        }

        blueprints
    }
}
impl Id {
    pub fn evaluate_might_bonuses(&mut self) -> i64 {
        let mut bonuses = 0;

        for effect in self.effects.iter() {
            match effect {
                Effect::Rise { might_bonus, .. } => {
                    bonuses += *might_bonus;
                }
                _ => (),
            }
        }

        bonuses
    }
    pub fn effects_on_take_damage<'a>(&mut self, mut damage: i64) -> (i64, Blueprints<'a>) {
        let mut blueprints = Vec::new();
        let mut i = 0;

        while i < self.effects.len() {
            match &mut self.effects[i] {
                Effect::Block => {
                    damage /= 2;
                    self.effects.remove(i);
                }
                _ => i += 1,
            }
        }

        (damage, blueprints)
    }
    pub fn effects_on_turn_end<'a>(&mut self, _battle: &mut Battle) -> Blueprints<'a> {
        let mut blueprints = Vec::new();
        let mut i = 0;

        while i < self.effects.len() {
            match &mut self.effects[i] {
                Effect::Rise {
                    turn_duration,
                    might_bonus,
                } => {
                    if *turn_duration <= 0 {
                        blueprints.push(ResponseBlueprint::default().set_content(f!(
                            "💪 | **{}** perdeu o efeito **Subir**. [**{}** {MGT_EMOJI}]",
                            self.name,
                            mark_thousands_and_show_sign(*might_bonus * -1),
                        )));

                        self.effects.remove(i);

                        continue;
                    } else {
                        *turn_duration -= 1;
                        i += 1;
                    }
                }
                _ => i += 1,
            }
        }

        blueprints
    }
}
impl Reflective for Id {
    const FOLDER_PATH: &'static str = "./database/ids";
    fn get_tag(&self) -> &str {
        self.tag.as_ref()
    }
}

#[derive(Deserialize, Serialize)]
pub enum Gender {
    Other = 0,
    Female = 1,
    Male = 2,
}

#[derive(Deserialize, Serialize)]
pub enum Age {
    Child = 1,
    Teen = 2,
    Young = 3,
    Adult = 4,
    MiddleAged = 5,
    Senior = 6,
}
