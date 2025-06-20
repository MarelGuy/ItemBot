use serenity::all::{
    Context, CreateActionRow, CreateButton, CreateEmbed, CreateInteractionResponse,
    CreateInteractionResponseMessage, CreateSelectMenu, CreateSelectMenuKind,
    CreateSelectMenuOption, Interaction,
};

use crate::{Database, database::Item};

pub async fn delete_button(ctx: &Context, interaction: &Interaction, database: &mut Database) {
    let items: Vec<Item> = database.get_all_items().await.unwrap();

    if let Some(message_component) = interaction.as_message_component() {
        let menu: bool = items.is_empty();

        let embed: CreateEmbed = if menu {
            CreateEmbed::new()
                .title("Errore")
                .description("Nessun oggetto trovato.")
                .color(0xFF_0000)
        } else {
            CreateEmbed::new()
                .title("Seleziona un oggetto da eliminare")
                .description("Scegli un oggetto dalla lista per eliminarlo.")
                .color(0x00_FF00)
        };

        let message: CreateInteractionResponseMessage = if menu {
            CreateInteractionResponseMessage::new()
                .embed(embed)
                .ephemeral(true)
        } else {
            let menu: CreateSelectMenu = CreateSelectMenu::new(
                "delete_item_menu",
                CreateSelectMenuKind::String {
                    options: items
                        .iter()
                        .map(|item| {
                            CreateSelectMenuOption::new(item.name.clone(), item.name.clone())
                                .description(format!("Count: {}", item.count))
                        })
                        .collect(),
                },
            );

            let back_button: CreateButton = CreateButton::new("back_button").label("Indietro");

            CreateInteractionResponseMessage::new()
                .embed(embed)
                .components(vec![
                    CreateActionRow::SelectMenu(menu),
                    CreateActionRow::Buttons(vec![back_button]),
                ])
                .ephemeral(true)
        };

        if let Err(why) = message_component
            .create_response(&ctx.http, CreateInteractionResponse::UpdateMessage(message))
            .await
        {
            println!("Cannot respond to interaction: {why}");
        }
    }
}
