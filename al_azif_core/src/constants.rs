pub use crate::_prelude::*;

pub const PREFIX: &str = "!";

pub const CACHE_FLUSH_ROUTINE: Duration = Duration::from_secs(800);
pub const CACHE_EXPIRE_TIME: Duration = Duration::from_secs(1200);
pub const RESPONSE_INTERVAL: Duration = Duration::from_millis(500);
pub const RESPONSE_TIMEOUT: Duration = Duration::from_secs(10);
pub const DELETE_INTERVAL: Duration = Duration::from_secs(1);

pub const ATTRIBUTES_LONG: &str = "Atributos";
pub const GO_BACK_EMOJI: &str = "🔙";

pub const HP_SHORT: &str = "PV";
pub const SP_SHORT: &str = "SP";

pub const HP_SHORT_PT: &str = "PV";
pub const SP_SHORT_PT: &str = "PE";

pub const CON_SHORT: &str = "CON";
pub const SPR_SHORT: &str = "SPR";
pub const MGT_SHORT: &str = "MGT";
pub const MOV_SHORT: &str = "MOV";
pub const DEX_SHORT: &str = "DEX";
pub const COG_SHORT: &str = "COG";
pub const CHA_SHORT: &str = "CHA";

pub const CON_SHORT_PT: &str = "CON";
pub const SPR_SHORT_PT: &str = "ESP";
pub const MGT_SHORT_PT: &str = "PDR";
pub const MOV_SHORT_PT: &str = "MOV";
pub const DEX_SHORT_PT: &str = "DES";
pub const COG_SHORT_PT: &str = "COG";
pub const CHA_SHORT_PT: &str = "CAR";

pub const CON_LONG: &str = "Constitution";
pub const SPR_LONG: &str = "Spirit";
pub const MGT_LONG: &str = "Might";
pub const MOV_LONG: &str = "Movement";
pub const DEX_LONG: &str = "Dexterity";
pub const COG_LONG: &str = "Cognition";
pub const CHA_LONG: &str = "Charisma";

pub const CON_LONG_PT: &str = "Constituição";
pub const SPR_LONG_PT: &str = "Espírito";
pub const MGT_LONG_PT: &str = "Poder";
pub const MOV_LONG_PT: &str = "Movimento";
pub const DEX_LONG_PT: &str = "Destreza";
pub const COG_LONG_PT: &str = "Cognição";
pub const CHA_LONG_PT: &str = "Carisma";

pub const CON_EMOJI: &str = "❤️";
pub const SPR_EMOJI: &str = "🌟";
pub const MGT_EMOJI: &str = "⚔️";
pub const MOV_EMOJI: &str = "🏃‍♂️";
pub const DEX_EMOJI: &str = "🤹";
pub const COG_EMOJI: &str = "💡";
pub const CHA_EMOJI: &str = "😎";

pub const STRIKE_LONG: &str = "Strike";
pub const SLASH_LONG: &str = "Slash";
pub const FIRE_LONG: &str = "Fire";
pub const ICE_LONG: &str = "Ice";
pub const LIGHTNING_LONG: &str = "Lightning";
pub const WIND_LONG: &str = "Wind";
pub const EARTH_LONG: &str = "Earth";
pub const WATER_LONG: &str = "Water";
pub const WOOD_LONG: &str = "Wood";
pub const CHAOS_LONG: &str = "Chaos";
pub const PURE_LONG: &str = "Pure";

pub const STRIKE_LONG_PT: &str = "Impacto";
pub const SLASH_LONG_PT: &str = "Corte";
pub const FIRE_LONG_PT: &str = "Fogo";
pub const ICE_LONG_PT: &str = "Gelo";
pub const LIGHTNING_LONG_PT: &str = "Raio";
pub const WIND_LONG_PT: &str = "Vento";
pub const EARTH_LONG_PT: &str = "Terra";
pub const WATER_LONG_PT: &str = "Água";
pub const WOOD_LONG_PT: &str = "Madeira";
pub const CHAOS_LONG_PT: &str = "Caos";
pub const PURE_LONG_PT: &str = "Puro";

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
pub const MODERATE_EMOJI: &str = "🟡";
pub const HEAVY_EMOJI: &str = "🔴";
pub const SEVERE_EMOJI: &str = "⚠️";

pub const fn xp_to_next_level(lvl: i64) -> i64 { ((lvl ^ 2) * 5) + lvl * 50 + 100 }

pub const PERMISSION_LEVEL_LONG_PT: &str = "Nível de Permissão";
pub const QUARENTINE_LONG_PT: &str = "Quarentena";
pub const BASIC_LONG_PT: &str = "Básico";
pub const MASTER_LONG_PT: &str = "Mestre";
