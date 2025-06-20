mod commands;
mod database;
mod interactions;

use std::env;

use serenity::{
    all::Command,
    async_trait,
    builder::{CreateInteractionResponse, CreateInteractionResponseMessage},
    model::{application::Interaction, gateway::Ready},
    prelude::*,
};
use tracing::{error, info};
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

use crate::{
    database::Database,
    interactions::{
        back_button, create_button, create_item_modal, delete_button, delete_item_menu,
        list_button, minus_button, plus_button, update_button, update_item_menu,
    },
};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = &interaction {
            let content: Option<CreateInteractionResponse> = match command.data.name.as_str() {
                "ping" => Some(commands::ping::run(&command.data.options())),
                "help" => Some(commands::help::run(&command.data.options())),
                "menu" => Some(commands::menu::run(&command.data.options(), false, "")),
                _ => Some(CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new()
                        .content("Questo comando non esiste!")
                        .ephemeral(true),
                )),
            };

            if let Some(content) = content {
                if let Err(why) = command.create_response(&ctx.http, content).await {
                    error!("Cannot respond to slash command: {why}");
                }
            }
        }

        if let Interaction::Component(component_interaction) = &interaction {
            let custom_id: &str = component_interaction.data.custom_id.as_str();

            let mut db: Database = Database::new().await;

            match custom_id {
                "list_button" => list_button(&ctx, &interaction, &mut db).await,
                "create_button" => create_button(&ctx, &interaction).await,
                "delete_button" => delete_button(&ctx, &interaction, &mut db).await,
                "update_button" => update_button(&ctx, &interaction, &mut db).await,
                "back_button" => back_button(&ctx, &interaction).await,
                "delete_item_menu" => delete_item_menu(&ctx, &interaction, &mut db).await,
                "update_item_menu" => update_item_menu(&ctx, &interaction).await,
                _ => {}
            }

            if let Some(item) = custom_id.strip_prefix("plus_button_") {
                plus_button(&ctx, &interaction, &mut db, item).await;
            }

            if let Some(item) = custom_id.strip_prefix("minus_button_") {
                minus_button(&ctx, &interaction, &mut db, item).await;
            }
        }

        if let Interaction::Modal(modal_interaction) = &interaction {
            let custom_id: &str = modal_interaction.data.custom_id.as_str();

            let mut db: Database = Database::new().await;

            if custom_id == "create_item_modal" {
                create_item_modal(&ctx, &interaction, &mut db).await;
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);

        if let Err(why) = Command::set_global_commands(
            &ctx.http,
            vec![
                commands::ping::register(),
                commands::help::register(),
                commands::menu::register(),
            ],
        )
        .await
        {
            error!("Failed to register commands: {why}");
        } else {
            info!("Global commands registered successfully.");
        }
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(EnvFilter::from_default_env())
        .with(fmt::layer().with_file(true).with_line_number(true))
        .init();

    let token: String = match env::var("DISCORD_TOKEN") {
        Ok(token) => token,
        Err(e) => {
            error!("Expected a token in the environment: {e}");

            return;
        }
    };

    let mut client: Client = match Client::builder(token, GatewayIntents::empty())
        .event_handler(Handler)
        .await
    {
        Ok(client) => client,
        Err(e) => {
            error!("Error creating client: {e}");

            return;
        }
    };

    if let Err(why) = client.start().await {
        error!("Client error: {why:?}");
    }
}
