use crate::prelude::*;

#[derive(Debug)]
pub enum ResponseModel {
    Send { blueprints: Vec<ResponseBlueprint> },
    // Only for interactions
    SendEphemeral { blueprint: ResponseBlueprint },
    SendLoose { blueprints: Vec<ResponseBlueprint> },
    // Only for component-based interactions
    Update { blueprint: ResponseBlueprint },
}
impl ResponseModel {
    pub fn send(blueprints: Vec<ResponseBlueprint>) -> Self {
        Self::Send { blueprints }
    }
    pub fn send_ephemeral(blueprint: ResponseBlueprint) -> Self {
        Self::SendEphemeral { blueprint }
    }
    pub fn send_loose(blueprints: Vec<ResponseBlueprint>) -> Self {
        Self::SendLoose { blueprints }
    }
    pub fn update(blueprint: ResponseBlueprint) -> Self {
        Self::Update { blueprint }
    }
}

#[derive(Clone, Debug, Default)]
pub struct ResponseBlueprint {
    pub content: Option<Cow<'static, str>>,
    pub embeds: Cow<'static, [CreateEmbed<'static>]>,
    pub attachments: Vec<CreateAttachment<'static>>,
    pub components: Cow<'static, [CreateActionRow<'static>]>,
    pub allowed_mentions: Option<CreateAllowedMentions<'static>>,
}
impl ResponseBlueprint {
    pub fn content(mut self, content: impl Into<Cow<'static, str>>) -> Self {
        self.content = Some(content.into());
        self
    }
    pub fn embeds(mut self, embeds: impl Into<Cow<'static, [CreateEmbed<'static>]>>) -> Self {
        self.embeds = embeds.into();
        self
    }
    pub fn components(mut self, components: impl Into<Cow<'static, [CreateActionRow<'static>]>>) -> Self {
        self.components = components.into();
        self
    }
}
impl From<ResponseBlueprint> for CreateInteractionResponseMessage<'static> {
    fn from(value: ResponseBlueprint) -> Self {
        let mut into = Self::default()
            .embeds(value.embeds)
            .add_files(value.attachments)
            .components(value.components);

        if let Some(content) = value.content {
            into = into.content(content);
        }
        if let Some(allowed_mentions) = value.allowed_mentions {
            into = into.allowed_mentions(allowed_mentions);
        }

        into
    }
}
impl From<ResponseBlueprint> for CreateMessage<'static> {
    fn from(value: ResponseBlueprint) -> Self {
        let mut into = Self::default()
            .embeds(value.embeds)
            .add_files(value.attachments)
            .components(value.components);

        if let Some(content) = value.content {
            into = into.content(content);
        }
        if let Some(allowed_mentions) = value.allowed_mentions {
            into = into.allowed_mentions(allowed_mentions);
        }

        into
    }
}

pub fn simple_send_response(content: impl Into<Cow<'static, str>>, ephemeral: bool) -> Result<Vec<ResponseModel>> {
    if ephemeral {
        Ok(vec![ResponseModel::SendEphemeral { 
            blueprint: ResponseBlueprint {
                content: Some(content.into()),
               ..Default::default()
            }
        }])
    } else {
        Ok(vec![ResponseModel::Send {
            blueprints: vec![ResponseBlueprint {
                content: Some(content.into()),
               ..Default::default()
            }]
        }])
    }
}
