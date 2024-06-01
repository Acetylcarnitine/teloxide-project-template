use std::sync::Arc;
use sqlx::{Pool, Postgres};
use teloxide::prelude::*;
use teloxide::RequestError;
use crate::database::traits::reader::DefaultReader;
use crate::database::types::user::User;
use crate::keyboards::personal_account_keyboards::{
    back_to_account_kb,
    personal_account_kb
};
use crate::utils::callback_parsed::CallbackParsed;

pub async fn to_personal_account(
    bot: Bot,
    call: CallbackQuery,
    pool: Arc<Pool<Postgres>>
) -> Result<(), RequestError> {
    let req = User::read_one(
        pool, call.from.id.0 as i64
    ).await;
    if req.is_err() {
        panic!("user with this id not found");
    } else {
        let user  = req.unwrap();
        let text = format!(
            "It's your personal account\nYour id is {}", user.id
        );
        bot.edit_message_text(
            call.from.id,
            call.message.unwrap().id,
            text
        )
            .reply_markup(personal_account_kb())
            .await?;
    }

    Ok(())
}

pub async fn show_deposit(
    bot: Bot,
    call: CallbackQuery,
    pool: Arc<Pool<Postgres>>
) -> Result<(), RequestError> {
    let req = User::read_one(
        pool, call.from.id.0 as i64
    ).await;
    if req.is_err() {
        panic!("User not found")
    } else {
        let user = req.unwrap();
        let text = format!("Your deposit is {}", user.deposit);
        bot.edit_message_text(
            call.from.id,
            call.message.unwrap().id,
            text
        )
            .reply_markup(back_to_account_kb())
            .await?;
    }

    Ok(())
}

pub async fn show_rating(
    bot: Bot,
    call: CallbackQuery,
    pool: Arc<Pool<Postgres>>
) -> Result<(), RequestError> {
    let req = User::read_one(
        pool, call.from.id.0 as i64
    ).await;
    if req.is_err() {
        panic!("User not found")
    } else {
        let user = req.unwrap();
        let text = format!("Your rating is {}", user.rating);
        bot.edit_message_text(
            call.from.id,
            call.message.unwrap().id,
            text
        )
            .reply_markup(back_to_account_kb())
            .await?;
    }

    Ok(())
}

// connect routes to account service
pub async fn account_routes(
    bot: Bot,
    parsed: &CallbackParsed,
    call: CallbackQuery,
    pool: Arc<Pool<Postgres>>
) -> Result<(), RequestError> {
    if parsed.route.is_none() {
        to_personal_account(bot, call, pool).await?;
    } else {
        let route = parsed.route.clone();
        match route.unwrap().as_str() {
            "/deposit" => { show_deposit(bot, call, pool).await?; },
            "/rating" => { show_rating(bot, call, pool).await?; },
            _ => {}
        };
    }

    Ok(())
}