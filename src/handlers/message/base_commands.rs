use teloxide::RequestError;
use teloxide::payloads::SendMessageSetters;
use teloxide::prelude::{Bot, Message, Requester};
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};
use teloxide::utils::command::BotCommands;

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

// remove this into keyboards for production
fn make_kb() -> InlineKeyboardMarkup {
    let kb = vec![vec![InlineKeyboardButton::callback("button", "aboba")]];
    InlineKeyboardMarkup::new(kb)
}

pub async fn base_command_handler(
    bot: Bot,
    msg: Message,
    cmd: BaseCommand,
) -> Result<(), RequestError> {
    match cmd {
        BaseCommand::Start => {
            bot.send_message(msg.chat.id, "This is start message")
                .reply_markup(make_kb())
                .await?;
        },
        BaseCommand::Help => {
            bot.send_message(msg.chat.id, BaseCommand::descriptions().to_string()).await?;
        }
    }

    Ok(())
}