use serenity::{
    all::{CreateActionRow, CreateButton, CreateEmbed},
    model::application::ResolvedOption,
};
use serenity::{
    all::{CreateInteractionResponse, CreateInteractionResponseMessage},
    builder::CreateCommand,
};
const LIST_BUTTON: (&str, &str) = ("list_button", "Lista");
const CREATE_BUTTON: (&str, &str) = ("create_button", "Crea");
const DELETE_BUTTON: (&str, &str) = ("delete_button", "Elimina");
const UPDATE_BUTTON: (&str, &str) = ("update_button", "Aggiorna");

pub fn run(
    _options: &[ResolvedOption],
    is_back: bool,
    deleted_item: &str,
) -> CreateInteractionResponse {
    let embed = if deleted_item.is_empty() {
        CreateEmbed::new()
            .title("Menu")
            .description("Scegli un'azione:")
            .color(0x00_FF00)
    } else {
        CreateEmbed::new()
            .title("Oggetto Eliminato")
            .description(format!(
                "L'oggetto '{deleted_item}' è stato eliminato con successo."
            ))
            .color(0x00_FF00)
    };

    let action_row: CreateActionRow = CreateActionRow::Buttons(vec![
        CreateButton::new(LIST_BUTTON.0).label(LIST_BUTTON.1),
        CreateButton::new(CREATE_BUTTON.0).label(CREATE_BUTTON.1),
        CreateButton::new(DELETE_BUTTON.0).label(DELETE_BUTTON.1),
        CreateButton::new(UPDATE_BUTTON.0).label(UPDATE_BUTTON.1),
    ]);

    let message: CreateInteractionResponseMessage = CreateInteractionResponseMessage::new()
        .embed(embed)
        .ephemeral(true)
        .components(vec![action_row]);

    if is_back {
        CreateInteractionResponse::UpdateMessage(message)
    } else {
        CreateInteractionResponse::Message(message)
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("menu").description("A menu command")
}
