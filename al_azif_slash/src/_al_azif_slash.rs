pub mod commands {
    pub mod battle;
    pub mod exp;
    pub mod help;
    pub mod id;
    pub mod ping;
}
pub mod _prelude;

use _prelude::*;
use commands::*;

pub enum SlashCommand<'a> {
    Battle(battle::SubCommand<'a>),
    Exp(exp::SubCommand<'a>),
    Help,
    Id(id::SubCommand<'a>),
    Ping,
}
impl<'a> SlashCommand<'a>{
    pub fn from_name_and_args(name: &str, args: &'a [ResolvedOption<'_>]) -> Option<Self> {
        match name {
            battle::NAME => Some(Self::Battle(battle::SubCommand::from_args(args)?)),
            exp::NAME => Some(Self::Exp(exp::SubCommand::from_args(args)?)),
            help::NAME => Some(Self::Help),
            id::NAME => Some(Self::Id(id::SubCommand::from_args(args)?)),
            ping::NAME => Some(Self::Ping),
            _ => None,
        }
    }
    pub fn all() -> Vec<Self> {
        let mut all = Vec::new();

        for sub in battle::SubCommand::all() {
            all.push(Self::Battle(sub));
        }
        for sub in exp::SubCommand::all() {
            all.push(Self::Exp(sub));
        }
        all.push(Self::Help);
        for sub in id::SubCommand::all() {
            all.push(Self::Id(sub));
        }
        all.push(Self::Ping);

        all
    }
    pub fn all_localized_order() -> Vec<Self> {
        let mut all = Vec::new();

        for sub in battle::SubCommand::all_localized_order() {
            all.push(Self::Battle(sub));
        }
        for sub in exp::SubCommand::all_localized_order() {
            all.push(Self::Exp(sub));
        }
        all.push(Self::Help);
        for sub in id::SubCommand::all_localized_order() {
            all.push(Self::Id(sub));
        }

        all.push(Self::Ping);
        all.sort_by(|a, b| a.get_name_localized().cmp(b.get_name_localized()));
        all
    }
    pub fn registers() -> [CreateCommand<'static>; 4]{
        [
            battle::register(),
            exp::register(),
            help::register(),
            id::register(),
        ]
    }
}
impl<'a> SlashCommand<'a> {
    pub fn get_name(&self) -> &'static str {
        match self {
            Self::Battle(_) => battle::NAME,
            Self::Exp(_) => exp::NAME,
            Self::Help => help::NAME,
            Self::Id(_) => id::NAME,
            Self::Ping => ping::NAME,
        }
    }
    pub fn get_description(&self) -> &'static str {
        match self {
            Self::Battle(_) => battle::DESCRIPTION,
            Self::Exp(_) => exp::DESCRIPTION,
            Self::Help => help::DESCRIPTION,
            Self::Id(_) => id::DESCRIPTION,
            Self::Ping => ping::DESCRIPTION,
        }
    }
    pub fn get_name_localized(&self) -> &'static str {
        match self {
            Self::Battle(_) => battle::NAME_LOCALIZED,
            Self::Exp(_) => exp::NAME_LOCALIZED,
            Self::Help => help::NAME_LOCALIZED,
            Self::Id(_) => id::NAME_LOCALIZED,
            Self::Ping => ping::NAME_LOCALIZED,
        }
    }
    pub fn get_description_localized(&self) -> &'static str {
        match self {
            Self::Battle(_) => battle::DESCRIPTION_LOCALIZED,
            Self::Exp(_) => exp::DESCRIPTION_LOCALIZED,
            Self::Help => help::DESCRIPTION_LOCALIZED,
            Self::Id(_) => id::DESCRIPTION_LOCALIZED,
            Self::Ping => ping::DESCRIPTION_LOCALIZED,
        }
    }
    pub async fn run(
        &self,
        bot: &impl AsBot,
        ctx: &Context,
        slash: &CommandInteraction,
    ) -> Result<Responses<'a>> {
        match self {
            Self::Battle(sub) => sub.run(bot, slash).await,
            Self::Exp(sub) => sub.run(bot).await,
            Self::Help => help::run().await,
            Self::Id(sub) => sub.run(bot).await,
            Self::Ping => ping::run(ctx, slash).await,
        }
    }
}
