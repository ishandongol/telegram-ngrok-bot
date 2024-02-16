use teloxide::utils::command::BotCommands;

#[derive(BotCommands, Clone, Debug)]
#[command(
    rename_rule = "lowercase",
    description = "Welcome to Yomari Bot. Here are the available commands:"
)]
pub enum TelegramBotCommand {
    #[command(description = "You already know what this does.")]
    Help,
    #[command(description = "Get the ngrok tunnel info.")]
    Info,
}
