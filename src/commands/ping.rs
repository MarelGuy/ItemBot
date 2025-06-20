use serenity::model::application::ResolvedOption;
use serenity::{
    all::{CreateInteractionResponse, CreateInteractionResponseMessage},
    builder::CreateCommand,
};

pub fn run(_options: &[ResolvedOption]) -> CreateInteractionResponse {
    CreateInteractionResponse::Message(
        CreateInteractionResponseMessage::new()
            .content("pong!")
            .ephemeral(true),
    )
}

pub fn register() -> CreateCommand {
    CreateCommand::new("ping").description("A ping command")
}
