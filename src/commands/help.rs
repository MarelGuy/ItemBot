use serenity::model::application::ResolvedOption;
use serenity::{
    all::{CreateInteractionResponse, CreateInteractionResponseMessage},
    builder::CreateCommand,
};

pub fn run(_options: &[ResolvedOption]) -> CreateInteractionResponse {
    CreateInteractionResponse::Message(
        CreateInteractionResponseMessage::new()
            .content("")
            .ephemeral(true),
    )
}

pub fn register() -> CreateCommand {
    CreateCommand::new("help").description("A help command")
}
