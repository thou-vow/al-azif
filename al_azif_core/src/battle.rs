use crate::_prelude::*;

#[derive(Deserialize, Serialize)]
pub struct Battle {
    pub tag:                    FixedString,
    pub opponents:              HashMap<FixedString, Opponent>,
    pub turn_counter:           i64,
    pub phase_counter:          i64,
    pub current_turn_owner_tag: FixedString,
    pub current_moment:         Moment,
    pub turn_value_cap:         i64,
}
impl Battle {
    pub fn new(tag: impl AsRef<str>) -> Self { Battle::_new(tag.as_ref()) }

    fn _new(tag: &str) -> Self {
        Self {
            tag:                    FixedString::from_str_trunc(tag),
            opponents:              HashMap::new(),
            turn_counter:           0,
            phase_counter:          0,
            current_turn_owner_tag: FixedString::from_static_trunc(""),
            current_moment:         Moment::None,
            turn_value_cap:         0,
        }
    }
}
impl Battle {
    pub async fn generate_turn_screen<'a>(&mut self, bot: &impl AsBot) -> Result<ResponseBlueprint<'a>> {
        let mut new_desc = String::new();

        for (id_tag, opponent) in &mut self.opponents {
            let number_of_filled_squares = (opponent.turn_value * 7 / self.turn_value_cap).clamp(0, 7) as usize;
            let filled_portion = "⬜".repeat(number_of_filled_squares);
            let empty_portion = "⬛".repeat(7 - number_of_filled_squares);

            let id_m = Mirror::<Id>::get(bot, id_tag).await?;
            let id = id_m.read().await;

            new_desc += &f!(
                "**{}** `{id_tag}`\n{filled_portion}{empty_portion}  **{}** / **{}**",
                id.name,
                mark_thousands(opponent.turn_value),
                mark_thousands(self.turn_value_cap)
            );

            if opponent.last_total_increased_turn_value_amount != 0 {
                new_desc +=
                    &f!(" *{}*", mark_thousands_and_show_sign(opponent.last_total_increased_turn_value_amount));
                opponent.last_total_increased_turn_value_amount = 0;
            }

            new_desc += "\n\n";
        }

        let new_embed = CreateEmbed::default().title(f!("Fase {}", self.phase_counter)).description(new_desc);

        Ok(ResponseBlueprint::new().set_embeds(vec![new_embed]))
    }

    fn get_next_turn_owner(&self) -> Option<&str> {
        self.opponents
            .iter()
            .max_by(|(_, opponent1), (_, opponent2)| {
                let order = opponent1.turn_value.cmp(&opponent2.turn_value);
                if order == Ordering::Equal {
                    if rand::thread_rng().gen_bool(0.5) {
                        return Ordering::Less;
                    }
                    return Ordering::Greater;
                }
                order
            })
            .filter(|(_, opponent)| opponent.turn_value >= self.turn_value_cap)
            .map(|(id_tag, _)| id_tag.as_ref())
    }
}
impl Reflective for Battle {
    const FOLDER_PATH: &'static str = "./database/battles";

    fn get_tag(&self) -> &str { self.tag.as_ref() }
}

#[derive(Deserialize, Serialize)]
pub struct Opponent {
    pub tag:                                    FixedString,
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
    None,
    PrimaryAction {
        primary_action_tag: FixedString,
        attacker_tag:       FixedString,
        target_tag:         FixedString,
        security_key:       i64,
    },
}

pub async fn advance<'a>(bot: &impl AsBot, battle: &mut Battle) -> Result<Vec<ResponseBlueprint<'a>>> {
    let mut blueprints = Vec::new();

    if let Some(current_turn_owner_tag) = battle
        .opponents
        .contains_key(&battle.current_turn_owner_tag)
        .then(|| battle.current_turn_owner_tag.clone())
    {
        blueprints
            .extend(Mirror::<Id>::get(bot, &current_turn_owner_tag).await?.write().await.end_turn(battle).await?);
        blueprints
            .push(ResponseBlueprint::new().set_content("-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-"));
    }

    loop {
        let mut mov_values = Vec::new();
        for id_tag in battle.opponents.keys() {
            let id_m = Mirror::<Id>::get(bot, id_tag).await?;
            let id = id_m.read().await;
            mov_values.push(id.movement);
        }
        battle.turn_value_cap = *mov_values.iter().max_by(|x, y| x.cmp(y)).unwrap();

        if let Some(next_turn_owner_tag) = battle.get_next_turn_owner().map(FixedString::from_str_trunc) {
            blueprints.extend(
                Mirror::<Id>::get(bot, &next_turn_owner_tag).await?.write().await.start_turn(battle).await?,
            );
            battle.turn_counter += 1;
            battle.current_turn_owner_tag = next_turn_owner_tag;
            break;
        }

        battle.phase_counter += 1;

        blueprints.extend(start_next_phase(bot, battle).await?);
    }

    Ok(blueprints)
}

pub async fn start_next_phase<'a>(bot: &impl AsBot, battle: &mut Battle) -> Result<Blueprints<'a>> {
    let mut blueprints = Vec::new();

    for (id_tag, opponent) in &mut battle.opponents {
        let id_m = Mirror::<Id>::get(bot, id_tag).await?;
        let id = id_m.read().await;
        opponent.add_turn_value(id.movement);
    }

    blueprints.push(ResponseBlueprint::new().set_content("🚩 | Iniciando a próxima fase..."));

    Ok(blueprints)
}
