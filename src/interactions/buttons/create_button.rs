use serenity::all::{
    Context, CreateActionRow, CreateInputText, CreateInteractionResponse, CreateModal, Interaction,
};

pub async fn create_button(ctx: &Context, interaction: &Interaction) {
    let modal: CreateModal = CreateModal::new("create_item_modal", "Crea un nuovo oggetto")
        .components(vec![CreateActionRow::InputText(CreateInputText::new(
            serenity::all::InputTextStyle::Short,
            "Nome dell'oggetto",
            "item_name",
        ))]);

    let response = CreateInteractionResponse::Modal(modal);

    if let Some(interaction) = interaction.as_message_component() {
        interaction
            .create_response(&ctx.http, response)
            .await
            .unwrap();
    }
}
