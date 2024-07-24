use crate::_prelude::*;

pub type Blueprints<'a> = Vec<ResponseBlueprint<'a>>;
pub type Responses<'a> = Vec<Response<'a>>;

#[derive(Debug)]
pub enum Response<'a> {
    // Only for prefix and slash commands
    DeleteOriginal,
    Send { blueprints: Vec<ResponseBlueprint<'a>> },
    SendAndDelete { blueprints: Vec<ResponseBlueprint<'a>> },
    // Only for slash commands and component-based interactions
    SendEphemeral { blueprint: ResponseBlueprint<'a> },
    SendLoose { blueprints: Vec<ResponseBlueprint<'a>> },
    SendLooseAndDelete { blueprints: Vec<ResponseBlueprint<'a>> },
    // Only for component-based interactions
    Update { blueprint: ResponseBlueprint<'a> },
    // Only for component-based interactions
    UpdateDelayless { blueprint: ResponseBlueprint<'a> },
}
impl<'a> Response<'a> {
    pub fn delete_original() -> Self { Self::DeleteOriginal }

    pub fn send(blueprints: Vec<ResponseBlueprint<'a>>) -> Self { Self::Send { blueprints } }

    pub fn send_and_delete(blueprints: Vec<ResponseBlueprint<'a>>) -> Self { Self::SendAndDelete { blueprints } }

    pub fn send_and_delete_with_original(blueprints: Vec<ResponseBlueprint<'a>>) -> Vec<Self> {
        vec![Self::delete_original(), Self::send_and_delete(blueprints)]
    }

    pub fn send_ephemeral(blueprint: ResponseBlueprint<'a>) -> Self { Self::SendEphemeral { blueprint } }

    pub fn send_loose(blueprints: Vec<ResponseBlueprint<'a>>) -> Self { Self::SendLoose { blueprints } }

    pub fn send_loose_and_delete(blueprints: Vec<ResponseBlueprint<'a>>) -> Self {
        Self::SendLooseAndDelete { blueprints }
    }

    pub fn send_loose_and_delete_with_original(blueprints: Vec<ResponseBlueprint<'a>>) -> Vec<Self> {
        vec![Self::delete_original(), Self::send_loose_and_delete(blueprints)]
    }

    pub fn update(blueprint: ResponseBlueprint<'a>) -> Self { Self::Update { blueprint } }

    pub fn update_delayless(blueprint: ResponseBlueprint<'a>) -> Self { Self::UpdateDelayless { blueprint } }
}

#[derive(Clone, Debug, Default)]
pub struct ResponseBlueprint<'a> {
    // Missing attachments and allowed_mentions
    pub new_content:    Option<Cow<'a, str>>,
    pub new_embeds:     Cow<'a, [CreateEmbed<'a>]>,
    pub new_components: Cow<'a, [CreateActionRow<'a>]>,
}
impl<'a> ResponseBlueprint<'a> {
    pub fn new() -> Self { Self::default() }

    pub fn add_embed(mut self, new_embed: CreateEmbed<'a>) -> Self {
        self.new_embeds.to_mut().push(new_embed);
        self
    }

    pub fn add_buttons(mut self, new_buttons: Vec<CreateButton<'a>>) -> Self {
        self.new_components.to_mut().push(CreateActionRow::Buttons(new_buttons));
        self
    }

    pub fn set_content(mut self, new_content: impl Into<Cow<'a, str>>) -> Self {
        self.new_content = Some(new_content.into());
        self
    }

    pub fn set_embeds(mut self, new_embeds: impl Into<Cow<'a, [CreateEmbed<'a>]>>) -> Self {
        self.new_embeds = new_embeds.into();
        self
    }

    pub fn set_components(mut self, new_components: impl Into<Cow<'a, [CreateActionRow<'a>]>>) -> Self {
        self.new_components = new_components.into();
        self
    }
}
impl<'a> ResponseBlueprint<'a> {
    pub fn create_interaction_response_message(&self) -> CreateInteractionResponseMessage<'a> {
        let mut into = CreateInteractionResponseMessage::new()
            .embeds(self.new_embeds.clone())
            .components(self.new_components.clone());

        if let Some(new_content) = &self.new_content {
            into = into.content(new_content.clone());
        }

        into
    }

    pub fn create_message(&self) -> CreateMessage<'a> {
        let mut into =
            CreateMessage::new().embeds(self.new_embeds.clone()).components(self.new_components.clone());

        if let Some(new_content) = &self.new_content {
            into = into.content(new_content.clone());
        }

        into
    }
}

pub fn simple_send<'a>(new_content: impl Into<Cow<'a, str>>) -> Result<Vec<Response<'a>>> {
    Ok(vec![Response::Send {
        blueprints: vec![ResponseBlueprint { new_content: Some(new_content.into()), ..Default::default() }],
    }])
}

pub fn simple_send_and_delete<'a>(new_content: impl Into<Cow<'a, str>>) -> Result<Vec<Response<'a>>> {
    Ok(vec![Response::SendAndDelete {
        blueprints: vec![ResponseBlueprint { new_content: Some(new_content.into()), ..Default::default() }],
    }])
}

pub fn simple_send_and_delete_with_original<'a>(
    new_content: impl Into<Cow<'a, str>>,
) -> Result<Vec<Response<'a>>> {
    Ok(Response::send_and_delete_with_original(vec![ResponseBlueprint {
        new_content: Some(new_content.into()),
        ..Default::default()
    }]))
}
