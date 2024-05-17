use teloxide::{Bot, RequestError};
use teloxide::prelude::{CallbackQuery, Requester};
use crate::utils::callback_parsed::CallbackParsed;

pub async fn route_callback_query(
    bot: Bot,
    call: CallbackQuery
) -> Result<(), RequestError> {
    bot.answer_callback_query(call.id).await?;
    let parsed = CallbackParsed::parse(&call.data.unwrap());
    // debug only
    println!("Success parse {:?}", parsed);
    // match base links and then routing on them
    todo!();

    Ok(())
}