use crate::prelude::*;

pub type ResponseResult = Result<(Vec<ResponseBlueprint>, ResponseMode)>;

#[derive(Clone, Default)]
pub struct ResponseBlueprint {
    pub content: Option<Cow<'static, str>>,
    pub embeds: Cow<'static, [CreateEmbed<'static>]>,
    pub attachments: Vec<CreateAttachment<'static>>,
    pub ephemeral: bool,
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
            .ephemeral(value.ephemeral)
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

pub enum ResponseMode {
    Normal,
    Delete,
}

pub fn simple_response(content: impl Into<Cow<'static, str>>, mode: ResponseMode) -> ResponseResult {
    Ok((vec![
        ResponseBlueprint {
            content: Some(content.into()),
           ..Default::default()
        },
    ], mode))
}
