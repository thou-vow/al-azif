#![allow(clippy::type_complexity)]

use crate::_prelude::*;

pub struct Empty;
pub struct FBattle {
    tag: FixedString<u8>,
    m:   Mirror<Battle>,
}
pub struct FPrimaryMoment(PrimaryMoment);
pub struct FReactiveMoment(ReactiveMoment);
pub struct FUser {
    tag: FixedString<u8>,
    m:   Mirror<Id>,
}
pub struct FRequiredTargets<'a, const LEN: usize> {
    tags_and_ms: [(&'a str, Mirror<Id>); LEN],
}
pub struct FOptionalTargets<'a, const LEN: usize> {
    tags_and_ms: [Option<(&'a str, Mirror<Id>)>; LEN],
}
pub struct FRequiredReservedArgs<'a, const LEN: usize>([&'a str; LEN]);
pub struct FOptionalReservedArgs<'a, const LEN: usize>([Option<&'a str>; LEN]);

pub struct Setting<
    'a,
    Bot: AsBot,
    BattleS = Empty,
    MomentS = Empty,
    UserS = Empty,
    RequiredTargetsS = Empty,
    OptionalTargetsS = Empty,
    ReservedArgsS = Empty,
    OptionalReservedArgsS = Empty,
> {
    pub bot:                      &'a Bot,
    args:                         VecDeque<&'a str>,
    battle_state:                 BattleS,
    moment_state:                 MomentS,
    user_state:                   UserS,
    required_targets_state:       RequiredTargetsS,
    optional_targets_state:       OptionalTargetsS,
    reserved_args_state:          ReservedArgsS,
    optional_reserved_args_state: OptionalReservedArgsS,
}

// Battle: empty   Moment: empty   User: empty   Targets: empty   Optional Targets: empty   Reserved Args: empty   Optional Reserved Args: empty
impl<'a, Bot: AsBot> Setting<'a, Bot, Empty, Empty, Empty, Empty, Empty, Empty> {
    pub fn new(bot: &'a Bot, args: VecDeque<&'a str>) -> Self {
        Self {
            bot,
            args,
            battle_state: Empty,
            moment_state: Empty,
            user_state: Empty,
            required_targets_state: Empty,
            optional_targets_state: Empty,
            reserved_args_state: Empty,
            optional_reserved_args_state: Empty,
        }
    }

    pub async fn fetch_battle(self, tag: String) -> Result<Setting<'a, Bot, FBattle, Empty, Empty, Empty, Empty, Empty, Empty>> {
        let tag = FixedString::from_string_trunc(tag);

        let Ok(m) = Mirror::<Battle>::get(self.bot, &tag).await else {
            return Err(PrefixError::Anticipated(ErrorResponse::send(vec![ResponseBlueprint::with_content(lang_diff!(self.bot,
                en: "No battle is currently happening in this channel.",
                pt: "Não há uma batalha acontecendo neste canal."
            ))])));
        };

        Ok(Setting {
            bot:                          self.bot,
            args:                         self.args,
            battle_state:                 FBattle { tag, m },
            moment_state:                 self.moment_state,
            user_state:                   self.user_state,
            required_targets_state:       self.required_targets_state,
            optional_targets_state:       self.optional_targets_state,
            reserved_args_state:          self.reserved_args_state,
            optional_reserved_args_state: self.optional_reserved_args_state,
        })
    }
}

// Battle: found  Moment: empty  User: empty  Targets: empty  Optional Targets: empty  Reserved Args: empty  Optional Reserved Args: empty
impl<'a, Bot: AsBot> Setting<'a, Bot, FBattle, Empty, Empty, Empty, Empty, Empty, Empty> {
    pub async fn require_primary_moment(self) -> Result<Setting<'a, Bot, FBattle, FPrimaryMoment, Empty, Empty, Empty, Empty, Empty>> {
        let battle = self.battle_state.m.read().await;

        let Moment::Primary(primary) = &battle.current_moment else {
            return Err(PrefixError::Anticipated(ErrorResponse::send(vec![ResponseBlueprint::with_content(lang_diff!(self.bot,
                en: "You can't use this command right now.",
                pt: "Você não pode usar este comando agora."
            ))])));
        };
        let primary = primary.to_owned();

        battle.unread();

        Ok(Setting {
            bot:                          self.bot,
            args:                         self.args,
            battle_state:                 self.battle_state,
            moment_state:                 FPrimaryMoment(primary),
            user_state:                   self.user_state,
            required_targets_state:       self.required_targets_state,
            optional_targets_state:       self.optional_targets_state,
            reserved_args_state:          self.reserved_args_state,
            optional_reserved_args_state: self.optional_reserved_args_state,
        })
    }

    pub async fn require_reactive_moment(self) -> Result<Setting<'a, Bot, FBattle, FReactiveMoment, Empty, Empty, Empty, Empty, Empty>> {
        let battle = self.battle_state.m.read().await;

        let Moment::Reactive(reactive) = &battle.current_moment else {
            return Err(PrefixError::Anticipated(ErrorResponse::send(vec![ResponseBlueprint::with_content(lang_diff!(self.bot,
                en: "You can't use this command right now.",
                pt: "Você não pode usar este comando agora."
            ))])));
        };
        let reactive = reactive.to_owned();

        battle.unread();

        Ok(Setting {
            bot:                          self.bot,
            args:                         self.args,
            battle_state:                 self.battle_state,
            moment_state:                 FReactiveMoment(reactive.clone()),
            user_state:                   self.user_state,
            required_targets_state:       self.required_targets_state,
            optional_targets_state:       self.optional_targets_state,
            reserved_args_state:          self.reserved_args_state,
            optional_reserved_args_state: self.optional_reserved_args_state,
        })
    }
}

// Battle: found,   Moment: primary,   User: empty,   Targets: empty,   Optional Targets: empty   Reserved Args: empty   Optional Reserved Args: empty
impl<'a, Bot: AsBot> Setting<'a, Bot, FBattle, FPrimaryMoment, Empty, Empty, Empty, Empty, Empty> {
    pub async fn fetch_user(mut self) -> Result<Setting<'a, Bot, FBattle, FPrimaryMoment, FUser, Empty, Empty, Empty, Empty>> {
        let tag = match self.args.pop_front() {
            Some(".") => self.moment_state.0.moment_owner_tag.clone(),
            Some(tag) => {
                let tag = FixedString::from_str_trunc(tag);

                if self.moment_state.0.moment_owner_tag != tag {
                    return Err(PrefixError::Anticipated(ErrorResponse::send(vec![ResponseBlueprint::with_content(lang_diff!(self.bot,
                        en: "You can't use this action right now.",
                        pt: "Você não pode usar esta ação agora."
                    ))])));
                }

                tag
            },
            None => {
                return Err(PrefixError::Anticipated(ErrorResponse::send(vec![ResponseBlueprint::with_content(lang_diff!(self.bot,
                    en: "You must specify a user.",
                    pt: "Você deve especificar um usuário."
                ))])))
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
            user_state:                   FUser { tag, m },
            required_targets_state:       self.required_targets_state,
            optional_targets_state:       self.optional_targets_state,
            reserved_args_state:          self.reserved_args_state,
            optional_reserved_args_state: self.optional_reserved_args_state,
        })
    }
}

// Battle: found,   Moment: reactive,   User: empty,   Targets: empty,   Optional Targets: empty   Reserved Args: empty   Optional Reserved Args: empty
impl<'a, Bot: AsBot> Setting<'a, Bot, FBattle, FReactiveMoment, Empty, Empty, Empty, Empty, Empty> {
    pub async fn fetch_user(mut self) -> Result<Setting<'a, Bot, FBattle, FReactiveMoment, FUser, Empty, Empty, Empty, Empty>> {
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

                if !self.battle_state.m.read().await.opponents.iter().any(|opponent| opponent.tag == tag) {
                    return Err(PrefixError::Anticipated(ErrorResponse::send(vec![ResponseBlueprint::with_content(lang_diff!(self.bot,
                        en: "You are not in the battle.",
                        pt: "Você não está na batalha."
                    ))])));
                }

                tag
            },
            None => {
                return Err(PrefixError::Anticipated(ErrorResponse::send(vec![ResponseBlueprint::with_content(lang_diff!(self.bot,
                    en: "You must specify a user.",
                    pt: "Você deve especificar um usuário."
                ))])))
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
            user_state:                   FUser { tag, m },
            required_targets_state:       self.required_targets_state,
            optional_targets_state:       self.optional_targets_state,
            reserved_args_state:          self.reserved_args_state,
            optional_reserved_args_state: self.optional_reserved_args_state,
        })
    }
}

// Battle: found,   Moment: any,   User: found,   Targets: empty,   Optional Targets: empty   Reserved Args: any   Optional Reserved Args: empty
impl<'a, Bot: AsBot, MomentS, ReservedArgsS> Setting<'a, Bot, FBattle, MomentS, FUser, Empty, Empty, ReservedArgsS, Empty> {
    pub async fn fetch_targets<const LEN: usize>(
        mut self,
        missing_target_arg_messages: [&'static str; LEN],
    ) -> Result<Setting<'a, Bot, FBattle, MomentS, FUser, FRequiredTargets<'a, LEN>, Empty, ReservedArgsS, Empty>> {
        let mut tags_and_ms: [Option<(&'a str, Mirror<Id>)>; LEN] = [const { None }; LEN];

        let battle = self.battle_state.m.read().await;

        for i in 0 .. LEN {
            match self.args.pop_front() {
                Some(tag) => {
                    let Ok(m) = Mirror::<Id>::get(self.bot, tag).await else {
                        return Err(PrefixError::Anticipated(ErrorResponse::send(vec![ResponseBlueprint::with_content(lang_diff!(self.bot,
                            en: f!("The Id `{tag}` was not found."),
                            pt: f!("O Id `{tag}` não foi encontrado.")
                        ))])));
                    };

                    if !battle.opponents.iter().any(|opponent| opponent.tag == tag) {
                        return Err(PrefixError::Anticipated(ErrorResponse::send(vec![ResponseBlueprint::with_content(lang_diff!(self.bot,
                            en: f!("The Id `{tag}` is not in the battle."),
                            pt: f!("O Id `{tag}` não está na batalha.")
                        ))])));
                    }

                    tags_and_ms[i] = Some((tag, m));
                },
                None => {
                    return Err(PrefixError::Anticipated(ErrorResponse::send(vec![ResponseBlueprint::with_content(missing_target_arg_messages[i])])));
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
            required_targets_state:       FRequiredTargets { tags_and_ms },
            optional_targets_state:       self.optional_targets_state,
            reserved_args_state:          self.reserved_args_state,
            optional_reserved_args_state: self.optional_reserved_args_state,
        })
    }
}

// Battle: found,   Moment: any,   User: found,   Targets: any,   Optional Targets: empty   Reserved Args: any   Optional Reserved Args: empty
impl<'a, Bot: AsBot, MomentS, RequiredTargetsS, ReservedArgsS>
    Setting<'a, Bot, FBattle, MomentS, FUser, RequiredTargetsS, Empty, ReservedArgsS, Empty>
{
    pub async fn fetch_optional_targets<const LEN: usize>(
        mut self,
    ) -> Result<Setting<'a, Bot, FBattle, MomentS, FUser, RequiredTargetsS, FOptionalTargets<'a, LEN>, ReservedArgsS>> {
        let mut tags_and_ms: [Option<(&'a str, Mirror<Id>)>; LEN] = [const { None }; LEN];

        let battle = self.battle_state.m.read().await;

        for tag_and_m in tags_and_ms.iter_mut() {
            if let Some(tag) = self.args.pop_front() {
                let Ok(m) = Mirror::<Id>::get(self.bot, tag).await else {
                    return Err(PrefixError::Anticipated(ErrorResponse::send(vec![ResponseBlueprint::with_content(lang_diff!(self.bot,
                        en: f!("The Id `{tag}` was not found."),
                        pt: f!("O Id `{tag}` não foi encontrado.")
                    ))])));
                };

                if !battle.opponents.iter().any(|opponent| opponent.tag == tag) {
                    return Err(PrefixError::Anticipated(ErrorResponse::send(vec![ResponseBlueprint::with_content(lang_diff!(self.bot,
                        en: f!("The Id `{tag}` is not in the battle."),
                        pt: f!("O Id `{tag}` não está na batalha.")
                    ))])));
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
            required_targets_state:       self.required_targets_state,
            optional_targets_state:       FOptionalTargets { tags_and_ms },
            reserved_args_state:          self.reserved_args_state,
            optional_reserved_args_state: self.optional_reserved_args_state,
        })
    }
}

// Battle: found,   Moment: any,   User: any,   Targets: any,   Optional Targets: any   Reserved Args: any   Optional Reserved Args: any
impl<'a, Bot: AsBot, MomentS, UserS, RequiredTargetsS, OptionalTargetsS, ReservedArgsS, OptionalReservedArgsS>
    Setting<'a, Bot, FBattle, MomentS, UserS, RequiredTargetsS, OptionalTargetsS, ReservedArgsS, OptionalReservedArgsS>
{
    pub fn get_battle_tag(&self) -> &FixedString<u8> { &self.battle_state.tag }

    pub fn get_battle_mirror(&self) -> &Mirror<Battle> { &self.battle_state.m }
}

// Battle: any,   Moment: reactive,   User: any,   Targets: any,   Optional Targets: any   Reserved Args: any   Optional Reserved Args: any
impl<'a, Bot: AsBot, BattleS, UserS, RequiredTargetsS, OptionalTargetsS, ReservedArgsS, OptionalReservedArgsS>
    Setting<'a, Bot, BattleS, FReactiveMoment, UserS, RequiredTargetsS, OptionalTargetsS, ReservedArgsS, OptionalReservedArgsS>
{
    pub fn get_primary_moment_owner_tag(&self) -> &FixedString<u8> { &self.moment_state.0.primary_moment_owner_tag }

    pub fn get_primary_action_tag(&self) -> &FixedString<u8> { &self.moment_state.0.primary_action_tag }
}

// Battle: any,   Moment: any,   User: found,   Targets: found,   Optional Targets: found   Reserved Args: any   Optional Reserved Args: any
impl<'a, Bot: AsBot, BattleS, MomentS, const L1: usize, const L2: usize, ReservedArgsS, OptionalReservedArgsS>
    Setting<'a, Bot, BattleS, MomentS, FUser, FRequiredTargets<'a, L1>, FOptionalTargets<'a, L2>, ReservedArgsS, OptionalReservedArgsS>
{
    /// If any of the targets (either required or optional) corresponds to the user, then return an error message
    pub fn unallow_any_self_any_target(self, error_message: &'static str) -> Result<Self> {
        if self.required_targets_state.tags_and_ms.iter().any(|(tag, _)| *tag == self.user_state.tag) {
            return Err(PrefixError::Anticipated(ErrorResponse::send(vec![ResponseBlueprint::with_content(error_message)])));
        }

        if self
            .optional_targets_state
            .tags_and_ms
            .iter()
            .filter_map(|tag| if let Some((target_tag, _)) = tag { Some(target_tag) } else { None })
            .any(|tag| *tag == self.user_state.tag)
        {
            return Err(PrefixError::Anticipated(ErrorResponse::send(vec![ResponseBlueprint::with_content(error_message)])));
        }

        Ok(self)
    }
}

// Battle: any,   Moment: any,   User: found,   Targets: found,   Optional Targets: any   Reserved Args: any   Optional Reserved Args: any
impl<'a, Bot: AsBot, BattleS, MomentS, const LEN: usize, OptionalTargetsS, ReservedArgsS, OptionalReservedArgsS>
    Setting<'a, Bot, BattleS, MomentS, FUser, FRequiredTargets<'a, LEN>, OptionalTargetsS, ReservedArgsS, OptionalReservedArgsS>
{
    /// If the required target of index INDEX corresponds to the user, then return an error message
    pub fn unallow_self_required_target<const INDEX: usize>(self, error_message: &'static str) -> Result<Self> {
        if self.required_targets_state.tags_and_ms[INDEX].0 == self.user_state.tag {
            return Err(PrefixError::Anticipated(ErrorResponse::send(vec![ResponseBlueprint::with_content(error_message)])));
        }

        Ok(self)
    }

    /// If any of the required targets corresponds to the user, then return an error message
    pub fn unallow_any_self_required_target(self, error_message: &'static str) -> Result<Self> {
        if self.required_targets_state.tags_and_ms.iter().any(|(tag, _)| *tag == self.user_state.tag) {
            return Err(PrefixError::Anticipated(ErrorResponse::send(vec![ResponseBlueprint::with_content(error_message)])));
        }

        Ok(self)
    }
}

// Battle: any,   Moment: any,   User: found,   Targets: any,   Optional Targets: empty   Reserved Args: empty   Optional Reserved Args: empty
impl<'a, Bot: AsBot, BattleS, MomentS, RequiredTargetsS> Setting<'a, Bot, BattleS, MomentS, FUser, RequiredTargetsS, Empty, Empty, Empty> {
    pub fn fetch_reserved_args<const LEN: usize>(
        mut self,
        missing_reserved_arg_messages: [&'static str; LEN],
    ) -> Result<Setting<'a, Bot, BattleS, MomentS, FUser, RequiredTargetsS, Empty, FRequiredReservedArgs<'a, LEN>, Empty>> {
        let mut reserved_args: [Option<&'a str>; LEN] = [(); LEN].map(|_| None);

        for i in 0 .. LEN {
            match self.args.pop_front() {
                Some(arg) => reserved_args[i] = Some(arg),
                None => return Err(PrefixError::Anticipated(ErrorResponse::send(vec![ResponseBlueprint::with_content(missing_reserved_arg_messages[i])]))),
            }
        }

        let reserved_args = reserved_args.map(|op| op.expect("This should never happen"));

        Ok(Setting {
            bot:                          self.bot,
            args:                         self.args,
            battle_state:                 self.battle_state,
            moment_state:                 self.moment_state,
            user_state:                   self.user_state,
            required_targets_state:       self.required_targets_state,
            optional_targets_state:       self.optional_targets_state,
            reserved_args_state:          FRequiredReservedArgs(reserved_args),
            optional_reserved_args_state: self.optional_reserved_args_state,
        })
    }
}

// Battle: any,   Moment: any,   User: found,   Targets: any,   Optional Targets: empty   Reserved Args: any   Optional Reserved Args: empty
impl<'a, Bot: AsBot, BattleS, MomentS, RequiredTargetsS, ReservedArgsS>
    Setting<'a, Bot, BattleS, MomentS, FUser, RequiredTargetsS, Empty, ReservedArgsS, Empty>
{
    pub fn fetch_optional_reserved_args<const LEN: usize>(
        mut self,
    ) -> Result<Setting<'a, Bot, BattleS, MomentS, FUser, RequiredTargetsS, Empty, ReservedArgsS, FOptionalReservedArgs<'a, LEN>>> {
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
            required_targets_state:       self.required_targets_state,
            optional_targets_state:       self.optional_targets_state,
            reserved_args_state:          self.reserved_args_state,
            optional_reserved_args_state: FOptionalReservedArgs(reserved_args),
        })
    }
}

// Battle: any,   Moment: any,   User: found,   Targets: any,   Optional Targets: found   Reserved Args: any   Optional Reserved Args: any
impl<'a, Bot: AsBot, BattleS, MomentS, RequiredTargetsS, const LEN: usize, ReservedArgsS, OptionalReservedArgsS>
    Setting<'a, Bot, BattleS, MomentS, FUser, RequiredTargetsS, FOptionalTargets<'a, LEN>, ReservedArgsS, OptionalReservedArgsS>
{
    /// If the optional target of index INDEX corresponds to the user, then return an error message
    pub fn unallow_self_optional_target<const INDEX: usize>(self, error_message: &'static str) -> Result<Self> {
        if let Some((target_tag, _)) = &self.optional_targets_state.tags_and_ms[INDEX] {
            if *target_tag == self.user_state.tag {
                return Err(PrefixError::Anticipated(ErrorResponse::send(vec![ResponseBlueprint::with_content(error_message)])));
            }
        }

        Ok(self)
    }

    /// If any of the optional targets corresponds to the user, then return an error message
    pub fn unallow_any_self_optional_target(self, error_message: &'static str) -> Result<Self> {
        if self
            .optional_targets_state
            .tags_and_ms
            .iter()
            .filter_map(|tag| if let Some((target_tag, _)) = tag { Some(target_tag) } else { None })
            .any(|tag| *tag == self.user_state.tag)
        {
            return Err(PrefixError::Anticipated(ErrorResponse::send(vec![ResponseBlueprint::with_content(error_message)])));
        }

        Ok(self)
    }
}

// Battle: any,   Moment: any,   User: found,   Targets: any,   Optional Targets: any   Reserved Args: any   Optional Reserved Args: any
impl<'a, Bot: AsBot, BattleS, MomentS, RequiredTargetsS, OptionalTargetsS, ReservedArgsS, OptionalReservedArgsS>
    Setting<'a, Bot, BattleS, MomentS, FUser, RequiredTargetsS, OptionalTargetsS, ReservedArgsS, OptionalReservedArgsS>
{
    pub fn get_user_tag(&self) -> &FixedString<u8> { &self.user_state.tag }

    pub fn get_user_mirror(&self) -> &Mirror<Id> { &self.user_state.m }
}

// Battle: any,   Moment: any,   User: any,   Targets: found,   Optional Targets: found   Reserved Args: any   Optional Reserved Args: any
impl<'a, Bot: AsBot, BattleS, MomentS, UserS, const L1: usize, const L2: usize, ReservedArgsS, OptionalReservedArgsS>
    Setting<'a, Bot, BattleS, MomentS, UserS, FRequiredTargets<'a, L1>, FOptionalTargets<'a, L2>, ReservedArgsS, OptionalReservedArgsS>
{
    /// If there is at least 1 repetitions over all targets (either required and optional), return an error message
    pub fn unallow_duplicate_target(self, error_message: &'static str) -> Result<Self> {
        for i in 0 .. L1 {
            for j in i + 1 .. L1 {
                if self.required_targets_state.tags_and_ms[i].0 == self.required_targets_state.tags_and_ms[j].0 {
                    return Err(PrefixError::Anticipated(ErrorResponse::send(vec![ResponseBlueprint::with_content(error_message)])));
                }
            }
            for j in 0 .. L2 {
                if let Some((tag_j, _)) = &self.optional_targets_state.tags_and_ms[j] {
                    if *tag_j == self.required_targets_state.tags_and_ms[i].0 {
                        return Err(PrefixError::Anticipated(ErrorResponse::send(vec![ResponseBlueprint::with_content(error_message)])));
                    }
                } else {
                    break;
                }
            }
        }

        for i in 0 .. L2 {
            for j in i + 1 .. L2 {
                if let (Some((tag_i, _)), Some((tag_j, _))) =
                    (&self.optional_targets_state.tags_and_ms[i], &self.optional_targets_state.tags_and_ms[j])
                {
                    if tag_i == tag_j {
                        return Err(PrefixError::Anticipated(ErrorResponse::send(vec![ResponseBlueprint::with_content(error_message)])));
                    }
                } else {
                    break;
                }
            }
        }

        Ok(self)
    }

    /// If there is at least N repetitions over all targets (either required and optional), return an error message
    pub fn unallow_any_n_target_repetitons<const N: usize>(self, error_message: &'static str) -> Result<Self> {
        for i in 0 .. L1 {
            let mut count = 0;
            for j in i + 1 .. L1 {
                if self.required_targets_state.tags_and_ms[i].0 == self.required_targets_state.tags_and_ms[j].0 {
                    count += 1;
                    if count >= N {
                        return Err(PrefixError::Anticipated(ErrorResponse::send(vec![ResponseBlueprint::with_content(error_message)])));
                    }
                }
            }

            for j in 0 .. L2 {
                if let Some((tag_j, _)) = &self.optional_targets_state.tags_and_ms[j] {
                    if *tag_j == self.required_targets_state.tags_and_ms[i].0 {
                        count += 1;
                        if count >= N {
                            return Err(PrefixError::Anticipated(ErrorResponse::send(vec![ResponseBlueprint::with_content(error_message)])));
                        }
                    }
                } else {
                    break;
                }
            }
        }

        for i in 0 .. L2 {
            let mut count = 0;
            for j in i + 1 .. L2 {
                if let (Some((tag_i, _)), Some((tag_j, _))) =
                    (&self.optional_targets_state.tags_and_ms[i], &self.optional_targets_state.tags_and_ms[j])
                {
                    if tag_i == tag_j {
                        count += 1;
                        if count >= N {
                            return Err(PrefixError::Anticipated(ErrorResponse::send(vec![ResponseBlueprint::with_content(error_message)])));
                        }
                    }
                } else {
                    break;
                }
            }
        }

        Ok(self)
    }
}

// Battle: any,   Moment: any,   User: any,   Targets: found,   Optional Targets: any   Reserved Args: any   Optional Reserved Args: any
impl<'a, Bot: AsBot, BattleS, MomentS, UserS, const LEN: usize, OptionalTargetsS, ReservedArgsS, OptionalReservedArgsS>
    Setting<'a, Bot, BattleS, MomentS, UserS, FRequiredTargets<'a, LEN>, OptionalTargetsS, ReservedArgsS, OptionalReservedArgsS>
{
    /// If these two required targets are the same (when both exists), return an error message
    pub fn unallow_duplicate_required_target<const I1: usize, const I2: usize>(self, error_message: &'static str) -> Result<Self> {
        if self.required_targets_state.tags_and_ms[I1].0 == self.required_targets_state.tags_and_ms[I2].0 {
            return Err(PrefixError::Anticipated(ErrorResponse::send(vec![ResponseBlueprint::with_content(error_message)])));
        }

        Ok(self)
    }

    /// If there is at least N repetitions over all required targets, return an error message
    pub fn unallow_any_n_required_target_repetitons<const N: usize>(self, error_message: &'static str) -> Result<Self> {
        for i in 0 .. LEN {
            let mut count = 0;
            for j in i + 1 .. LEN {
                if self.required_targets_state.tags_and_ms[i].0 == self.required_targets_state.tags_and_ms[j].0 {
                    count += 1;
                    if count >= N {
                        return Err(PrefixError::Anticipated(ErrorResponse::send(vec![ResponseBlueprint::with_content(error_message)])));
                    }
                }
            }
        }

        Ok(self)
    }

    pub fn get_required_target_tags_and_ms(&self) -> &[(&'a str, Mirror<Id>); LEN] { &self.required_targets_state.tags_and_ms }

    pub fn get_required_target_tags(&self) -> [&'a str; LEN] {
        let mut tags: [&'a str; LEN] = unsafe { mem::zeroed() };
        for (i, tags_and_ms) in self.required_targets_state.tags_and_ms.iter().enumerate() {
            tags[i] = tags_and_ms.0;
        }
        tags
    }

    pub fn get_required_target_ms(&self) -> [&Mirror<Id>; LEN] {
        let mut ms: [&Mirror<Id>; LEN] = unsafe { mem::zeroed() };
        for (i, tags_and_ms) in self.required_targets_state.tags_and_ms.iter().enumerate() {
            ms[i] = &tags_and_ms.1;
        }
        ms
    }
}

// Battle: any,   Moment: any,   User: any,   Targets: any,   Optional Targets: found   Reserved Args: any   Optional Reserved Args: any
impl<'a, Bot: AsBot, BattleS, MomentS, UserS, RequiredTargetsS, const LEN: usize, ReservedArgsS, OptionalReservedArgsS>
    Setting<'a, Bot, BattleS, MomentS, UserS, RequiredTargetsS, FOptionalTargets<'a, LEN>, ReservedArgsS, OptionalReservedArgsS>
{
    /// If these two optional targets are the same (when both exists), return an error message
    pub fn unallow_duplicate_optional_target<const I1: usize, const I2: usize>(self, error_message: &'static str) -> Result<Self> {
        if let (Some((tag_i, _)), Some((tag_j, _))) =
            (&self.optional_targets_state.tags_and_ms[I1], &self.optional_targets_state.tags_and_ms[I2])
        {
            if tag_i == tag_j {
                return Err(PrefixError::Anticipated(ErrorResponse::send(vec![ResponseBlueprint::with_content(error_message)])));
            }
        }

        Ok(self)
    }

    /// If there is at least N repetitions over all optional targets, return an error message
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
                            return Err(PrefixError::Anticipated(ErrorResponse::send(vec![ResponseBlueprint::with_content(error_message)])));
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

// Battle: any   Moment: any   User: any   Targets: any   Optional Targets: any   Reserved Args: found   Optional Reserved Args: any
impl<'a, Bot: AsBot, BattleS, MomentS, UserS, RequiredTargetsS, OptionalTargetsS, const LEN: usize, OptionalReservedArgsS>
    Setting<'a, Bot, BattleS, MomentS, UserS, RequiredTargetsS, OptionalTargetsS, FRequiredReservedArgs<'a, LEN>, OptionalReservedArgsS>
{
    pub fn get_reserved_args(&self) -> &[&'a str; LEN] { &self.reserved_args_state.0 }
}

// Battle: any  Moment: any  User: any  Targets: any  Optional Targets: any  Reserved Args: any  Optional Reserved Args: found
impl<'a, Bot: AsBot, BattleS, MomentS, UserS, RequiredTargetsS, OptionalTargetsS, ReservedArgsS, const LEN: usize>
    Setting<'a, Bot, BattleS, MomentS, UserS, RequiredTargetsS, OptionalTargetsS, ReservedArgsS, FOptionalReservedArgs<'a, LEN>>
{
    pub fn get_optional_reserved_args(&self) -> &[Option<&'a str>; LEN] { &self.optional_reserved_args_state.0 }
}
