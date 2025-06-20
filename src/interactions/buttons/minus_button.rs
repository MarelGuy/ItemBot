use serenity::all::{
    Context, CreateEmbed, CreateInteractionResponse, CreateInteractionResponseMessage, Interaction,
};

use crate::{Database, database::Item};

pub async fn minus_button(
    ctx: &Context,
    interaction: &Interaction,
    database: &mut Database,
    item: &str,
) {
    let item: Item = database.update_item_by_name(item, 1, "-").await.unwrap();

    if let Some(message_component) = interaction.as_message_component() {
        let embed: CreateEmbed = CreateEmbed::new()
            .title("Aggiornato")
            .description(format!("{}: {}.", item.name, item.count))
            .color(0x00_FF00);

        let message = message_component
            .create_response(
                &ctx.http,
                CreateInteractionResponse::UpdateMessage(
                    CreateInteractionResponseMessage::new()
                        .embed(embed)
                        .ephemeral(true),
                ),
            )
            .await;

        if let Err(why) = message {
            tracing::error!("Failed to respond to interaction: {}", why);
        }
    }
}
