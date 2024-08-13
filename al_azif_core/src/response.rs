use crate::_prelude::*;

pub type Blueprints = Vec<ResponseBlueprint>;
pub type Responses = Vec<Response>;

#[derive(Debug)]
pub enum Response {
    // Only for prefix and slash commands
    DeleteOriginal,
    // Only for component-based interactions
    EditDefer { blueprint: ResponseBlueprint },
    // Only for component-based interactions
    EditDeferAndDelete { blueprint: ResponseBlueprint },
    Send { blueprints: Vec<ResponseBlueprint> },
    SendAndDelete { blueprints: Vec<ResponseBlueprint> },
    // Only for slash commands and component-based interactions
    SendEphemeral { blueprint: ResponseBlueprint },
    SendLoose { blueprints: Vec<ResponseBlueprint> },
    SendLooseAndDelete { blueprints: Vec<ResponseBlueprint> },
    // Only for component-based interactions
    Update { blueprint: ResponseBlueprint },
    // Only for component-based interactions
    UpdateDelayless { blueprint: ResponseBlueprint },
}
impl Response {
    pub fn delete_original() -> Self { Self::DeleteOriginal }

    pub fn edit_defer(blueprint: ResponseBlueprint) -> Self { Self::EditDefer { blueprint } }

    pub fn edit_defer_and_delete(blueprint: ResponseBlueprint) -> Self { Self::EditDeferAndDelete { blueprint } }
    pub fn send(blueprints: Vec<ResponseBlueprint>) -> Self { Self::Send { blueprints } }

    pub fn send_and_delete(blueprints: Vec<ResponseBlueprint>) -> Self { Self::SendAndDelete { blueprints } }

    pub fn send_and_delete_with_original(blueprints: Vec<ResponseBlueprint>) -> Vec<Self> {
        vec![Self::delete_original(), Self::send_and_delete(blueprints)]
    }

    pub fn send_ephemeral(blueprint: ResponseBlueprint) -> Self { Self::SendEphemeral { blueprint } }

    pub fn send_loose(blueprints: Vec<ResponseBlueprint>) -> Self { Self::SendLoose { blueprints } }

    pub fn send_loose_and_delete(blueprints: Vec<ResponseBlueprint>) -> Self { Self::SendLooseAndDelete { blueprints } }

    pub fn send_loose_and_delete_with_original(blueprints: Vec<ResponseBlueprint>) -> Vec<Self> {
        vec![Self::delete_original(), Self::send_loose_and_delete(blueprints)]
    }

    pub fn update(blueprint: ResponseBlueprint) -> Self { Self::Update { blueprint } }

    pub fn update_delayless(blueprint: ResponseBlueprint) -> Self { Self::UpdateDelayless { blueprint } }
}

#[derive(Clone, Debug, Default)]
pub struct ResponseBlueprint {
    // Missing attachments and allowed_mentions
    pub new_content:    Option<Cow<'static, str>>,
    pub new_embeds:     Cow<'static, [CreateEmbed<'static>]>,
    pub new_components: Cow<'static, [CreateActionRow<'static>]>,
}
impl ResponseBlueprint {
    pub fn new() -> Self { Self::default() }

    pub fn with_content(new_content: impl Into<Cow<'static, str>>) -> Self { Self::new().set_content(new_content) }

    pub fn add_embed(mut self, new_embed: CreateEmbed<'static>) -> Self {
        self.new_embeds.to_mut().push(new_embed);
        self
    }

    pub fn add_buttons(mut self, new_buttons: Vec<CreateButton<'static>>) -> Self {
        self.new_components.to_mut().push(CreateActionRow::Buttons(new_buttons));
        self
    }

    pub fn set_content(mut self, new_content: impl Into<Cow<'static, str>>) -> Self {
        self.new_content = Some(new_content.into());
        self
    }

    pub fn set_embeds(mut self, new_embeds: impl Into<Cow<'static, [CreateEmbed<'static>]>>) -> Self {
        self.new_embeds = new_embeds.into();
        self
    }

    pub fn set_components(mut self, new_components: impl Into<Cow<'static, [CreateActionRow<'static>]>>) -> Self {
        self.new_components = new_components.into();
        self
    }
}
impl ResponseBlueprint {
    pub fn create_interaction_response_message(&self) -> CreateInteractionResponseMessage<'static> {
        let mut into = CreateInteractionResponseMessage::new()
            .embeds(self.new_embeds.clone())
            .components(self.new_components.clone());

        if let Some(new_content) = &self.new_content {
            into = into.content(new_content.clone());
        }

        into
    }

    pub fn create_message(&self) -> CreateMessage<'static> {
        let mut into = CreateMessage::new().embeds(self.new_embeds.clone()).components(self.new_components.clone());

        if let Some(new_content) = &self.new_content {
            into = into.content(new_content.clone());
        }

        into
    }

    pub fn edit_interaction_response(&self) -> EditInteractionResponse<'static> {
        let mut into = EditInteractionResponse::new()
            .embeds(self.new_embeds.clone())
            .components(self.new_components.clone());
    
        if let Some(new_content) = &self.new_content {
            into = into.content(new_content.clone());
        }

        into
    }
}

#[derive(Debug)]
pub enum ErrorResponse {
    EditDefer { blueprint: ResponseBlueprint },
    Send { blueprints: Vec<ResponseBlueprint> },
    SendLoose { blueprints: Vec<ResponseBlueprint> },
}
impl ErrorResponse {
    pub fn edit_defer(blueprint: ResponseBlueprint) -> Self { Self::EditDefer { blueprint } }
    pub fn send(blueprints: Vec<ResponseBlueprint>) -> Self { Self::Send { blueprints } }
    pub fn send_loose(blueprints: Vec<ResponseBlueprint>) -> Self { Self::SendLoose { blueprints } }
}

pub fn simple_send(new_content: impl Into<Cow<'static, str>>) -> Vec<Response> {
    vec![Response::Send { blueprints: vec![ResponseBlueprint { new_content: Some(new_content.into()), ..Default::default() }] }]
}
