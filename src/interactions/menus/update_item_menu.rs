use serenity::all::{
    ComponentInteractionData, ComponentInteractionDataKind, Context, CreateActionRow, CreateButton,
    CreateEmbed, CreateInteractionResponse, CreateInteractionResponseMessage, Interaction,
};

pub async fn update_item_menu(ctx: &Context, interaction: &Interaction) {
    if let Some(component_interaction) = interaction.as_message_component() {
        let value: &ComponentInteractionData = &component_interaction.data;

        let item_name: String =
            if let ComponentInteractionDataKind::StringSelect { values } = &value.kind {
                values.first().cloned().unwrap_or_default()
            } else {
                return;
            };

        let plus_button: CreateButton =
            CreateButton::new(format!("plus_button_{item_name}")).label("+");
        let minus_button: CreateButton =
            CreateButton::new(format!("minus_button_{item_name}")).label("-");

        let back_button: CreateButton = CreateButton::new("back_button").label("Indietro");

        let embed: CreateEmbed = CreateEmbed::new()
            .title("Aggiorna")
            .description(format!("Hai selezionato l'item: {item_name}"))
            .color(0x00_FF00);

        let message: CreateInteractionResponseMessage = CreateInteractionResponseMessage::new()
            .embed(embed)
            .components(vec![CreateActionRow::Buttons(vec![
                plus_button,
                minus_button,
                back_button,
            ])]);

        if let Err(why) = component_interaction
            .create_response(&ctx.http, CreateInteractionResponse::UpdateMessage(message))
            .await
        {
            tracing::error!("Failed to respond to interaction: {}", why);
        }
    }
}
