use std::sync::Arc;
use sqlx::{Pool, Postgres};
use teloxide::RequestError;
use teloxide::payloads::SendMessageSetters;
use teloxide::prelude::{Bot, Message, Requester};
use teloxide::utils::command::BotCommands;
use crate::database::types::user::User;
use crate::keyboards::base_command_keyboards::start_kb;

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "Bot support next commands:"
)]
pub enum BaseCommand {
    #[command(description = "Start or restart the bot")]
    Start,
    #[command(description = "Display this message")]
    Help
}

pub async fn base_command_handler(
    bot: Bot,
    msg: Message,
    cmd: BaseCommand,
    pool: Arc<Pool<Postgres>>
) -> Result<(), RequestError> {
    match cmd {
        BaseCommand::Start => {
            let _ = User::save(msg.from().unwrap().id.0 as i64, pool).await;
            bot.send_message(msg.chat.id, "This is start message")
                .reply_markup(start_kb())
                .await?;
        },
        BaseCommand::Help => {
            bot.send_message(msg.chat.id, BaseCommand::descriptions().to_string()).await?;
        }
    }

    Ok(())
}