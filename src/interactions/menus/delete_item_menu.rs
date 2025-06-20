use serenity::all::{
    ComponentInteractionData, ComponentInteractionDataKind, Context, CreateInteractionResponse,
    Interaction,
};

use crate::{Database, commands::menu};

pub async fn delete_item_menu(ctx: &Context, interaction: &Interaction, database: &mut Database) {
    if let Some(component_interaction) = interaction.as_message_component() {
        let value: &ComponentInteractionData = &component_interaction.data;

        let item_name: String =
            if let ComponentInteractionDataKind::StringSelect { values } = &value.kind {
                values.first().cloned().unwrap_or_default()
            } else {
                return;
            };

        database
            .delete_item_by_name(item_name.clone())
            .await
            .unwrap();

        let menu: CreateInteractionResponse = menu::run(&[], true, &item_name);

        if let Err(why) = component_interaction.create_response(&ctx.http, menu).await {
            tracing::error!("Failed to respond to interaction: {}", why);
        }
    }
}
