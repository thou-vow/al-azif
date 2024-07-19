use serenity::all::*;

pub fn copy_button<'a>(button: &Button) -> CreateButton<'a> {
    let mut copy = match &button.data {
        ButtonKind::Link { url } => CreateButton::new_link(url.clone()),
        ButtonKind::NonLink { custom_id, style } => {
            CreateButton::new(custom_id.clone()).style(*style)
        }
    }
    .disabled(button.disabled);

    if let Some(emoji) = &button.emoji {
        copy = copy.emoji(emoji.clone());
    }

    if let Some(label) = &button.label {
        copy = copy.label(label.clone());
    }

    copy
}
