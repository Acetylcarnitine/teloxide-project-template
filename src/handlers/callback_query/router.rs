use std::sync::Arc;
use sqlx::{Pool, Postgres};
use teloxide::{Bot, RequestError};
use teloxide::prelude::{CallbackQuery, Requester};
use crate::handlers::callback_query::account_service::account_routes;
use crate::handlers::callback_query::balance_service::balance_routes;
use crate::utils::callback_parsed::CallbackParsed;

pub async fn route_callback_query(
    bot: Bot,
    call: CallbackQuery,
    pool: Arc<Pool<Postgres>>
) -> Result<(), RequestError> {
    bot.answer_callback_query(call.clone().id).await?;
    let parsed = CallbackParsed::parse(&call.data.clone().unwrap());
    // debug only
    println!("Success parse {:?}", parsed);
    // match base links and then routing on them to services
    match parsed.base.as_str() {
        "account" => { account_routes(bot, &parsed, call.clone(), pool).await? },
        "balance" => { balance_routes(bot, &parsed, call.clone(), pool).await? }
        _ => {}
    };

    Ok(())
}