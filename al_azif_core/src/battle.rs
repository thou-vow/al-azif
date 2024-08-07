use crate::_prelude::*;

#[derive(Deserialize, Serialize)]
pub struct Battle {
    pub tag:                    FixedString<u8>,
    pub opponents:              Vec<Opponent>,
    pub turn_counter:           i64,
    pub phase_counter:          i64,
    pub current_turn_owner_tag: FixedString<u8>,
    pub current_moment:         Moment,
    pub turn_value_cap:         i64,
}
impl Battle {
    pub fn new(tag: impl AsRef<str>) -> Self { Battle::_new(tag.as_ref()) }

    fn _new(tag: &str) -> Self {
        Self {
            tag:                    FixedString::from_str_trunc(tag),
            opponents:              Vec::new(),
            turn_counter:           0,
            phase_counter:          0,
            current_turn_owner_tag: FixedString::from_static_trunc(""),
            current_moment:         Moment::Primary(PrimaryMoment { moment_owner_tag: FixedString::from_static_trunc("") }),
            turn_value_cap:         0,
        }
    }
}
impl Battle {
    pub async fn advance(&mut self, bot: &impl AsBot) -> Result<Blueprints> {
        let mut blueprints = Vec::new();

        if let Some(current_turn_owner_tag) = self
            .opponents
            .iter()
            .find(|opponent| opponent.tag == self.current_turn_owner_tag)
            .map(|opponent| &opponent.tag)
        {
            blueprints.extend(Mirror::<Id>::get(bot, &current_turn_owner_tag).await?.write().await.end_turn(bot, self).await?);
            blueprints.push(ResponseBlueprint::new().set_content("-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-"));
        }

        loop {
            let mut mov_values = Vec::new();
            for opponent in self.opponents.iter() {
                let id_m = Mirror::<Id>::get(bot, &opponent.tag).await?;
                let id = id_m.read().await;
                mov_values.push(id.movement);
            }
            self.turn_value_cap = *mov_values.iter().max_by(|x, y| x.cmp(y)).unwrap();

            if let Some(next_turn_owner_tag) = self.get_next_turn_owner().map(ToOwned::to_owned) {
                blueprints.extend(Mirror::<Id>::get(bot, &next_turn_owner_tag).await?.write().await.start_turn(self).await?);
                self.turn_counter += 1;
                self.current_turn_owner_tag = next_turn_owner_tag.clone();
                self.current_moment = Moment::Primary(PrimaryMoment { moment_owner_tag: next_turn_owner_tag });
                break;
            }

            self.phase_counter += 1;

            blueprints.extend(self.start_next_phase(bot).await?);
        }

        Ok(blueprints)
    }

    async fn start_next_phase(&mut self, bot: &impl AsBot) -> Result<Blueprints> {
        let mut blueprints = Vec::new();

        for opponent in self.opponents.iter_mut() {
            let id_m = Mirror::<Id>::get(bot, &opponent.tag).await?;
            let id = id_m.read().await;
            opponent.add_turn_value(id.movement);
        }

        blueprints.push(ResponseBlueprint::new().set_content("🚩 | Iniciando a próxima fase..."));

        Ok(blueprints)
    }

    pub async fn generate_turn_screen(&mut self, bot: &impl AsBot) -> Result<ResponseBlueprint> {
        let mut new_desc = String::new();

        for opponent in self.opponents.iter_mut() {
            let number_of_filled_squares = (opponent.turn_value * 7 / self.turn_value_cap).clamp(0, 7) as usize;
            let filled_portion = "⬜".repeat(number_of_filled_squares);
            let empty_portion = "⬛".repeat(7 - number_of_filled_squares);

            let id_m = Mirror::<Id>::get(bot, &opponent.tag).await?;
            let id = id_m.read().await;

            new_desc += &f!(
                "**{}** `{}`\n{filled_portion}{empty_portion}  **{}** / **{}**",
                id.name,
                opponent.tag,
                mark_thousands(opponent.turn_value),
                mark_thousands(self.turn_value_cap)
            );

            if opponent.last_total_increased_turn_value_amount != 0 {
                new_desc += &f!(" *{}*", mark_thousands_and_show_sign(opponent.last_total_increased_turn_value_amount));
                opponent.last_total_increased_turn_value_amount = 0;
            }

            new_desc += "\n\n";
        }

        let new_embed = CreateEmbed::default()
            .author(CreateEmbedAuthor::new(lang_diff!(bot,
                en: f!("Phase {}", self.phase_counter),
                pt: f!("Fase {}", self.phase_counter)
            )))
            .description(new_desc);

        Ok(ResponseBlueprint::new().set_embeds(vec![new_embed]))
    }

    fn get_next_turn_owner(&self) -> Option<&FixedString<u8>> {
        self.opponents
            .iter()
            .max_by(|opponent1, opponent2| {
                let order = opponent1.turn_value.cmp(&opponent2.turn_value);
                if order == Ordering::Equal {
                    if rand::thread_rng().gen_bool(0.5) {
                        return Ordering::Less;
                    }
                    return Ordering::Greater;
                }
                order
            })
            .filter(|opponent| opponent.turn_value >= self.turn_value_cap)
            .map(|opponent| &opponent.tag)
    }
}
impl Reflective for Battle {
    const FOLDER_PATH: &'static str = "./database/battles";

    fn get_tag(&self) -> &str { self.tag.as_ref() }
}

#[derive(Deserialize, Serialize)]
pub struct Opponent {
    pub tag:                                    FixedString<u8>,
    pub turn_value:                             i64,
    pub last_total_increased_turn_value_amount: i64,
}
impl Opponent {
    pub fn add_turn_value(&mut self, add: i64) {
        self.turn_value += add;
        self.last_total_increased_turn_value_amount += add;
    }

    pub fn sub_turn_value(&mut self, sub: i64) {
        self.turn_value -= sub;
        self.last_total_increased_turn_value_amount -= sub;
    }
}

#[derive(Deserialize, Serialize)]
pub enum Moment {
    Primary(PrimaryMoment),
    Reactive(ReactiveMoment),
}
#[derive(Clone, Deserialize, Serialize)]
pub struct PrimaryMoment {
    pub moment_owner_tag: FixedString<u8>,
}
#[derive(Clone, Deserialize, Serialize)]
pub struct ReactiveMoment {
    pub primary_moment_owner_tag: FixedString<u8>,
    pub primary_action_tag:       FixedString<u8>,
    pub target_tags:              Vec<FixedString<u8>>,
    pub target_index:             usize,
}
