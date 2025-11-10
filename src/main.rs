mod accounting_client_api;
mod services;
mod dto;

use teloxide::{prelude::*, utils::command::BotCommands};
use dotenvy::dotenv;
use std::env;
use std::sync::Arc;
use crate::services::user_service::{SessionStore, SessionStoreInMemory};

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "Why can't you be a normal wife?")]
    Help,
    #[command(description = "handle a username.")]
    Username(String),
    #[command(description = "handle a username and an age.", parse_with = "split")]
    UsernameAndAge { username: String, age: u8 },
}

async fn answer(bot: Bot, msg: Message, cmd: Command, session_store: Arc<SessionStoreInMemory>) -> ResponseResult<()> {
    let user = msg.from.unwrap();
    let username = user.username.unwrap();
    let session = session_store.get_or_create(
        user.id.to_string(),
        username,
        msg.chat.id.to_string(),
    ).await;
    
    match cmd {
        Command::Help => bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?,
        Command::Username(_) => {
            let username = session.username.clone();
            bot.send_message(msg.chat.id, format!("Your username is @{username}.")).await?
        }
        Command::UsernameAndAge { username, age } => {
            bot.send_message(msg.chat.id, format!("Your username is @{username} and age is {age}."))
                .await?
        }
    };

    Ok(())
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Загружаем .env только если файл существует
    if std::path::Path::new(".env").exists() {
        dotenv().ok();
    }

    let session_store = Arc::new(SessionStoreInMemory::new());

    let bot_token = env::var("BOT_TOKEN").expect("BOT_TOKEN not set");
    log::info!("Starting throw dice bot...");
    let bot = Bot::new(bot_token);

    Command::repl(bot, move |bot, msg, cmd| {
        let session_store = session_store.clone(); // передаём копию Arc внутрь async функции
        async move { answer(bot, msg, cmd, session_store).await }
    })
        .await;
    
    Ok(())
}
