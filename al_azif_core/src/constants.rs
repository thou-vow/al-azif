pub use crate::prelude::*;

pub const PREFIX: &str = "!";

pub const CACHE_FLUSH_ROUTINE: Duration = Duration::from_secs(800);
pub const CACHE_EXPIRE_TIME: Duration = Duration::from_secs(1200);
pub const RESPONSE_INTERVAL: Duration = Duration::from_secs(1);

pub const ATTRIBUTES_LONG: &str = "Atributos";
pub const GO_BACK_EMOJI: &str = "🔙";

pub const HP_SHORT: &str = "PV";
pub const SP_SHORT: &str = "SP";

pub const CON_SHORT: &str = "CON";
pub const SPR_SHORT: &str = "ESP";
pub const MGT_SHORT: &str = "PDR";
pub const MOV_SHORT: &str = "MOV";
pub const DEX_SHORT: &str = "DES";
pub const COG_SHORT: &str = "COG";
pub const CHA_SHORT: &str = "CAR";

pub const CON_LONG: &str = "Constituição";
pub const SPR_LONG: &str = "Espírito";
pub const MGT_LONG: &str = "Poder";
pub const MOV_LONG: &str = "Movimento";
pub const DEX_LONG: &str = "Destreza";
pub const COG_LONG: &str = "Cognição";
pub const CHA_LONG: &str = "Carisma";

pub const CON_EMOJI: &str = "❤️";
pub const SPR_EMOJI: &str = "🌟";
pub const MGT_EMOJI: &str = "⚔️";
pub const MOV_EMOJI: &str = "🏃‍♂️";
pub const DEX_EMOJI: &str = "🤹";
pub const COG_EMOJI: &str = "💡";
pub const CHA_EMOJI: &str = "😎";

pub const STRIKE_LONG: &str = "Impacto";
pub const SLASH_LONG: &str = "Corte";
pub const FIRE_LONG: &str = "Fogo";
pub const ICE_LONG: &str = "Gelo";
pub const LIGHTNING_LONG: &str = "Raio";
pub const WIND_LONG: &str = "Vento";
pub const EARTH_LONG: &str = "Terra";
pub const WATER_LONG: &str = "Água";
pub const WOOD_LONG: &str = "Madeira";
pub const CHAOS_LONG: &str = "Caos";
pub const PURE_LONG: &str = "Puro";

pub const STRIKE_EMOJI: &str = "💥";
pub const SLASH_EMOJI: &str = "🗡";
pub const FIRE_EMOJI: &str = "🔥";
pub const ICE_EMOJI: &str = "❄️";
pub const LIGHTNING_EMOJI: &str = "⚡";
pub const WIND_EMOJI: &str = "🌪";
pub const EARTH_EMOJI: &str = "🪨";
pub const WATER_EMOJI: &str = "💧";
pub const WOOD_EMOJI: &str = "🌿";
pub const CHAOS_EMOJI: &str = "🌀";
pub const PURE_EMOJI: &str = "☸️";

pub const LIGHT_EMOJI: &str = "🟢";
pub const MEDIUM_EMOJI: &str = "🟡";
pub const HEAVY_EMOJI: &str =  "🔴";
pub const SEVERE_EMOJI: &str = "⚠️";

pub const fn xp_to_next_level(lvl: i64) -> i64 {
    ((lvl ^ 2) * 5) + lvl * 50 + 100
}

pub const PL_LONG: &str = "Nível de Permissão";
pub const PL_QUARENTINE_LONG: &str = "Quarentena";
pub const PL_BASIC_LONG: &str = "Básico";
pub const PL_MASTER_LONG: &str = "Mestre";