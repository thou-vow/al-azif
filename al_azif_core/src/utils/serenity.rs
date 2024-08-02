use crate::_prelude::*;

pub fn copy_button<'a>(button: &Button) -> CreateButton<'a> {
    let mut copy = match &button.data {
        ButtonKind::Link { url } => CreateButton::new_link(url.clone()),
        ButtonKind::NonLink { custom_id, style } => CreateButton::new(custom_id.clone()).style(*style),
        ButtonKind::Premium { sku_id } => CreateButton::new_premium(sku_id),
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
