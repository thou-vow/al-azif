use crate::prelude::*;

pub type Blueprints<'a> = Vec<ResponseBlueprint<'a>>;
pub type Models<'a> = Vec<ResponseModel<'a>>;

#[derive(Debug)]
pub enum ResponseModel<'a> {
    Send {
        blueprints: Vec<ResponseBlueprint<'a>>,
    },
    // Only for interactions
    SendEphemeral {
        blueprint: ResponseBlueprint<'a>,
    },
    SendLoose {
        blueprints: Vec<ResponseBlueprint<'a>>,
    },
    // Only for component-based interactions
    Update {
        blueprint: ResponseBlueprint<'a>,
    },
}
impl<'a> ResponseModel<'a> {
    pub fn send(blueprints: Vec<ResponseBlueprint<'a>>) -> Self {
        Self::Send { blueprints }
    }
    pub fn send_ephemeral(blueprint: ResponseBlueprint<'a>) -> Self {
        Self::SendEphemeral { blueprint }
    }
    pub fn send_loose(blueprints: Vec<ResponseBlueprint<'a>>) -> Self {
        Self::SendLoose { blueprints }
    }
    pub fn update(blueprint: ResponseBlueprint<'a>) -> Self {
        Self::Update { blueprint }
    }
}

#[derive(Clone, Debug, Default)]
pub struct ResponseBlueprint<'a> {
    // Missing attachments and allowed_mentions
    pub content: Option<Cow<'a, str>>,
    pub embeds: Cow<'a, [CreateEmbed<'a>]>,
    pub components: Cow<'a, [CreateActionRow<'a>]>,
}
impl<'a> ResponseBlueprint<'a> {
    pub fn assign_content(mut self, content: impl Into<Cow<'a, str>>) -> Self {
        self.content = Some(content.into());
        self
    }
    pub fn assign_embeds(mut self, embeds: impl Into<Cow<'a, [CreateEmbed<'a>]>>) -> Self {
        self.embeds = embeds.into();
        self
    }
    pub fn assign_components(
        mut self,
        components: impl Into<Cow<'a, [CreateActionRow<'a>]>>,
    ) -> Self {
        self.components = components.into();
        self
    }
}
impl<'a> ResponseBlueprint<'a> {
    pub fn get_mut_button(&mut self, row: usize, column: usize) -> Option<&mut CreateButton<'a>> {
        match self.components.to_mut().get_mut(row) {
            Some(CreateActionRow::Buttons(buttons)) => buttons.get_mut(column),
            _ => None,
        }
    }
    pub fn get_mut_embed(&mut self, index: usize) -> Option<&mut CreateEmbed<'a>> {
        self.embeds.to_mut().get_mut(index)
    }
}

impl<'a> From<ResponseBlueprint<'a>> for CreateInteractionResponseMessage<'a> {
    fn from(value: ResponseBlueprint<'a>) -> Self {
        let mut into = Self::default()
            .embeds(value.embeds)
            .components(value.components);

        if let Some(content) = value.content {
            into = into.content(content);
        }

        into
    }
}

impl<'a> From<ResponseBlueprint<'a>> for CreateMessage<'a> {
    fn from(value: ResponseBlueprint<'a>) -> Self {
        let mut into = Self::default()
            .embeds(value.embeds)
            .components(value.components);

        if let Some(content) = value.content {
            into = into.content(content);
        }

        into
    }
}

pub fn simple_send<'a>(content: impl Into<Cow<'a, str>>) -> Result<Vec<ResponseModel<'a>>> {
    Ok(vec![ResponseModel::Send {
        blueprints: vec![ResponseBlueprint {
            content: Some(content.into()),
            ..Default::default()
        }],
    }])
}
