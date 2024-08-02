use crate::_prelude::*;

pub struct Empty;
pub struct FoundBattle {
    tag: FixedString,
    m:   Mirror<Battle>,
}
pub struct FoundPrimaryMoment(PrimaryMoment);
pub struct FoundReactiveMoment(ReactiveMoment);
pub struct FoundUser {
    tag: FixedString,
    m:   Mirror<Id>,
}
pub struct FoundTargets<const LEN: usize> {
    tags_and_ms: [(FixedString, Mirror<Id>); LEN],
}

pub struct FoundOptionalTargets<const LEN: usize> {
    tags_and_ms: [Option<(FixedString, Mirror<Id>)>; LEN],
}

pub struct Setting<'a, Bot: AsBot, BattleS = Empty, MomentS = Empty, UserS = Empty, TargetsS = Empty, OptionalTargetsS = Empty> {
    pub bot:                &'a Bot,
    args:                   VecDeque<&'a str>,
    battle_state:           BattleS,
    moment_state:           MomentS,
    user_state:             UserS,
    targets_state:          TargetsS,
    optional_targets_state: OptionalTargetsS,
}

// Battle: empty,   Moment: empty,   User: empty,   Targets: empty,   Optional Targets: empty
impl<'a, Bot: AsBot> Setting<'a, Bot, Empty, Empty, Empty, Empty, Empty> {
    pub fn new(bot: &'a Bot, args: VecDeque<&'a str>) -> Self {
        Self {
            bot,
            args,
            battle_state: Empty,
            moment_state: Empty,
            user_state: Empty,
            targets_state: Empty,
            optional_targets_state: Empty,
        }
    }

    pub async fn fetch_battle(self, tag: impl AsRef<str>) -> Result<Setting<'a, Bot, FoundBattle>> {
        let tag = FixedString::from_str_trunc(tag.as_ref());

        let Ok(m) = Mirror::<Battle>::get(self.bot, &tag).await else {
            return Err(PrefixError::Expected(vec![ResponseBlueprint::with_content(lang_diff!(self.bot,
                en: "No battle is currently happening in this channel.",
                pt: "Não há uma batalha acontecendo neste canal."
            ))]));
        };

        Ok(Setting {
            bot:                    self.bot,
            args:                   self.args,
            battle_state:           FoundBattle { tag, m },
            moment_state:           self.moment_state,
            user_state:             self.user_state,
            targets_state:          self.targets_state,
            optional_targets_state: self.optional_targets_state,
        })
    }
}

// Battle: found,   Moment: empty,   User: empty,   Targets: empty,   Optional Targets: empty
impl<'a, Bot: AsBot> Setting<'a, Bot, FoundBattle, Empty, Empty, Empty, Empty> {
    pub async fn require_primary_moment(self) -> Result<Setting<'a, Bot, FoundBattle, FoundPrimaryMoment, Empty, Empty, Empty>> {
        let battle = self.battle_state.m.read().await;

        let Moment::Primary(primary) = &battle.current_moment else {
            return Err(PrefixError::Expected(vec![ResponseBlueprint::with_content(lang_diff!(self.bot,
                en: "You can't use this command right now.",
                pt: "Você não pode usar este comando agora."
            ))]));
        };
        let primary = primary.to_owned();

        battle.unread();

        Ok(Setting {
            bot:                    self.bot,
            args:                   self.args,
            battle_state:           self.battle_state,
            moment_state:           FoundPrimaryMoment(primary.clone()),
            user_state:             self.user_state,
            targets_state:          self.targets_state,
            optional_targets_state: self.optional_targets_state,
        })
    }

    pub async fn require_reactive_moment(self) -> Result<Setting<'a, Bot, FoundBattle, FoundReactiveMoment, Empty, Empty, Empty>> {
        let battle = self.battle_state.m.read().await;

        let Moment::Reactive(reactive) = &battle.current_moment else {
            return Err(PrefixError::Expected(vec![ResponseBlueprint::with_content(lang_diff!(self.bot,
                en: "You can't use this command right now.",
                pt: "Você não pode usar este comando agora."
            ))]));
        };
        let reactive = reactive.to_owned();

        battle.unread();

        Ok(Setting {
            bot:                    self.bot,
            args:                   self.args,
            battle_state:           self.battle_state,
            moment_state:           FoundReactiveMoment(reactive.clone()),
            user_state:             self.user_state,
            targets_state:          self.targets_state,
            optional_targets_state: self.optional_targets_state,
        })
    }
}

// Battle: found,   Moment: primary,   User: empty,   Targets: empty,   Optional Targets: empty
impl<'a, Bot: AsBot> Setting<'a, Bot, FoundBattle, FoundPrimaryMoment, Empty, Empty, Empty> {
    pub async fn fetch_user(mut self) -> Result<Setting<'a, Bot, FoundBattle, FoundPrimaryMoment, FoundUser, Empty, Empty>> {
        let tag = match self.args.pop_front() {
            Some(".") => self.moment_state.0.moment_owner_tag.clone(),
            Some(tag) => {
                let tag = FixedString::from_str_trunc(tag);

                if self.moment_state.0.moment_owner_tag != tag {
                    return Err(PrefixError::Expected(vec![ResponseBlueprint::with_content(lang_diff!(self.bot,
                        en: "You can't use this action right now.",
                        pt: "Você não pode usar esta ação agora."
                    ))]));
                }

                tag
            },
            None => {
                return Err(PrefixError::Expected(vec![ResponseBlueprint::with_content(lang_diff!(self.bot,
                    en: "You must specify a user.",
                    pt: "Você deve especificar um usuário."
                ))]))
            },
        };

        let m = match Mirror::<Id>::get(self.bot, &tag).await {
            Ok(user_m) => user_m,
            Err(e) => return Err(e.into()),
        };

        Ok(Setting {
            bot:                    self.bot,
            args:                   self.args,
            battle_state:           self.battle_state,
            moment_state:           self.moment_state,
            user_state:             FoundUser { tag, m },
            targets_state:          self.targets_state,
            optional_targets_state: self.optional_targets_state,
        })
    }
}

// Battle: found,   Moment: reactive,   User: empty,   Targets: empty,   Optional Targets: empty
impl<'a, Bot: AsBot> Setting<'a, Bot, FoundBattle, FoundReactiveMoment, Empty, Empty, Empty> {
    pub async fn fetch_user(mut self) -> Result<Setting<'a, Bot, FoundBattle, FoundReactiveMoment, FoundUser, Empty, Empty>> {
        let tag = match self.args.pop_front() {
            Some(".") => match self.moment_state.0.target_tags.get(self.moment_state.0.target_index) {
                Some(tag) => tag.clone(),
                None => {
                    return Err(PrefixError::ReactiveMomentTargetTagsIndexOutOfBounds {
                        primary_action_tag: self.moment_state.0.primary_action_tag,
                        index:              self.moment_state.0.target_index,
                    })
                },
            },
            Some(tag) => {
                let tag = FixedString::from_str_trunc(tag);

                if !self.battle_state.m.read().await.opponents.contains_key(&tag) {
                    return Err(PrefixError::Expected(vec![ResponseBlueprint::with_content(lang_diff!(self.bot,
                        en: "You are not in the battle.",
                        pt: "Você não está na batalha."
                    ))]));
                }

                tag
            },
            None => {
                return Err(PrefixError::Expected(vec![ResponseBlueprint::with_content(lang_diff!(self.bot,
                    en: "You must specify a user.",
                    pt: "Você deve especificar um usuário."
                ))]))
            },
        };

        let m = match Mirror::<Id>::get(self.bot, &tag).await {
            Ok(user_m) => user_m,
            Err(e) => return Err(e.into()),
        };

        Ok(Setting {
            bot:                    self.bot,
            args:                   self.args,
            battle_state:           self.battle_state,
            moment_state:           self.moment_state,
            user_state:             FoundUser { tag, m },
            targets_state:          self.targets_state,
            optional_targets_state: self.optional_targets_state,
        })
    }
}

// Battle: found,   Moment: any,   User: found,   Targets: empty,   Optional Targets: empty
impl<'a, Bot: AsBot, MomentS> Setting<'a, Bot, FoundBattle, MomentS, FoundUser, Empty, Empty> {
    pub async fn fetch_targets<const LEN: usize>(
        mut self,
        missing_target_arg_messages: [&'static str; LEN],
    ) -> Result<Setting<'a, Bot, FoundBattle, MomentS, FoundUser, FoundTargets<LEN>>> {
        let mut tags_and_ms: [Option<(FixedString, Mirror<Id>)>; LEN] = [(); LEN].map(|_| None);

        let battle = self.battle_state.m.read().await;

        for i in 0 .. LEN {
            match self.args.pop_front() {
                Some(tag) => {
                    let Ok(m) = Mirror::<Id>::get(self.bot, tag).await else {
                        return Err(PrefixError::Expected(vec![ResponseBlueprint::with_content(lang_diff!(self.bot,
                            en: f!("The Id `{tag}` was not found."),
                            pt: f!("O Id `{tag}` não foi encontrado.")
                        ))]));
                    };

                    if !battle.opponents.contains_key(tag) {
                        return Err(PrefixError::Expected(vec![ResponseBlueprint::with_content(lang_diff!(self.bot,
                            en: f!("The Id `{tag}` is not in the battle."),
                            pt: f!("O Id `{tag}` não está na batalha.")
                        ))]));
                    }

                    tags_and_ms[i] = Some((FixedString::from_str_trunc(tag), m));
                },
                None => {
                    return Err(PrefixError::Expected(vec![ResponseBlueprint::with_content(missing_target_arg_messages[i])]));
                },
            }
        }

        battle.unread();

        let tags_and_ms = tags_and_ms.map(|t| t.expect("This should never happen."));
        
        Ok(Setting {
            bot:                    self.bot,
            args:                   self.args,
            battle_state:           self.battle_state,
            moment_state:           self.moment_state,
            user_state:             self.user_state,
            targets_state:          FoundTargets { tags_and_ms },
            optional_targets_state: self.optional_targets_state,
        })
    }
}

// Battle: found,   Moment: any,   User: found,   Targets: any,   Optional Targets: empty
impl<'a, Bot: AsBot, MomentS, TargetsS> Setting<'a, Bot, FoundBattle, MomentS, FoundUser, TargetsS, Empty> {
    pub async fn fetch_optional_targets<const LEN: usize>(
        mut self,
    ) -> Result<Setting<'a, Bot, FoundBattle, MomentS, FoundUser, TargetsS, FoundOptionalTargets<LEN>>> {
        let mut tags_and_ms: [Option<(FixedString, Mirror<Id>)>; LEN] = [(); LEN].map(|_| None);

        let battle = self.battle_state.m.read().await;

        for tag_and_m in tags_and_ms.iter_mut().take(LEN) {
            if let Some(tag) = self.args.pop_front() {
                let Ok(m) = Mirror::<Id>::get(self.bot, tag).await else {
                    return Err(PrefixError::Expected(vec![ResponseBlueprint::with_content(lang_diff!(self.bot,
                        en: f!("The Id `{tag}` was not found."),
                        pt: f!("O Id `{tag}` não foi encontrado.")
                    ))]));
                };

                if !battle.opponents.contains_key(tag) {
                    return Err(PrefixError::Expected(vec![ResponseBlueprint::with_content(lang_diff!(self.bot,
                        en: f!("The Id `{tag}` is not in the battle."),
                        pt: f!("O Id `{tag}` não está na batalha.")
                    ))]));
                }

                *tag_and_m = Some((FixedString::from_str_trunc(tag), m));
            }
        }

        battle.unread();

        Ok(Setting {
            bot:                    self.bot,
            args:                   self.args,
            battle_state:           self.battle_state,
            moment_state:           self.moment_state,
            user_state:             self.user_state,
            targets_state:          self.targets_state,
            optional_targets_state: FoundOptionalTargets { tags_and_ms },
        })
    }
}

// Battle: found,   Moment: any,   User: any,   Targets: any,   Optional Targets: any
impl<'a, Bot: AsBot, MomentS, UserS, TargetsS, OptionalTargetsS> Setting<'a, Bot, FoundBattle, MomentS, UserS, TargetsS, OptionalTargetsS> {
    pub fn get_battle_tag(&self) -> &FixedString { &self.battle_state.tag }

    pub fn get_battle_mirror(&self) -> &Mirror<Battle> { &self.battle_state.m }
}

// Battle: any,   Moment: reactive,   User: any,   Targets: any,   Optional Targets: any
impl<'a, Bot: AsBot, BattleS, UserS, TargetsS, OptionalTargetsS>
    Setting<'a, Bot, BattleS, FoundReactiveMoment, UserS, TargetsS, OptionalTargetsS>
{
    pub fn get_primary_moment_owner_tag(&self) -> &FixedString { &self.moment_state.0.primary_moment_owner_tag }

    pub fn get_primary_action_tag(&self) -> &FixedString { &self.moment_state.0.primary_action_tag }
}

// Battle: any,   Moment: any,   User: found,   Targets: found,   Optional Targets: any
impl<'a, Bot: AsBot, BattleS, MomentS, const LEN: usize, OptionalTargetsS>
    Setting<'a, Bot, BattleS, MomentS, FoundUser, FoundTargets<LEN>, OptionalTargetsS>
{
    pub fn unallow_self_target<const INDEX: usize>(self, error_message: &'static str) -> Result<Self> {
        if self.targets_state.tags_and_ms[INDEX].0 == self.user_state.tag {
            return Err(PrefixError::Expected(vec![ResponseBlueprint::with_content(error_message)]));
        }

        Ok(self)
    }

    pub fn unallow_any_self_target(self, error_message: &'static str) -> Result<Self> {
        if self.targets_state.tags_and_ms.iter().any(|(tag, _)| *tag == self.user_state.tag) {
            return Err(PrefixError::Expected(vec![ResponseBlueprint::with_content(error_message)]));
        }

        Ok(self)
    }
}

// Battle: any,   Moment: any,   User: found,   Targets: any,   Optional Targets: found
impl<'a, Bot: AsBot, BattleS, MomentS, TargetsS, const LEN: usize>
    Setting<'a, Bot, BattleS, MomentS, FoundUser, TargetsS, FoundOptionalTargets<LEN>>
{
    pub fn unallow_self_optional_target<const INDEX: usize>(self, error_message: &'static str) -> Result<Self> {
        if let Some((target_tag, _)) = &self.optional_targets_state.tags_and_ms[INDEX] {
            if *target_tag == self.user_state.tag {
                return Err(PrefixError::Expected(vec![ResponseBlueprint::with_content(error_message)]));
            }
        }

        Ok(self)
    }

    pub fn unallow_any_self_optional_target(self, error_message: &'static str) -> Result<Self> {
        if self
            .optional_targets_state
            .tags_and_ms
            .iter()
            .filter_map(|tag| if let Some((target_tag, _)) = tag { Some(target_tag) } else { None })
            .any(|tag| *tag == self.user_state.tag)
        {
            return Err(PrefixError::Expected(vec![ResponseBlueprint::with_content(error_message)]));
        }

        Ok(self)
    }
}

// Battle: any,   Moment: any,   User: found,   Targets: any,   Optional Targets: any
impl<'a, Bot: AsBot, BattleS, MomentS, TargetsS, OptionalTargetsS>
    Setting<'a, Bot, BattleS, MomentS, FoundUser, TargetsS, OptionalTargetsS>
{
    pub fn get_user_tag(&self) -> &FixedString { &self.user_state.tag }

    pub fn get_user_mirror(&self) -> &Mirror<Id> { &self.user_state.m }
}

// Battle: any,   Moment: any,   User: any,   Targets: found,   Optional Targets: any
impl<'a, Bot: AsBot, BattleS, MomentS, UserS, const LEN: usize, OptionalTargetsS>
    Setting<'a, Bot, BattleS, MomentS, UserS, FoundTargets<LEN>, OptionalTargetsS>
{
    pub fn get_target_tags_and_ms(&self) -> &[(FixedString, Mirror<Id>); LEN] { &self.targets_state.tags_and_ms }

    pub fn get_target_tags(&self) -> [&FixedString; LEN] {
        let mut tags: [&FixedString; LEN] = unsafe { mem::zeroed() };
        for (i, tags_and_ms) in self.targets_state.tags_and_ms.iter().enumerate() {
            tags[i] = &tags_and_ms.0;
        }
        tags
    }

    pub fn get_target_ms(&self) -> [&Mirror<Id>; LEN] {
        let mut ms: [&Mirror<Id>; LEN] = unsafe { mem::zeroed() };
        for (i, tags_and_ms) in self.targets_state.tags_and_ms.iter().enumerate() {
            ms[i] = &tags_and_ms.1;
        }
        ms
    }
}

// Battle: any,   Moment: any,   User: any,   Targets: any,   Optional Targets: found
impl<'a, Bot: AsBot, BattleS, MomentS, UserS, TargetsS, const LEN: usize>
    Setting<'a, Bot, BattleS, MomentS, UserS, TargetsS, FoundOptionalTargets<LEN>>
{
    pub fn get_optional_target_tags_and_ms(&self) -> &[Option<(FixedString, Mirror<Id>)>; LEN] { &self.optional_targets_state.tags_and_ms }

    pub fn get_optional_target_tags(&self) -> [Option<&FixedString>; LEN] {
        let mut tags: [Option<&FixedString>; LEN] = unsafe { mem::zeroed() };
        for (i, tags_and_ms) in self.optional_targets_state.tags_and_ms.iter().enumerate() {
            tags[i] = tags_and_ms.as_ref().map(|(tag, _)| tag);
        }
        tags
    }

    pub fn get_optional_target_ms(&self) -> [Option<&FixedString>; LEN] {
        let mut ms: [Option<&FixedString>; LEN] = unsafe { mem::zeroed() };
        for (i, tags_and_ms) in self.optional_targets_state.tags_and_ms.iter().enumerate() {
            ms[i] = tags_and_ms.as_ref().map(|(tag, _)| tag);
        }
        ms
    }
}
