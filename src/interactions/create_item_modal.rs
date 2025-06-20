use serenity::all::{
    ActionRowComponent, Context, CreateEmbed, CreateInteractionResponse,
    CreateInteractionResponseMessage, Interaction, ModalInteractionData,
};

use crate::Database;

pub async fn create_item_modal(ctx: &Context, interaction: &Interaction, database: &mut Database) {
    if let Interaction::Modal(modal_interaction) = interaction {
        let data: ModalInteractionData = modal_interaction.data.clone();

        for row in data.components {
            for component in row.components {
                if let ActionRowComponent::InputText(text_input) = component {
                    if text_input.custom_id != "item_name" {
                        continue;
                    }

                    let value: String = text_input.value.unwrap();

                    let embed: CreateEmbed = if database
                        .get_item_by_name(value.clone())
                        .await
                        .unwrap()
                        .is_some()
                    {
                        CreateEmbed::new()
                            .title("Errore")
                            .description("Questo oggetto esiste già!")
                            .color(0xFF_0000)
                    } else {
                        let item_name: String = value;

                        database.add_item(item_name.clone(), 0).await;

                        CreateEmbed::new()
                            .title("Item")
                            .description(format!("Item '{item_name}' creato!"))
                            .color(0x00_FF00)
                    };

                    let message: CreateInteractionResponseMessage =
                        CreateInteractionResponseMessage::new().embed(embed);

                    modal_interaction
                        .create_response(
                            &ctx.http,
                            CreateInteractionResponse::UpdateMessage(message),
                        )
                        .await
                        .unwrap();

                    return;
                }
            }
        }
    }
}
