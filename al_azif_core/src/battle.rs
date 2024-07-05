use crate::prelude::*;

#[derive(Deserialize, Serialize)]
pub struct Battle {
    pub tag: FixedString,
    pub opponents: HashMap<FixedString, Opponent>,
    pub action_counter: i64,
    pub turn_counter: i64,
    pub phase_counter: i64,
    pub current_turn_owner_tag: FixedString,
    pub current_moment: Moment,
    pub action_value_cap: i64,
}
impl Battle {
    pub fn new(tag: &str) -> Self {
        Self {
            tag: FixedString::from_str_trunc(tag),
            opponents: HashMap::new(),
            action_counter: 0,
            turn_counter: 0,
            phase_counter: 0,
            current_turn_owner_tag: FixedString::from_static_trunc(""),
            current_moment: Moment::None,
            action_value_cap: 0,
        }
    }
    pub async fn generate_turn_screen(&self, bot: &impl AsBot) -> Result<ResponseBlueprint> {
        let mut desc = String::new();

        for (id_tag, opponent) in &self.opponents {
            let number_of_filled_squares = (opponent.action_value * 10 / self.action_value_cap).clamp(0, 10) as usize;
            let filled_portion = "⬜".repeat(number_of_filled_squares);
            let empty_portion = "⬛".repeat(10 - number_of_filled_squares);

            let id_m = Mirror::<Id>::get(bot, id_tag).await?;
            let id = id_m.read().await;
            
            desc += &f!("**{}** [``{id_tag}``]\n{filled_portion}{empty_portion} {} / {}\n\n",
                id.name,
                mark_thousands(opponent.action_value),
                mark_thousands(self.action_value_cap)
            );
        }

        let embed = CreateEmbed::default()
            .title(f!("Fase: {}", self.phase_counter))
            .description(desc);

        Ok(ResponseBlueprint::default().embeds(vec![embed]))
    }
    fn get_next_turn_owner(&self) -> Option<&str> {
        self.opponents.iter()
            .max_by(|(_, opponent1), (_, opponent2)| {
                let order = opponent1.action_value.cmp(&opponent2.action_value);
                if order == Ordering::Equal {
                    if rand::thread_rng().gen_bool(0.5) {
                        return Ordering::Less;
                    }
                    return Ordering::Greater;
                }
                order
            })
            .filter(|(_, opponent)| opponent.action_value >= self.action_value_cap)
            .map(|(id_tag, _)| id_tag.as_ref())
    }
}
impl Reflective for Battle {
    const FOLDER_PATH: &'static str = "./database/battles";
    fn get_tag(&self) -> &str {
        self.tag.as_ref()
    }
}

#[derive(Deserialize, Serialize)]
pub struct Opponent {
    pub tag: FixedString,
    pub action_value: i64,
}

#[derive(Deserialize, Serialize)]
pub enum Moment {
    None,
    Attacking {
        action_tag: FixedString,
        user_tag: FixedString,
        target_tag: FixedString,
    },
    Defending,
}

pub async fn advance(bot: &impl AsBot, battle: &mut Battle) -> Result<Vec<ResponseBlueprint>> {
    let mut blueprints = Vec::new();

    if let Some(current_turn_owner_tag) = battle.opponents
        .contains_key(&battle.current_turn_owner_tag)
        .then(|| battle.current_turn_owner_tag.clone())
    {
        blueprints.extend(
            Mirror::<Id>::get(bot, &current_turn_owner_tag).await?
                .write().await
                .end_turn(battle).await?
        );
    }
    
    loop {
        if let Some(next_turn_owner_tag) = battle.get_next_turn_owner().map(FixedString::from_str_trunc) {
            blueprints.extend(
                Mirror::<Id>::get(bot, &next_turn_owner_tag).await?
                    .write().await.start_turn(battle).await?
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

pub async fn start_next_phase(bot: &impl AsBot, battle: &mut Battle) -> Result<Vec<ResponseBlueprint>> {
    let mut blueprints = Vec::new();

    for (id_tag, opponent) in &mut battle.opponents {
        let id_m = Mirror::<Id>::get(bot, id_tag).await?;
        let id = id_m.read().await;
        opponent.action_value += id.movement;
    }

    blueprints.push(ResponseBlueprint::default().content("Iniciando a próxima fase..."));

    Ok(blueprints)
}