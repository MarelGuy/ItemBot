use serenity::all::{
    Context, CreateActionRow, CreateEmbed, CreateInteractionResponse,
    CreateInteractionResponseMessage, CreateSelectMenu, CreateSelectMenuKind,
    CreateSelectMenuOption, Interaction,
};

use crate::{Database, database::Item};

pub async fn update_button(ctx: &Context, interaction: &Interaction, database: &mut Database) {
    let items: Vec<Item> = database.get_all_items().await.unwrap();

    if let Some(message_component) = interaction.as_message_component() {
        let is_menu: bool = items.is_empty();

        let menu: CreateSelectMenu = CreateSelectMenu::new(
            "update_item_menu",
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

        let embed: CreateEmbed = if is_menu {
            CreateEmbed::new()
                .title("Errore")
                .description("Nessun oggetto trovato.")
                .color(0xFF_0000)
        } else {
            CreateEmbed::new()
                .title("Aggiorna")
                .description("Scegli un oggetto dalla lista per aggiornarlo.")
                .color(0x00_FF00)
        };

        let message: CreateInteractionResponseMessage = if is_menu {
            CreateInteractionResponseMessage::new()
                .embed(embed)
                .ephemeral(true)
        } else {
            CreateInteractionResponseMessage::new()
                .embed(embed)
                .components(vec![CreateActionRow::SelectMenu(menu)])
                .ephemeral(true)
        };

        if let Err(why) = message_component
            .create_response(&ctx.http, CreateInteractionResponse::UpdateMessage(message))
            .await
        {
            tracing::error!("Failed to respond to interaction: {}", why);
        }
    }
}
