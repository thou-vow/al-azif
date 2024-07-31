use crate::_prelude::*;

macro parse_slash_arg {
    ($label:lifetime, $iter:expr, $name:expr, &str) => {
        if let Some(opt) = $iter.find(|opt| opt.name == $name) {
            match opt {
                ResolvedOption { name: $name, value: ResolvedValue::String(value), .. } => *value,
                ResolvedOption { name: _, value: ResolvedValue::String(_), .. } => {
                    break $label Err(EventError::ExpectedAnotherSlashCommandOptionName {
                        r#type:        "String",
                        expected_name: $name,
                    })
                },
                ResolvedOption { name: $name, value: _, .. } => {
                    break $label Err(EventError::ExpectedAnotherSlashCommandOptionType {
                        name:          $name,
                        expected_type: "String",
                    })
                },
                _ => break $label Err(EventError::ExpectedAnotherSlashCommandOption {
                    expected_name: $name,
                    expected_type: "String",
                }),
            }
        } else {
            break $label Err(EventError::MissingRequiredSlashCommandOption { name: $name })
        }
    },
    ($label:lifetime, $iter:expr, $name:expr, Option<str>) => {
        if let Some(opt) = $iter.find(|opt| opt.name == $name) {
            match opt {
                ResolvedOption { name: $name, value: ResolvedValue::String(value), .. } => Some(*value),
                ResolvedOption { name: _, value: ResolvedValue::String(_), .. } => {
                    break $label Err(EventError::ExpectedAnotherSlashCommandOptionName {
                        r#type:        "String",
                        expected_name: $name,
                    })
                },
                ResolvedOption { name: $name, value: _, .. } => {
                    break $label Err(EventError::ExpectedAnotherSlashCommandOptionType {
                        name:          $name,
                        expected_type: "String",
                    })
                },
                _ => break $label Err(EventError::ExpectedAnotherSlashCommandOption {
                    expected_name: $name,
                    expected_type: "String",
                }),
            }
        } else {
            None
        }
    },
    ($label:lifetime, $iter:expr, $name:expr, i64) => {
        if let Some(opt) = $iter.find(|opt| opt.name == $name) {
            match opt {
                ResolvedOption { name: $name, value: ResolvedValue::Integer(value), .. } => *value,
                ResolvedOption { name: _, value: ResolvedValue::Integer(_), .. } => {
                    break $label Err(EventError::ExpectedAnotherSlashCommandOptionName {
                        r#type:        "Integer",
                        expected_name: $name,
                    })
                },
                ResolvedOption { name: $name, value: _, .. } => {
                    break $label Err(EventError::ExpectedAnotherSlashCommandOptionType {
                        name:          $name,
                        expected_type: "Integer",
                    })
                },
                _ => break $label Err(EventError::ExpectedAnotherSlashCommandOption {
                    expected_name: $name,
                    expected_type: "Integer",
                }),
            }
        } else {
            break $label Err(EventError::MissingRequiredSlashCommandOption { name: $name })
        }
    },
}

pub async fn register(bot: &impl AsBot, ctx: &Context) -> Result<()> {
    use al_azif_slash::commands::*;

    bot.get_main_guild()
        .set_commands(&ctx.http, &[battle::register(), exp::register(), help::register(), id::register()])
        .await
        .map_err(EventError::CouldNotSetSlashCommands)?;

    Ok(())
}

pub async fn run(bot: &impl AsBot, ctx: &Context, slash: &CommandInteraction) -> Result<()> {
    use al_azif_slash::commands::*;
    use ResolvedOption as RO;
    use ResolvedValue as RV;

    let name = slash.data.name.as_str();
    let options = slash.data.options();

    let execution_result = 'match_slash: {
        match name {
            battle::NAME => match options.as_slice() {
                [RO { name: battle::end::NAME, value: RV::SubCommand(_), .. }, ..] => {
                    battle::end::run_slash(bot, slash).await.map_err(EventError::Slash)
                },
                [RO { name: battle::join::NAME, value: RV::SubCommand(args), .. }, ..] => {
                    let mut iter = args.iter();
                    let ids = parse_slash_arg!('match_slash, iter, "ids", &str);
                    battle::join::run_slash(bot, slash, ids).await.map_err(EventError::Slash)
                },
                [RO { name: battle::start::NAME, value: RV::SubCommand(args), .. }, ..] => {
                    let mut iter = args.iter();
                    let ids = parse_slash_arg!('match_slash, iter, "ids", &str);
                    battle::start::run_slash(bot, slash, ids).await.map_err(EventError::Slash)
                },
                _ => Err(EventError::InvalidSlashCommand { name: FixedString::from_str_trunc(name) }),
            },
            exp::NAME => match options.as_slice() {
                [RO { name: exp::bestow::NAME, value: RV::SubCommand(args), .. }, ..] => {
                    let mut iter = args.iter();
                    let ids = parse_slash_arg!('match_slash, iter, "ids", &str);
                    let amount = parse_slash_arg!('match_slash, iter, "amount", i64);
                    exp::bestow::run_slash(bot, ids, amount).await.map_err(EventError::Slash)
                },
                _ => Err(EventError::InvalidSlashCommand { name: FixedString::from_str_trunc(name) }),
            },
            id::NAME => match options.as_slice() {
                [RO { name: id::distribute::NAME, value: RV::SubCommand(args), .. }, ..] => {
                    let mut iter = args.iter();
                    let id = parse_slash_arg!('match_slash, iter, "id", &str);
                    id::distribute::run_slash(bot, id).await.map_err(EventError::Slash)
                },
                _ => Err(EventError::InvalidSlashCommand { name: FixedString::from_str_trunc(name) }),
            },
            help::NAME => help::run_slash().await.map_err(EventError::Slash),
            ping::NAME => ping::run_slash(ctx, slash).await.map_err(EventError::Slash),
            _ => Err(EventError::InvalidSlashCommand { name: FixedString::from_str_trunc(name) }),
        }
    };

    let responses = execution_result?;

    perform_response_responses(ctx, slash, responses).await
}

pub async fn perform_response_responses<'a>(ctx: &Context, slash: &CommandInteraction, responses: Responses<'a>) -> Result<()> {
    let mut msgs_to_delete = Vec::new();

    for response in responses {
        match response {
            Response::DeleteOriginal => (),
            Response::Send { blueprints } => {
                let Some(first_blueprint) = blueprints.first() else {
                    continue;
                };

                slash
                    .create_response(&ctx.http, CreateInteractionResponse::Message(first_blueprint.create_interaction_response_message()))
                    .await
                    .map_err(EventError::CouldNotCreateInteractionResponse)?;

                tokio::time::sleep(RESPONSE_INTERVAL).await;

                for blueprint in blueprints.iter().skip(1) {
                    slash
                        .channel_id
                        .send_message(&ctx.http, blueprint.create_message())
                        .await
                        .map_err(EventError::CouldNotSendMessage)?;

                    tokio::time::sleep(RESPONSE_INTERVAL).await;
                }
            },
            Response::SendAndDelete { blueprints } => {
                let Some(first_blueprint) = blueprints.first() else {
                    continue;
                };

                slash
                    .create_response(&ctx.http, CreateInteractionResponse::Message(first_blueprint.create_interaction_response_message()))
                    .await
                    .map_err(EventError::CouldNotCreateInteractionResponse)?;
                msgs_to_delete.push(
                    ctx.http
                        .get_original_interaction_response(&slash.token)
                        .await
                        .map_err(EventError::CouldNotGetOriginalInteractionResponse)?,
                );

                tokio::time::sleep(RESPONSE_INTERVAL).await;

                for blueprint in blueprints.iter().skip(1) {
                    msgs_to_delete.push(
                        slash
                            .channel_id
                            .send_message(&ctx.http, blueprint.create_message())
                            .await
                            .map_err(EventError::CouldNotSendMessage)?,
                    );

                    tokio::time::sleep(RESPONSE_INTERVAL).await;
                }
            },
            Response::SendEphemeral { blueprint } => {
                slash
                    .create_response(
                        &ctx.http,
                        CreateInteractionResponse::Message(blueprint.create_interaction_response_message().ephemeral(true)),
                    )
                    .await
                    .map_err(EventError::CouldNotCreateInteractionResponse)?;

                tokio::time::sleep(RESPONSE_INTERVAL).await;
            },
            Response::SendLoose { blueprints } => {
                for blueprint in blueprints {
                    slash
                        .channel_id
                        .send_message(&ctx.http, blueprint.create_message())
                        .await
                        .map_err(EventError::CouldNotSendMessage)?;

                    tokio::time::sleep(RESPONSE_INTERVAL).await;
                }
            },
            Response::SendLooseAndDelete { blueprints } => {
                for blueprint in blueprints {
                    msgs_to_delete.push(
                        slash
                            .channel_id
                            .send_message(&ctx.http, blueprint.create_message())
                            .await
                            .map_err(EventError::CouldNotSendMessage)?,
                    );

                    tokio::time::sleep(RESPONSE_INTERVAL).await;
                }
            },
            Response::Update { .. } => (),
            Response::UpdateDelayless { .. } => (),
        }
    }

    tokio::time::sleep(RESPONSE_TIMEOUT).await;

    for msg_to_delete in msgs_to_delete {
        msg_to_delete.delete(&ctx.http, None).await.map_err(EventError::CouldNotDeleteMessage)?;

        tokio::time::sleep(RESPONSE_INTERVAL).await;
    }

    Ok(())
}
