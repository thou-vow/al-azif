use crate::_prelude::*;

pub async fn register(bot: &impl AsBot, ctx: &Context) -> Result<()> {
    use al_azif_slash::commands::*;

    bot.get_main_guild_id()
        .set_commands(&ctx.http, &[battle::register(), exp::register(), help::register(), id::register(), voice::register()])
        .await
        .map_err(EventError::CouldNotSetSlashCommands)?;

    Ok(())
}

pub async fn run(bot: &impl AsBot, ctx: &Context, slash: &CommandInteraction) -> Result<()> {
    use al_azif_slash::commands::*;
    use ResolvedOption as RO;
    use ResolvedValue as RV;

    let name = slash.data.name.as_str();
    let options: Vec<ResolvedOption> = slash.data.options();

    let execution_result = 'match_slash: {
        match name {
            battle::TAG => match options.as_slice() {
                [RO { name: battle::end::TAG, value: RV::SubCommand(_), .. }, ..] => {
                    battle::end::run_slash(bot, slash).await.map_err(EventError::Slash)
                },
                [RO { name: battle::join::TAG, value: RV::SubCommand(args), .. }, ..] => {
                    let mut iter = args.iter();
                    let ids = parse_slash_arg!('match_slash, iter, "ids", &str);
                    battle::join::run_slash(bot, slash, ids).await.map_err(EventError::Slash)
                },
                [RO { name: battle::start::TAG, value: RV::SubCommand(args), .. }, ..] => {
                    let mut iter = args.iter();
                    let ids = parse_slash_arg!('match_slash, iter, "ids", &str);
                    battle::start::run_slash(bot, slash, ids).await.map_err(EventError::Slash)
                },
                _ => Err(EventError::InvalidSlashSubCommand { command_tag: battle::TAG }),
            },
            exp::TAG => match options.as_slice() {
                [RO { name: exp::bestow::TAG, value: RV::SubCommand(args), .. }, ..] => {
                    let mut iter = args.iter();
                    let ids = parse_slash_arg!('match_slash, iter, "ids", &str);
                    let amount = parse_slash_arg!('match_slash, iter, "amount", i64);
                    exp::bestow::run_slash(bot, ids, amount).await.map_err(EventError::Slash)
                },
                _ => Err(EventError::InvalidSlashSubCommand { command_tag: exp::TAG }),
            },
            id::TAG => match options.as_slice() {
                [RO { name: id::distribute::TAG, value: RV::SubCommand(args), .. }, ..] => {
                    let mut iter = args.iter();
                    let id = parse_slash_arg!('match_slash, iter, "id", &str);
                    id::distribute::run_slash(bot, id).await.map_err(EventError::Slash)
                },
                _ => Err(EventError::InvalidSlashSubCommand { command_tag: id::TAG }),
            },
            help::TAG => help::run_slash().await.map_err(EventError::Slash),
            ping::TAG => ping::run_slash(ctx, slash).await.map_err(EventError::Slash),
            voice::TAG => match options.as_slice() {
                [RO { name: voice::play::TAG, value: RV::SubCommandGroup(sub_options), .. }, ..] => match sub_options.as_slice() {
                    [RO { name: voice::play::youtube::TAG, value: RV::SubCommand(args), .. }, ..] => {
                        let mut iter = args.iter();
                        let url = parse_slash_arg!('match_slash, iter, "url", &str);
                        voice::play::youtube::run_slash(bot, ctx, slash, url).await.map_err(EventError::Slash)
                    },
                    _ => Err(EventError::InvalidSlashSubCommandOfGroup { command_tag: voice::TAG, group_tag: voice::play::TAG }),
                },
                _ => Err(EventError::InvalidSlashSubCommand { command_tag: voice::TAG }),
            },
            _ => Err(EventError::InvalidSlashCommand { tag: FixedString::from_str_trunc(name) }),
        }
    };

    let responses = match execution_result {
        Ok(responses) => responses,
        Err(EventError::Prefix(PrefixError::Anticipated(ErrorResponse::EditDefer { blueprint }))) => {
            vec![Response::edit_defer_and_delete(blueprint)]
        },
        Err(EventError::Prefix(PrefixError::Anticipated(ErrorResponse::Send { blueprints }))) => {
            vec![Response::send_and_delete(blueprints)]
        },
        Err(EventError::Prefix(PrefixError::Anticipated(ErrorResponse::SendLoose { blueprints }))) => {
            vec![Response::send_loose_and_delete(blueprints)]
        },
        Err(err) => return Err(err),
    };

    perform_responses(ctx, slash, responses).await
}

pub async fn perform_responses(ctx: &Context, slash: &CommandInteraction, responses: Responses) -> Result<()> {
    let mut msgs_to_delete = Vec::new();

    for response in responses {
        match response {
            Response::DeleteOriginal => (),
            Response::EditDefer { blueprint } => {
                slash
                    .edit_response(&ctx.http, blueprint.edit_interaction_response())
                    .await
                    .map_err(EventError::CouldNotEditInteractionResponse)?;

                tokio::time::sleep(RESPONSE_INTERVAL).await;
            },
            Response::EditDeferAndDelete { blueprint } => {
                slash
                    .edit_response(&ctx.http, blueprint.edit_interaction_response())
                    .await
                    .map_err(EventError::CouldNotEditInteractionResponse)?;
                msgs_to_delete.push(
                    ctx.http
                        .get_original_interaction_response(&slash.token)
                        .await
                        .map_err(EventError::CouldNotGetOriginalInteractionResponse)?,
                );

                tokio::time::sleep(RESPONSE_INTERVAL).await;
            },
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
