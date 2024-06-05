use crate::prelude::*;

pub fn register() -> CreateCommand<'static> {
    CreateCommand::new("calc").description("Calculate expr")
        .description_localized("pt-BR", "Calcular expressão")
        .add_option(CreateCommandOption::new(CommandOptionType::String, "expression", "The expression to calculate")
            .name_localized("pt-BR", "expressão")
            .description_localized("pt-BR", "A expressão a ser calculada")
        )
}

pub async fn run_command(bot: &impl AsBot, _slash: &CommandInteraction, args: &[ResolvedOption<'_>]) -> ResponseResult {
    let ResolvedValue::String(expr) = &args[0].value else {
        unreachable!("The 'expression' argument of the 'calc' command must be a string!");
    };

    let expr = match proccess_bracketed(bot, expr.split_whitespace().collect()).await
        .map(|expr| expr.into_boxed_str())
    {
        Ok(expr) => expr,
        Err(why) => { match why {
            BracketedError::IdNotFound(id) => {
                return simple_response(f!("Id não encontrado: {id}"), ResponseMode::Delete);
            },
            BracketedError::InvalidIdAttribute(attr) => {
                return simple_response(f!("Atributo inválido: {attr}"), ResponseMode::Delete)
            },
            BracketedError::InvalidIdProperty(prop) => {
                return simple_response(f!("Propriedade do Id inválida: {prop}"), ResponseMode::Delete);
            },
            BracketedError::MissingIdAttribute => {
                return simple_response("Atributo ausente.", ResponseMode::Delete)
            },
            BracketedError::MissingIdProperty => {
                return simple_response("Propriedade do Id ausente.", ResponseMode::Delete);
            },
            BracketedError::UnmatchedOpeningBracket => {
                return simple_response("Colchete de fechamento faltando.", ResponseMode::Delete);
            },
        }}
    };

    let value = match calculator::tokenize(&expr)
        .and_then(|tokens| calculator::parse(&tokens))
        .and_then(|tree| calculator::evaluate(&tree))
    {
        Ok(value) => value,
        Err(why) => { match why {
            CalcError::InvalidCharacter(ch) => {
                return simple_response(f!("Carácter inválido: {ch}"), ResponseMode::Delete);
            },
            CalcError::UnexpectedToken => {
                return simple_response("Token inesperado.", ResponseMode::Delete);
            },
            CalcError::UnmatchedOpeningParenthesis => {
                return simple_response("Parêntese de fechamento faltando.", ResponseMode::Delete);
            },
        }}
    };

    Ok((vec![ResponseBlueprint::default().content(f!("{expr} → {value}"))], ResponseMode::Normal))
}

#[derive(Debug, Error)]
enum BracketedError {
    #[error("Id not found: {0}")]
    IdNotFound(Box<str>),
    #[error("Invalid Id attribute: {0}")]
    InvalidIdAttribute(Box<str>),
    #[error("Invalid Id property: {0}")]
    InvalidIdProperty(Box<str>),
    #[error("Missing Id attribute")]
    MissingIdAttribute,
    #[error("Missing Id property")]
    MissingIdProperty,
    #[error("Unmatched opening bracket")]
    UnmatchedOpeningBracket,
}

async fn proccess_bracketed(bot: &impl AsBot, mut expr: String) -> Result<String, BracketedError> {
    let mut start = 0;

    while let Some(open_bracket_pos) = expr[start..].find('[') {
        let open_bracket_pos = start + open_bracket_pos;
        if let Some(close_bracket_pos) = expr[open_bracket_pos..].find(']') {
            let close_bracket_pos = open_bracket_pos + close_bracket_pos;

            let parts = expr[open_bracket_pos + 1..close_bracket_pos]
                .split('.')
                .collect::<Box<[&str]>>();

            let Ok(id_m) = Mirror::<Id>::get(bot, parts[0]).await else {
                return Err(BracketedError::IdNotFound(parts[0].into()));
            };

            let len = parts.len();
            if len < 2 {
                return Err(BracketedError::MissingIdProperty)
            }

            let value = match parts[1] {
                "attributes" | "atributos" | "attr" | "atr" => {
                    if len < 3 {
                        return Err(BracketedError::MissingIdAttribute)
                    }

                    match parts[2] {
                        "constitution" | "constituição" | "constituicao" | "con" => {
                            id_m.read().await.attributes.constitution as f64
                        },
                        "spirit" | "espírito" | "espirito" | "spr" | "esp" => {
                            id_m.read().await.attributes.spirit as f64
                        },
                        "might" | "poder" | "mgt" | "pdr" => {
                            id_m.read().await.attributes.might as f64
                        },
                        "movement" | "movimento" | "mov"=> {
                            id_m.read().await.attributes.movement as f64
                        },
                        "dexterity" | "destreza" | "dex" | "des" => {
                            id_m.read().await.attributes.dexterity as f64
                        },
                        "cognition" | "cognição" | "cognicao" | "cog" => {
                            id_m.read().await.attributes.cognition as f64
                        },
                        "charisma" | "carisma" | "cha" | "car" => {
                            id_m.read().await.attributes.charisma as f64
                        },
                        _ => return Err(BracketedError::InvalidIdAttribute(parts[2].into()))
                    }
                },
                _ => return Err(BracketedError::InvalidIdProperty(parts[1].into()))
            };

            expr.replace_range(open_bracket_pos..=close_bracket_pos, &value.to_string());	

            start = open_bracket_pos;
        } else {
            return Err(BracketedError::UnmatchedOpeningBracket);
        }
    }

    Ok(expr)
}