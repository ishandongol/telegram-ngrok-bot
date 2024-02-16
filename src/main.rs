use clap::Parser;
use telegram_ngrok_bot::{
    bot_command::TelegramBotCommand,
    cli::Cli,
    models::config::{BotConfig, NgrokConfig},
    ngrok_api::NgrokApi,
};
use teloxide::{prelude::*, utils::command::BotCommands, Bot};

type HandlerResult = Result<(), String>;

#[tokio::main]
async fn main() {
    let (bot_config, ngrok_config) = parse_configs();
    let bot = Bot::new(bot_config.token.clone());
    let client = reqwest::Client::new();
    let ngrok_api = NgrokApi::new(ngrok_config, client);
    let command_handler = teloxide::filter_command::<TelegramBotCommand, _>()
        .branch(dptree::case![TelegramBotCommand::Help].endpoint(help_command_handler))
        .branch(dptree::case![TelegramBotCommand::Info].endpoint(info_command_handler));
    let message_handler = Update::filter_message().branch(command_handler);
    bot.send_message(bot_config.chat_id, "🥑").await.unwrap();
    send_ngrok_info(&ngrok_api, &bot_config, &bot)
        .await
        .unwrap();
    Dispatcher::builder(bot, message_handler)
        .dependencies(dptree::deps![ngrok_api, bot_config])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}

async fn send_ngrok_info(
    ngrok_api: &NgrokApi,
    bot_config: &BotConfig,
    bot: &Bot,
) -> Result<(), String> {
    let tunnel_info = ngrok_api.get_tunnel().await;
    match tunnel_info {
        Ok(tunnels) => {
            for (index, tunnel) in tunnels.iter().enumerate() {
                bot.send_message(
                    bot_config.chat_id,
                    format!("Tunnel Info {index}:\n{}", tunnel),
                )
                .await
                .map_err(|e| e.to_string())?;
            }
        }
        Err(e) => {
            bot.send_message(bot_config.chat_id, e)
                .await
                .map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}

async fn info_command_handler(
    bot_config: BotConfig,
    ngrok_api: NgrokApi,
    bot: Bot,
) -> HandlerResult {
    send_ngrok_info(&ngrok_api, &bot_config, &bot).await?;
    Ok(())
}
async fn help_command_handler(bot_config: BotConfig, bot: Bot) -> HandlerResult {
    bot.send_message(
        bot_config.chat_id,
        TelegramBotCommand::descriptions().to_string(),
    )
    .await
    .map_err(|e| e.to_string())?;
    Ok(())
}
fn parse_configs() -> (BotConfig, NgrokConfig) {
    let cli = Cli::parse();
    let bot_config: BotConfig = BotConfig::from_file(cli.config).unwrap();
    let ngrok_config =
        NgrokConfig::from_file(cli.ngrok_config, bot_config.ngrok_api_url.clone()).unwrap();
    (bot_config, ngrok_config)
}
