use serenity::all::{
    Context, CreateEmbed, CreateInteractionResponse, CreateInteractionResponseMessage, Interaction,
};

use std::{fmt::Write, io};

use crate::{Database, database::Item};

pub async fn list_button(ctx: &Context, interaction: &Interaction, database: &mut Database) {
    let items: Result<Vec<Item>, io::Error> = database.get_all_items().await;

    if let Some(message_component) = interaction.as_message_component() {
        let desc: String = if items.as_ref().unwrap().is_empty() {
            "Nessun oggetto trovato.".to_owned()
        } else {
            items.unwrap().iter().fold(String::new(), |mut acc, item| {
                writeln!(acc, "- {}: {}", item.name, item.count).unwrap();
                acc
            })
        };

        let embed: CreateEmbed = CreateEmbed::new()
            .title("Lista")
            .description(desc)
            .color(0x00_FF00);

        let message: CreateInteractionResponseMessage = CreateInteractionResponseMessage::new()
            .embed(embed)
            .ephemeral(true);

        if let Err(why) = message_component
            .create_response(&ctx.http, CreateInteractionResponse::UpdateMessage(message))
            .await
        {
            println!("Cannot respond to interaction: {why}");
        }
    }
}
