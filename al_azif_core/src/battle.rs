use crate::prelude::*;

#[derive(Default, Deserialize, Serialize)]
pub struct Battle {
    pub tag: Box<str>,
    pub opponents: HashMap<Box<str>, Opponent>,
    pub state: BattleState,
}
impl Battle {
    pub fn new(tag: &str) -> Self {
        Self {
            tag: tag.into(),
            ..Default::default()
        }
    }
    pub async fn advance(&mut self, bot: &impl AsBot) -> Result<Vec<ResponseBlueprint>> {
        let mut blueprints = Vec::new();

        if let Some(current_turn_owner_tag) = self.opponents
            .contains_key(&self.state.current_turn_owner_tag)
            .then(|| self.state.current_turn_owner_tag.clone())
        {
            blueprints.extend(self.end_turn(bot, &current_turn_owner_tag).await?);
        }
        
        loop {
            if let Some(next_turn_owner_tag) = self.get_next_turn_owner().map(Box::<str>::from) {
                blueprints.extend(self.start_turn(bot, &next_turn_owner_tag).await?);
                self.state.turn += 1;
                self.state.current_turn_owner_tag = next_turn_owner_tag;
                break;
            }

            self.state.phase += 1;
            blueprints.extend(self.start_next_phase(bot).await?);
        }

        Ok(blueprints)
    }
    pub async fn generate_turn_screen(&self, bot: &impl AsBot) -> Result<ResponseBlueprint> {
        let mut desc = String::new();

        for (id_tag, opponent) in &self.opponents {
            let number_of_filled_squares = (opponent.action_value * 10 / self.state.action_value_cap).clamp(0, 10) as usize;
            let filled_portion = "⬜".repeat(number_of_filled_squares);
            let empty_portion = "⬛".repeat(10 - number_of_filled_squares);

            let id_m = Mirror::<Id>::get(bot, id_tag).await?;
            let id = id_m.read().await;
            
            desc += &f!("**{}** [``{id_tag}``]\n{filled_portion}{empty_portion} {} / {}\n\n",
                id.ego.name,
                mark_thousands(opponent.action_value),
                mark_thousands(self.state.action_value_cap)
            );
        }

        let embed = CreateEmbed::default()
            .title(f!("Fase: {}", self.state.phase))
            .description(desc);

        Ok(ResponseBlueprint::default().embeds(vec![embed]))
    }
}
impl Battle {
    async fn end_turn(&mut self, bot: &impl AsBot, id_tag: &str) -> Result<Vec<ResponseBlueprint>> {
        let mut blueprints = Vec::new();
        
        let opponent = self.opponents.get_mut(id_tag).unwrap();
        opponent.action_value -= self.state.action_value_cap;

        blueprints.push(ResponseBlueprint::default().content(f!("Fim do turno de {} [``{id_tag}``].",
            Mirror::<Id>::get(bot, id_tag).await?.read().await.ego.name
        )));
        
        Ok(blueprints)
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
            .filter(|(_, opponent)| opponent.action_value >= self.state.action_value_cap)
            .map(|(tag, _)| tag.as_ref())
    }
    async fn start_next_phase(&mut self, bot: &impl AsBot) -> Result<Vec<ResponseBlueprint>> {
        let mut blueprints = Vec::new();

        for (id_tag, opponent) in &mut self.opponents {
            let id_m = Mirror::<Id>::get(bot, id_tag).await?;
            let id = id_m.read().await;
            opponent.action_value += id.attributes.movement;
        }

        blueprints.push(ResponseBlueprint::default().content("Iniciando a próxima fase..."));

        Ok(blueprints)
    }
    async fn start_turn(&mut self, bot: &impl AsBot, id_tag: &str) -> Result<Vec<ResponseBlueprint>> {
        let mut blueprints = Vec::new();

        blueprints.push(ResponseBlueprint::default().content(f!("Agora é o turno de {} [``{id_tag}``].",
            Mirror::<Id>::get(bot, id_tag).await?.read().await.ego.name
        )));

        Ok(blueprints)
    }
}
impl Reflective for Battle {
    const FOLDER_PATH: &'static str = "./database/battles";
    fn get_tag(&self) -> &str {
        self.tag.as_ref()
    }
}

#[derive(Default, Deserialize, Serialize)]
pub struct Opponent {
    pub action_value: i64,
}

#[derive(Default, Deserialize, Serialize)]
pub struct BattleState {
    pub turn: i64,
    pub phase: i64,
    pub current_turn_owner_tag: Box<str>,
    pub action_value_cap: i64,
}