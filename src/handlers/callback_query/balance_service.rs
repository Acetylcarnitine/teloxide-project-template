use std::sync::Arc;
use sqlx::{Pool, Postgres};
use teloxide::prelude::*;
use teloxide::RequestError;
use crate::database::types::user::User;
use crate::keyboards::personal_account_keyboards::{back_to_account_kb, replenish_kb};
use crate::utils::callback_parsed::CallbackParsed;

pub async fn show_replenish_message(
    bot: Bot,
    call: CallbackQuery,
    value: i64
) -> Result<(), RequestError> {
    bot.edit_message_text(
        call.from.id,
        call.message.unwrap().id,
        format!("Current value is {value}")
    )
        .reply_markup(replenish_kb(value))
        .await?;

    Ok(())
}

pub async fn change_value(
    bot: Bot,
    call: CallbackQuery,
    parsed: &CallbackParsed
) -> Result<(), RequestError> {
    let added = parsed.get_arg::<i64>(0).unwrap();
    let mut current = parsed.get_arg::<i64>(1).unwrap() + added;
    if current < 0 {
        current = 0;
    }
    bot.edit_message_text(
        call.from.id,
        call.message.unwrap().id,
        format!("Current value is {current}")
    )
        .reply_markup(replenish_kb(current))
        .await?;

    Ok(())
}

pub async fn apply_replenish(
    bot: Bot,
    call: CallbackQuery,
    pool: Arc<Pool<Postgres>>,
    parsed: &CallbackParsed
) -> Result<(), RequestError> {
    let arg = parsed.get_arg::<i64>(0);
    if arg.is_err() {
        panic!("Parsed params is none");
    }
    let res = User::add_to_balance(
        call.from.id.0 as i64, arg.clone().unwrap(), pool
    ).await;
    let mut text = String::new();
    if res.is_err() {
        text = "Oops, we have some error with database".to_string()
    } else {
        text = format!("Value {} added to your deposit!", arg.unwrap());
    }
    bot.edit_message_text(
        call.from.id,
        call.message.unwrap().id,
        text
    )
        .reply_markup(back_to_account_kb())
        .await?;

    Ok(())
}

// connecting all routes to balance service
pub async fn balance_routes(
    bot: Bot,
    parsed: &CallbackParsed,
    call: CallbackQuery,
    pool: Arc<Pool<Postgres>>
) -> Result<(), RequestError> {
    if parsed.route.is_none() {
        let cur_value = parsed.get_arg::<i64>(0);
        show_replenish_message(bot, call, cur_value.unwrap()).await?;
    } else {
        let route = parsed.route.clone();
        match route.unwrap().as_str() {
            "/add" => { change_value(bot, call, parsed).await?; },
            "/apply" => { apply_replenish(bot, call, pool, parsed).await?; }
            _ => {}
        };
    }

    Ok(())
}