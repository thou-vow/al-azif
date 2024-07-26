use crate::_prelude::*;

macro_rules! match_slash {
    ($name:expr, $args:expr,
    $(
        ($($($pattern_parts:ident)::+),+)
        =>
        $($function_parts:ident)::+
        [$(
            $($($api_tools_parts:ident)::+),+
            $(
                ;
                $($arg_name:ident: $arg_type:tt),+
            )?
        )?]
    ),*) => {
        'match_slash: {
            // This is a workaround to cheat the macro scoping rules.
            // It makes the macro think every variable on this scope that uses '_args' will use it,
            // unless using a sub or sub-sub command.
            // This will most likely be optimized by the compiler.
            let _args = $args;
            match ($name, _args) {
                $(match_slash!(@pattern ($($($pattern_parts)::+),+)) => {
                    // '_iter' is included in this macro to access '_args' in the same scope.
                    // Since $()? requires the optional metavariable to be present, it can't be written only when there are args.
                    // Therefore, this inclusion is necessary and will most likely be optimized by the compiler.
                    let mut _iter = _args.iter();
                    $($function_parts)::+($($($($api_tools_parts)::*),* $(, ($({
                        match parse_slash_arg!(_iter, stringify!($arg_name), $arg_type) {
                            Ok(value) => value,
                            Err(e) => break 'match_slash Err(e),
                        }
                    }),+))?)?).await.map_err(EventError::Slash)
                }),*
                (_, _) => Err(EventError::InvalidSlashCommand { name: FixedString::from_str_trunc($name) }),
            }
        }
    };

    // Subcommand
    (@pattern (
        $($cmd_name_parts:ident)::+,
        $($sub_cmd_name_parts:ident)::+
    )) => {
        ($($cmd_name_parts)::+, [ResolvedOption { name: $($sub_cmd_name_parts)::+, value: ResolvedValue::SubCommand(_args), .. }, ..])
    };

    // Command
    (@pattern (
        $($cmd_name_parts:ident)::+
    )) => {
        ($($cmd_name_parts)::+, _args)
    };
}

macro_rules! parse_slash_arg {
    ($iter:expr, $name:expr, str) => {
        if let Some(opt) = $iter.find(|opt| opt.name == $name) {
            match opt {
                ResolvedOption { name: $name, value: ResolvedValue::String(value), .. } => Ok(*value),
                ResolvedOption { name: _, value: ResolvedValue::String(_), .. } => {
                    Err(EventError::ExpectedAnotherSlashCommandOptionName {
                        r#type:        "String",
                        expected_name: $name,
                    })
                },
                ResolvedOption { name: $name, value: _, .. } => {
                    Err(EventError::ExpectedAnotherSlashCommandOptionType {
                        name:          $name,
                        expected_type: "String",
                    })
                },
                _ => Err(EventError::ExpectedAnotherSlashCommandOption {
                    expected_name: $name,
                    expected_type: "String",
                }),
            }
        } else {
            Err(EventError::MissingRequiredSlashCommandOption { name: $name })
        }
    };
    ($iter:expr, $name:expr, Option<str>) => {
        if let Some(opt) = $iter.find(|opt| opt.name == $name) {
            match opt {
                ResolvedOption { name: $name, value: ResolvedValue::String(value), .. } => Ok(Some(*value)),
                ResolvedOption { name: _, value: ResolvedValue::String(_), .. } => {
                    Err(EventError::ExpectedAnotherSlashCommandOptionName {
                        r#type:        "String",
                        expected_name: $name,
                    })
                },
                ResolvedOption { name: $name, value: _, .. } => {
                    Err(EventError::ExpectedAnotherSlashCommandOptionType {
                        name:          $name,
                        expected_type: "String",
                    })
                },
                _ => Err(EventError::ExpectedAnotherSlashCommandOption {
                    expected_name: $name,
                    expected_type: "String",
                }),
            }
        } else {
            Ok(None)
        }
    };
    ($iter:expr, $name:expr, i64) => {
        if let Some(opt) = $iter.find(|opt| opt.name == $name) {
            match opt {
                ResolvedOption { name: $name, value: ResolvedValue::Integer(value), .. } => Ok(*value),
                ResolvedOption { name: _, value: ResolvedValue::Integer(_), .. } => {
                    Err(EventError::ExpectedAnotherSlashCommandOptionName {
                        r#type:        "Integer",
                        expected_name: $name,
                    })
                },
                ResolvedOption { name: $name, value: _, .. } => {
                    Err(EventError::ExpectedAnotherSlashCommandOptionType {
                        name:          $name,
                        expected_type: "Integer",
                    })
                },
                _ => Err(EventError::ExpectedAnotherSlashCommandOption {
                    expected_name: $name,
                    expected_type: "Integer",
                }),
            }
        } else {
            Err(EventError::MissingRequiredSlashCommandOption { name: $name })
        }
    };
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

    let args = slash.data.options();

    let execution_result = match_slash!(
        slash.data.name.as_str(), args.as_slice(),
        (battle::NAME, battle::end::NAME) => battle::end::run[bot, slash],
        (battle::NAME, battle::join::NAME) => battle::join::run[bot, slash; ids: str],
        (battle::NAME, battle::start::NAME) => battle::start::run[bot, slash; ids: str],
        (exp::NAME, exp::bestow::NAME) => exp::bestow::run[bot; ids: str, value: i64],
        (id::NAME, id::distribute::NAME) => id::distribute::run[bot; id: str],
        (help::NAME) => help::run[],
        (ping::NAME) => ping::run[ctx, slash]
    );

    let responses = execution_result?;

    perform_response_responses(ctx, slash, responses).await
}

pub async fn perform_response_responses<'a>(
    ctx: &Context,
    slash: &CommandInteraction,
    responses: Responses<'a>,
) -> Result<()> {
    let mut msgs_to_delete = Vec::new();

    for response in responses {
        match response {
            Response::DeleteOriginal => (),
            Response::Send { blueprints } => {
                let Some(first_blueprint) = blueprints.first() else {
                    continue;
                };

                slash
                    .create_response(
                        &ctx.http,
                        CreateInteractionResponse::Message(first_blueprint.create_interaction_response_message()),
                    )
                    .await
                    .map_err(EventError::CouldNotCreateInteractionResponse)?;

                for blueprint in blueprints.iter().skip(1) {
                    slash
                        .channel_id
                        .send_message(&ctx.http, blueprint.create_message())
                        .await
                        .map_err(EventError::CouldNotSendMessage)?;
                }
            },
            Response::SendAndDelete { blueprints } => {
                let Some(first_blueprint) = blueprints.first() else {
                    continue;
                };

                slash
                    .create_response(
                        &ctx.http,
                        CreateInteractionResponse::Message(first_blueprint.create_interaction_response_message()),
                    )
                    .await
                    .map_err(EventError::CouldNotCreateInteractionResponse)?;
                msgs_to_delete.push(
                    ctx.http
                        .get_original_interaction_response(&slash.token)
                        .await
                        .map_err(EventError::CouldNotGetOriginalInteractionResponse)?,
                );

                for blueprint in blueprints.iter().skip(1) {
                    msgs_to_delete.push(
                        slash
                            .channel_id
                            .send_message(&ctx.http, blueprint.create_message())
                            .await
                            .map_err(EventError::CouldNotSendMessage)?,
                    );
                }
            },
            Response::SendEphemeral { blueprint } => {
                slash
                    .create_response(
                        &ctx.http,
                        CreateInteractionResponse::Message(
                            blueprint.create_interaction_response_message().ephemeral(true),
                        ),
                    )
                    .await
                    .map_err(EventError::CouldNotCreateInteractionResponse)?;
            },
            Response::SendLoose { blueprints } => {
                for blueprint in blueprints {
                    slash
                        .channel_id
                        .send_message(&ctx.http, blueprint.create_message())
                        .await
                        .map_err(EventError::CouldNotSendMessage)?;
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
