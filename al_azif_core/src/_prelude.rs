pub use crate::{
    battle::{Battle, Moment, Opponent, PrimaryMoment, ReactiveMoment},
    bot::{AsBot, InMemoryDatabase, Lang},
    constants::*,
    database::{self, Error as DatabaseError, Reflective},
    effect::{self, all::*, AsEffect, Effect},
    id::{Age, Gender, Id},
    lang_diff,
    mirror::{InMemoryStore, Mirror, ReadMirror, WriteMirror},
    player::Player,
    response::{self, Blueprints, ErrorResponse, Response, ResponseBlueprint, Responses},
    utils::{
        self,
        fmt::{join_with_and, mark_thousands, mark_thousands_and_show_sign},
        roll::{RollExpression, RollSummary},
    },
    voice::{self, Error as VoiceError},
    Error as CoreError,
};
pub use ahash::{AHashMap, AHashSet};
pub use const_format::formatcp as fc;
pub use rand::Rng;
pub use reqwest::Client as ReqwestClient;
pub use rusty_ytdl::{Video, VideoError, VideoOptions, VideoQuality, VideoSearchOptions};
pub use serde::{Deserialize, Serialize};
pub use serenity::{
    async_trait,
    builder::{
        CreateActionRow, CreateButton, CreateCommand, CreateCommandOption, CreateEmbed, CreateEmbedAuthor, CreateEmbedFooter,
        CreateInteractionResponse, CreateInteractionResponseFollowup, CreateInteractionResponseMessage, CreateMessage, CreateSelectMenu,
        CreateSelectMenuOption, EditInteractionResponse,
    },
    client::{Context, EventHandler},
    model::{
        application::{
            ActionRow, ActionRowComponent, Button, ButtonKind, ButtonStyle, CommandInteraction, CommandOptionType, ComponentInteraction,
            Interaction, ResolvedOption, ResolvedValue,
        },
        channel::{Channel, Embed, Message, ReactionConversionError, ReactionType},
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
pub use songbird::{
    error::JoinError,
    input::{AuxMetadata, File as AudioFile, HttpRequest as AudioHttpRequest, Input, YoutubeDl},
    Event as VoiceEvent, EventContext as VoiceEventContext, EventHandler as VoiceEventHandler, Songbird, TrackEvent,
};
pub use std::{
    borrow::Cow,
    cmp::{max, min, Ordering, Reverse},
    collections::{HashMap, HashSet, LinkedList, VecDeque},
    convert::Infallible,
    fmt::{self, Debug, Display, Formatter},
    format as f, fs,
    future::Future,
    io, iter, mem,
    ops::{ControlFlow, Deref, DerefMut},
    str::Split,
    sync::Arc,
    time::{Duration, Instant},
};
pub use thiserror::Error;
pub use tokio::sync::{Mutex, RwLock, RwLockReadGuard, RwLockWriteGuard};
pub use tracing::{debug, error, info, instrument, span, trace, warn, Level};

pub(crate) type Result<T> = std::result::Result<T, CoreError>;
