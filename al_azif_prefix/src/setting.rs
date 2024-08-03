use crate::_prelude::*;

pub struct Empty;
pub struct FoundBattle {
    tag: FixedString<u8>,
    m:   Mirror<Battle>,
}
pub struct FoundPrimaryMoment(PrimaryMoment);
pub struct FoundReactiveMoment(ReactiveMoment);
pub struct FoundUser {
    tag: FixedString<u8>,
    m:   Mirror<Id>,
}
pub struct FoundTargets<'a, const LEN: usize> {
    tags_and_ms: [(&'a str, Mirror<Id>); LEN],
}
pub struct FoundOptionalTargets<'a, const LEN: usize> {
    tags_and_ms: [Option<(&'a str, Mirror<Id>)>; LEN],
}
pub struct FoundReservedArgs<'a, const LEN: usize>([&'a str; LEN]);
pub struct FoundOptionalReservedArgs<'a, const LEN: usize>([Option<&'a str>; LEN]);

pub struct Setting<
    'a,
    Bot: AsBot,
    BattleS = Empty,
    MomentS = Empty,
    UserS = Empty,
    TargetsS = Empty,
    OptionalTargetsS = Empty,
    ReservedArgsS = Empty,
    OptionalReservedArgsS = Empty,
> {
    pub bot:                      &'a Bot,
    args:                         VecDeque<&'a str>,
    battle_state:                 BattleS,
    moment_state:                 MomentS,
    user_state:                   UserS,
    targets_state:                TargetsS,
    optional_targets_state:       OptionalTargetsS,
    reserved_args_state:          ReservedArgsS,
    optional_reserved_args_state: OptionalReservedArgsS,
}

// Battle: empty,   Moment: empty,   User: empty,   Targets: empty,   Optional Targets: empty   Reserved Args: empty   Optional Reserved Args: empty
impl<'a, Bot: AsBot> Setting<'a, Bot, Empty, Empty, Empty, Empty, Empty, Empty> {
    pub fn new(bot: &'a Bot, args: VecDeque<&'a str>) -> Self {
        Self {
            bot,
            args,
            battle_state: Empty,
            moment_state: Empty,
            user_state: Empty,
            targets_state: Empty,
            optional_targets_state: Empty,
            reserved_args_state: Empty,
            optional_reserved_args_state: Empty,
        }
    }

    pub async fn fetch_battle(self, tag: String) -> Result<Setting<'a, Bot, FoundBattle, Empty, Empty, Empty, Empty, Empty, Empty>> {
        let tag = FixedString::from_string_trunc(tag);

        let Ok(m) = Mirror::<Battle>::get(self.bot, &tag).await else {
            return Err(PrefixError::Expected(vec![ResponseBlueprint::with_content(lang_diff!(self.bot,
                en: "No battle is currently happening in this channel.",
                pt: "Não há uma batalha acontecendo neste canal."
            ))]));
        };

        Ok(Setting {
            bot:                          self.bot,
            args:                         self.args,
            battle_state:                 FoundBattle { tag, m },
            moment_state:                 self.moment_state,
            user_state:                   self.user_state,
            targets_state:                self.targets_state,
            optional_targets_state:       self.optional_targets_state,
            reserved_args_state:          self.reserved_args_state,
            optional_reserved_args_state: self.optional_reserved_args_state,
        })
    }
}

// Battle: found,   Moment: empty,   User: empty,   Targets: empty,   Optional Targets: empty   Reserved Args: empty   Optional Reserved Args: empty
impl<'a, Bot: AsBot> Setting<'a, Bot, FoundBattle, Empty, Empty, Empty, Empty, Empty, Empty> {
    pub async fn require_primary_moment(
        self,
    ) -> Result<Setting<'a, Bot, FoundBattle, FoundPrimaryMoment, Empty, Empty, Empty, Empty, Empty>> {
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
            bot:                          self.bot,
            args:                         self.args,
            battle_state:                 self.battle_state,
            moment_state:                 FoundPrimaryMoment(primary.clone()),
            user_state:                   self.user_state,
            targets_state:                self.targets_state,
            optional_targets_state:       self.optional_targets_state,
            reserved_args_state:          self.reserved_args_state,
            optional_reserved_args_state: self.optional_reserved_args_state,
        })
    }

    pub async fn require_reactive_moment(
        self,
    ) -> Result<Setting<'a, Bot, FoundBattle, FoundReactiveMoment, Empty, Empty, Empty, Empty, Empty>> {
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
            bot:                          self.bot,
            args:                         self.args,
            battle_state:                 self.battle_state,
            moment_state:                 FoundReactiveMoment(reactive.clone()),
            user_state:                   self.user_state,
            targets_state:                self.targets_state,
            optional_targets_state:       self.optional_targets_state,
            reserved_args_state:          self.reserved_args_state,
            optional_reserved_args_state: self.optional_reserved_args_state,
        })
    }
}

// Battle: found,   Moment: primary,   User: empty,   Targets: empty,   Optional Targets: empty   Reserved Args: empty   Optional Reserved Args: empty
impl<'a, Bot: AsBot> Setting<'a, Bot, FoundBattle, FoundPrimaryMoment, Empty, Empty, Empty, Empty, Empty> {
    pub async fn fetch_user(mut self) -> Result<Setting<'a, Bot, FoundBattle, FoundPrimaryMoment, FoundUser, Empty, Empty, Empty, Empty>> {
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
            bot:                          self.bot,
            args:                         self.args,
            battle_state:                 self.battle_state,
            moment_state:                 self.moment_state,
            user_state:                   FoundUser { tag, m },
            targets_state:                self.targets_state,
            optional_targets_state:       self.optional_targets_state,
            reserved_args_state:          self.reserved_args_state,
            optional_reserved_args_state: self.optional_reserved_args_state,
        })
    }
}

// Battle: found,   Moment: reactive,   User: empty,   Targets: empty,   Optional Targets: empty   Reserved Args: empty   Optional Reserved Args: empty
impl<'a, Bot: AsBot> Setting<'a, Bot, FoundBattle, FoundReactiveMoment, Empty, Empty, Empty, Empty, Empty> {
    pub async fn fetch_user(mut self) -> Result<Setting<'a, Bot, FoundBattle, FoundReactiveMoment, FoundUser, Empty, Empty, Empty, Empty>> {
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
            bot:                          self.bot,
            args:                         self.args,
            battle_state:                 self.battle_state,
            moment_state:                 self.moment_state,
            user_state:                   FoundUser { tag, m },
            targets_state:                self.targets_state,
            optional_targets_state:       self.optional_targets_state,
            reserved_args_state:          self.reserved_args_state,
            optional_reserved_args_state: self.optional_reserved_args_state,
        })
    }
}

// Battle: found,   Moment: any,   User: found,   Targets: empty,   Optional Targets: empty   Reserved Args: any   Optional Reserved Args: empty
impl<'a, Bot: AsBot, MomentS, ReservedArgsS> Setting<'a, Bot, FoundBattle, MomentS, FoundUser, Empty, Empty, ReservedArgsS, Empty> {
    pub async fn fetch_targets<const LEN: usize>(
        mut self,
        missing_target_arg_messages: [&'static str; LEN],
    ) -> Result<Setting<'a, Bot, FoundBattle, MomentS, FoundUser, FoundTargets<'a, LEN>, Empty, ReservedArgsS, Empty>> {
        let mut tags_and_ms: [Option<(&'a str, Mirror<Id>)>; LEN] = [(); LEN].map(|_| None);

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

                    tags_and_ms[i] = Some((tag, m));
                },
                None => {
                    return Err(PrefixError::Expected(vec![ResponseBlueprint::with_content(missing_target_arg_messages[i])]));
                },
            }
        }

        battle.unread();

        let tags_and_ms = tags_and_ms.map(|op| op.expect("This should never happen."));

        Ok(Setting {
            bot:                          self.bot,
            args:                         self.args,
            battle_state:                 self.battle_state,
            moment_state:                 self.moment_state,
            user_state:                   self.user_state,
            targets_state:                FoundTargets { tags_and_ms },
            optional_targets_state:       self.optional_targets_state,
            reserved_args_state:          self.reserved_args_state,
            optional_reserved_args_state: self.optional_reserved_args_state,
        })
    }
}

// Battle: found,   Moment: any,   User: found,   Targets: any,   Optional Targets: empty   Reserved Args: any   Optional Reserved Args: empty
impl<'a, Bot: AsBot, MomentS, TargetsS, ReservedArgsS>
    Setting<'a, Bot, FoundBattle, MomentS, FoundUser, TargetsS, Empty, ReservedArgsS, Empty>
{
    pub async fn fetch_optional_targets<const LEN: usize>(
        mut self,
    ) -> Result<Setting<'a, Bot, FoundBattle, MomentS, FoundUser, TargetsS, FoundOptionalTargets<'a, LEN>, ReservedArgsS>> {
        let mut tags_and_ms: [Option<(&'a str, Mirror<Id>)>; LEN] = [(); LEN].map(|_| None);

        let battle = self.battle_state.m.read().await;

        for tag_and_m in tags_and_ms.iter_mut() {
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

                *tag_and_m = Some((tag, m));
            }
        }

        battle.unread();

        Ok(Setting {
            bot:                          self.bot,
            args:                         self.args,
            battle_state:                 self.battle_state,
            moment_state:                 self.moment_state,
            user_state:                   self.user_state,
            targets_state:                self.targets_state,
            optional_targets_state:       FoundOptionalTargets { tags_and_ms },
            reserved_args_state:          self.reserved_args_state,
            optional_reserved_args_state: self.optional_reserved_args_state,
        })
    }
}

// Battle: found,   Moment: any,   User: any,   Targets: any,   Optional Targets: any   Reserved Args: any   Optional Reserved Args: any
impl<'a, Bot: AsBot, MomentS, UserS, TargetsS, OptionalTargetsS, ReservedArgsS, OptionalReservedArgsS>
    Setting<'a, Bot, FoundBattle, MomentS, UserS, TargetsS, OptionalTargetsS, ReservedArgsS, OptionalReservedArgsS>
{
    pub fn get_battle_tag(&self) -> &FixedString<u8> { &self.battle_state.tag }

    pub fn get_battle_mirror(&self) -> &Mirror<Battle> { &self.battle_state.m }
}

// Battle: any,   Moment: reactive,   User: any,   Targets: any,   Optional Targets: any   Reserved Args: any   Optional Reserved Args: any
impl<'a, Bot: AsBot, BattleS, UserS, TargetsS, OptionalTargetsS, ReservedArgsS, OptionalReservedArgsS>
    Setting<'a, Bot, BattleS, FoundReactiveMoment, UserS, TargetsS, OptionalTargetsS, ReservedArgsS, OptionalReservedArgsS>
{
    pub fn get_primary_moment_owner_tag(&self) -> &FixedString<u8> { &self.moment_state.0.primary_moment_owner_tag }

    pub fn get_primary_action_tag(&self) -> &FixedString<u8> { &self.moment_state.0.primary_action_tag }
}

// Battle: any,   Moment: any,   User: found,   Targets: found,   Optional Targets: found   Reserved Args: any   Optional Reserved Args: any
impl<'a, Bot: AsBot, BattleS, MomentS, const L1: usize, const L2: usize, ReservedArgsS, OptionalReservedArgsS>
    Setting<'a, Bot, BattleS, MomentS, FoundUser, FoundTargets<'a, L1>, FoundOptionalTargets<'a, L2>, ReservedArgsS, OptionalReservedArgsS>
{
    pub fn unallow_any_self_any_target(self, error_message: &'static str) -> Result<Self> {
        if self.targets_state.tags_and_ms.iter().any(|(tag, _)| *tag == self.user_state.tag) {
            return Err(PrefixError::Expected(vec![ResponseBlueprint::with_content(error_message)]));
        }

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

// Battle: any,   Moment: any,   User: found,   Targets: found,   Optional Targets: any   Reserved Args: any   Optional Reserved Args: any
impl<'a, Bot: AsBot, BattleS, MomentS, const LEN: usize, OptionalTargetsS, ReservedArgsS, OptionalReservedArgsS>
    Setting<'a, Bot, BattleS, MomentS, FoundUser, FoundTargets<'a, LEN>, OptionalTargetsS, ReservedArgsS, OptionalReservedArgsS>
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

// Battle: any,   Moment: any,   User: found,   Targets: any,   Optional Targets: empty   Reserved Args: empty   Optional Reserved Args: empty
impl<'a, Bot: AsBot, BattleS, MomentS, TargetsS> Setting<'a, Bot, BattleS, MomentS, FoundUser, TargetsS, Empty, Empty, Empty> {
    pub fn fetch_reserved_args<const LEN: usize>(
        mut self,
        missing_reserved_arg_messages: [&'static str; LEN],
    ) -> Result<Setting<'a, Bot, BattleS, MomentS, FoundUser, TargetsS, Empty, FoundReservedArgs<'a, LEN>, Empty>> {
        let mut reserved_args: [Option<&'a str>; LEN] = [(); LEN].map(|_| None);

        for i in 0 .. LEN {
            match self.args.pop_front() {
                Some(arg) => reserved_args[i] = Some(arg),
                None => return Err(PrefixError::Expected(vec![ResponseBlueprint::with_content(missing_reserved_arg_messages[i])])),
            }
        }

        let reserved_args = reserved_args.map(|op| op.expect("This should never happen"));

        Ok(Setting {
            bot:                          self.bot,
            args:                         self.args,
            battle_state:                 self.battle_state,
            moment_state:                 self.moment_state,
            user_state:                   self.user_state,
            targets_state:                self.targets_state,
            optional_targets_state:       self.optional_targets_state,
            reserved_args_state:          FoundReservedArgs(reserved_args),
            optional_reserved_args_state: self.optional_reserved_args_state,
        })
    }
}

// Battle: any,   Moment: any,   User: found,   Targets: any,   Optional Targets: empty   Reserved Args: any   Optional Reserved Args: empty
impl<'a, Bot: AsBot, BattleS, MomentS, TargetsS, ReservedArgsS>
    Setting<'a, Bot, BattleS, MomentS, FoundUser, TargetsS, Empty, ReservedArgsS, Empty>
{
    pub fn fetch_optional_reserved_args<const LEN: usize>(
        mut self,
    ) -> Result<Setting<'a, Bot, BattleS, MomentS, FoundUser, TargetsS, Empty, ReservedArgsS, FoundOptionalReservedArgs<'a, LEN>>> {
        let mut reserved_args: [Option<&'a str>; LEN] = [(); LEN].map(|_| None);

        for reserved_arg in reserved_args.iter_mut() {
            if let Some(arg) = self.args.pop_front() {
                *reserved_arg = Some(arg)
            }
        }

        Ok(Setting {
            bot:                          self.bot,
            args:                         self.args,
            battle_state:                 self.battle_state,
            moment_state:                 self.moment_state,
            user_state:                   self.user_state,
            targets_state:                self.targets_state,
            optional_targets_state:       self.optional_targets_state,
            reserved_args_state:          self.reserved_args_state,
            optional_reserved_args_state: FoundOptionalReservedArgs(reserved_args),
        })
    }
}

// Battle: any,   Moment: any,   User: found,   Targets: any,   Optional Targets: found   Reserved Args: any   Optional Reserved Args: any
impl<'a, Bot: AsBot, BattleS, MomentS, TargetsS, const LEN: usize, ReservedArgsS, OptionalReservedArgsS>
    Setting<'a, Bot, BattleS, MomentS, FoundUser, TargetsS, FoundOptionalTargets<'a, LEN>, ReservedArgsS, OptionalReservedArgsS>
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

// Battle: any,   Moment: any,   User: found,   Targets: any,   Optional Targets: any   Reserved Args: any   Optional Reserved Args: any
impl<'a, Bot: AsBot, BattleS, MomentS, TargetsS, OptionalTargetsS, ReservedArgsS, OptionalReservedArgsS>
    Setting<'a, Bot, BattleS, MomentS, FoundUser, TargetsS, OptionalTargetsS, ReservedArgsS, OptionalReservedArgsS>
{
    pub fn get_user_tag(&self) -> &FixedString<u8> { &self.user_state.tag }

    pub fn get_user_mirror(&self) -> &Mirror<Id> { &self.user_state.m }
}

// Battle: any,   Moment: any,   User: any,   Targets: found,   Optional Targets: found   Reserved Args: any   Optional Reserved Args: any
impl<'a, Bot: AsBot, BattleS, MomentS, UserS, const L1: usize, const L2: usize, ReservedArgsS, OptionalReservedArgsS>
    Setting<'a, Bot, BattleS, MomentS, UserS, FoundTargets<'a, L1>, FoundOptionalTargets<'a, L2>, ReservedArgsS, OptionalReservedArgsS>
{
    pub fn unallow_any_n_any_target_repetitons<const N: usize>(self, error_message: &'static str) -> Result<Self> {
        for i in 0 .. L1 {
            let mut count = 0;
            for j in i + 1 .. L1 {
                if self.targets_state.tags_and_ms[i].0 == self.targets_state.tags_and_ms[j].0 {
                    count += 1;
                    if count >= N {
                        return Err(PrefixError::Expected(vec![ResponseBlueprint::with_content(error_message)]));
                    }
                }
            }

            for j in 0 .. L2 {
                // To optimize further should break at None case
                if let Some((target_tag, _)) = &self.optional_targets_state.tags_and_ms[j] {
                    if *target_tag == self.targets_state.tags_and_ms[i].0 {
                        count += 1;
                        if count >= N {
                            return Err(PrefixError::Expected(vec![ResponseBlueprint::with_content(error_message)]));
                        }
                    }
                }
            }
        }

        for i in 0 .. L2 {
            let mut count = 0;
            for j in i + 1 .. L2 {
                // To optimize further should break when the second is a None case
                if let (Some((tag_i, _)), Some((tag_j, _))) =
                    (&self.optional_targets_state.tags_and_ms[i], &self.optional_targets_state.tags_and_ms[j])
                {
                    if tag_i == tag_j {
                        count += 1;
                        if count >= N {
                            return Err(PrefixError::Expected(vec![ResponseBlueprint::with_content(error_message)]));
                        }
                    }
                }
            }
        }

        Ok(self)
    }
}

// Battle: any,   Moment: any,   User: any,   Targets: found,   Optional Targets: any   Reserved Args: any   Optional Reserved Args: any
impl<'a, Bot: AsBot, BattleS, MomentS, UserS, const LEN: usize, OptionalTargetsS, ReservedArgsS, OptionalReservedArgsS>
    Setting<'a, Bot, BattleS, MomentS, UserS, FoundTargets<'a, LEN>, OptionalTargetsS, ReservedArgsS, OptionalReservedArgsS>
{
    pub fn unallow_duplicate_target<const I1: usize, const I2: usize>(self, error_message: &'static str) -> Result<Self> {
        if self.targets_state.tags_and_ms[I1].0 == self.targets_state.tags_and_ms[I2].0 {
            return Err(PrefixError::Expected(vec![ResponseBlueprint::with_content(error_message)]));
        }

        Ok(self)
    }

    pub fn unallow_any_n_target_repetitons<const N: usize>(self, error_message: &'static str) -> Result<Self> {
        for i in 0 .. LEN {
            let mut count = 0;
            for j in i + 1 .. LEN {
                if self.targets_state.tags_and_ms[i].0 == self.targets_state.tags_and_ms[j].0 {
                    count += 1;
                    if count >= N {
                        return Err(PrefixError::Expected(vec![ResponseBlueprint::with_content(error_message)]));
                    }
                }
            }
        }

        Ok(self)
    }

    pub fn get_target_tags_and_ms(&self) -> &[(&'a str, Mirror<Id>); LEN] { &self.targets_state.tags_and_ms }

    pub fn get_target_tags(&self) -> [&'a str; LEN] {
        let mut tags: [&'a str; LEN] = unsafe { mem::zeroed() };
        for (i, tags_and_ms) in self.targets_state.tags_and_ms.iter().enumerate() {
            tags[i] = tags_and_ms.0;
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

// Battle: any,   Moment: any,   User: any,   Targets: any,   Optional Targets: found   Reserved Args: any   Optional Reserved Args: any
impl<'a, Bot: AsBot, BattleS, MomentS, UserS, TargetsS, const LEN: usize, ReservedArgsS, OptionalReservedArgsS>
    Setting<'a, Bot, BattleS, MomentS, UserS, TargetsS, FoundOptionalTargets<'a, LEN>, ReservedArgsS, OptionalReservedArgsS>
{
    pub fn unallow_duplicate_optional_target<const I1: usize, const I2: usize>(self, error_message: &'static str) -> Result<Self> {
        if let (Some((tag_i, _)), Some((tag_j, _))) =
            (&self.optional_targets_state.tags_and_ms[I1], &self.optional_targets_state.tags_and_ms[I2])
        {
            if tag_i == tag_j {
                return Err(PrefixError::Expected(vec![ResponseBlueprint::with_content(error_message)]));
            }
        }

        Ok(self)
    }

    pub fn unallow_any_n_optional_target_repetitons<const N: usize>(self, error_message: &'static str) -> Result<Self> {
        for i in 0 .. LEN {
            let mut count = 0;
            for j in i + 1 .. LEN {
                if let (Some((tag_i, _)), Some((tag_j, _))) =
                    (&self.optional_targets_state.tags_and_ms[i], &self.optional_targets_state.tags_and_ms[j])
                {
                    if tag_i == tag_j {
                        count += 1;
                        if count >= N {
                            return Err(PrefixError::Expected(vec![ResponseBlueprint::with_content(error_message)]));
                        }
                    }
                }
            }
        }

        Ok(self)
    }

    pub fn get_optional_target_tags_and_ms(&self) -> &[Option<(&'a str, Mirror<Id>)>; LEN] { &self.optional_targets_state.tags_and_ms }

    pub fn get_optional_target_tags(&self) -> [Option<&'a str>; LEN] {
        let mut tags: [Option<&'a str>; LEN] = unsafe { mem::zeroed() };
        for (i, tags_and_ms) in self.optional_targets_state.tags_and_ms.iter().enumerate() {
            tags[i] = tags_and_ms.as_ref().map(|(tag, _)| *tag);
        }
        tags
    }

    pub fn get_optional_target_ms(&self) -> [Option<&Mirror<Id>>; LEN] {
        let mut ms: [Option<&Mirror<Id>>; LEN] = unsafe { mem::zeroed() };
        for (i, tags_and_ms) in self.optional_targets_state.tags_and_ms.iter().enumerate() {
            ms[i] = tags_and_ms.as_ref().map(|(_, ms)| ms);
        }
        ms
    }
}

// Battle: any,   Moment: any,   User: any,   Targets: any,   Optional Targets: any   Reserved Args: found   Optional Reserved Args: any
impl<'a, Bot: AsBot, BattleS, MomentS, UserS, TargetsS, OptionalTargetsS, const LEN: usize, OptionalReservedArgsS>
    Setting<'a, Bot, BattleS, MomentS, UserS, TargetsS, OptionalTargetsS, FoundReservedArgs<'a, LEN>, OptionalReservedArgsS>
{
    pub fn get_reserved_args(&self) -> &[&'a str; LEN] { &self.reserved_args_state.0 }
}

// Battle: any,   Moment: any,   User: any,   Targets: any,   Optional Targets: any   Reserved Args: any   Optional Reserved Args: found
impl<'a, Bot: AsBot, BattleS, MomentS, UserS, TargetsS, OptionalTargetsS, ReservedArgsS, const LEN: usize>
    Setting<'a, Bot, BattleS, MomentS, UserS, TargetsS, OptionalTargetsS, ReservedArgsS, FoundOptionalReservedArgs<'a, LEN>>
{
    pub fn get_optional_reserved_args(&self) -> &[Option<&'a str>; LEN] { &self.optional_reserved_args_state.0 }
}
