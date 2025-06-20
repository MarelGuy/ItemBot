use serenity::all::{Context, CreateInteractionResponse, Interaction};

use crate::commands::menu;

pub async fn back_button(ctx: &Context, interaction: &Interaction) {
    let message: CreateInteractionResponse = menu::run(&[], true, "");

    if let Some(interaction) = interaction.as_message_component() {
        interaction
            .create_response(&ctx.http, message)
            .await
            .unwrap();
    }
}
