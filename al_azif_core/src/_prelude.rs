pub use crate::{
    battle::{advance, Battle, Moment, Opponent},
    bot::{lang_diff, AsBot, InMemoryDatabase, Lang},
    constants::*,
    database::{self, Error as DatabaseError, Reflective},
    effect::Effect,
    id::{Age, Gender, Id},
    mirror::{InMemoryStore, Mirror, ReadMirror, WriteMirror},
    player::Player,
    response::{self, Blueprints, Response, ResponseBlueprint, Responses},
    utils::{
        self,
        fmt::{join_with_and, mark_thousands, mark_thousands_and_show_sign},
        roll::{RollExpression, RollSummary},
    },
    Error as CoreError,
};
pub use const_format::formatcp as fc;
pub use rand::Rng;
pub use serde::{Deserialize, Serialize};
pub use serenity::{
    async_trait,
    builder::{
        CreateActionRow, CreateButton, CreateCommand, CreateCommandOption, CreateEmbed, CreateEmbedAuthor,
        CreateEmbedFooter, CreateInteractionResponse, CreateInteractionResponseFollowup,
        CreateInteractionResponseMessage, CreateMessage, CreateSelectMenu, CreateSelectMenuOption,
        EditInteractionResponse,
    },
    client::{Context, EventHandler},
    model::{
        application::{
            ActionRow, ActionRowComponent, Button, ButtonKind, ButtonStyle, CommandInteraction, CommandOptionType,
            ComponentInteraction, Interaction, ResolvedOption, ResolvedValue,
        },
        channel::{Channel, Embed, Message, ReactionType},
        gateway::{GatewayIntents, Ready},
        guild::Guild,
        id::{ChannelId, GuildId, MessageId, UserId},
        timestamp::Timestamp,
        user::User,
        Colour,
    },
    prelude::SerenityError,
    Client,
};
pub use small_fixed_array::{FixedArray, FixedString};
pub use std::{
    borrow::Cow,
    cmp::{max, min, Ordering, Reverse},
    collections::{HashMap, HashSet, LinkedList},
    convert::Infallible,
    fmt::{self, Display, Formatter},
    format as f, fs,
    future::Future,
    io, iter, mem,
    ops::{ControlFlow, Deref, DerefMut, FromResidual, Try},
    sync::Arc,
    time::{Duration, Instant},
};
pub use thiserror::Error;
pub use tokio::sync::{Mutex, RwLock, RwLockReadGuard, RwLockWriteGuard};
pub use tracing::{debug, error, info, instrument, span, subscriber, trace, warn, Level};

pub(crate) type Result<T> = std::result::Result<T, CoreError>;
