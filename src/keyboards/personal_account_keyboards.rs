use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};
use crate::utils::callback_builder::CallbackBuilder;

// for the personal account
pub fn personal_account_kb() -> InlineKeyboardMarkup {
    let callback = CallbackBuilder::new("balance")
        .bind(0)
        .build();

    let kb = vec![
        vec![
            InlineKeyboardButton::callback("deposit", "account/deposit"),
            InlineKeyboardButton::callback("rating", "account/rating")
        ],
        vec![InlineKeyboardButton::callback("balance", callback)]
    ];
    InlineKeyboardMarkup::new(kb)
}

// for returning to the personal account
pub fn back_to_account_kb() -> InlineKeyboardMarkup {
    let kb = vec![
        vec![InlineKeyboardButton::callback("back", "account")]
    ];
    InlineKeyboardMarkup::new(kb)
}

pub fn replenish_kb(cur_val: i64) -> InlineKeyboardMarkup {
    let add_callback = CallbackBuilder::new("balance")
        .route("/add")
        .bind(100)
        .bind(cur_val)
        .build();

    let minus_callback = CallbackBuilder::new("balance")
        .route("/add")
        .bind(-100)
        .bind(cur_val)
        .build();

    let apply_callback = CallbackBuilder::new("balance")
        .route("/apply")
        .bind(cur_val)
        .build();

    let kb = vec![
        vec![
            InlineKeyboardButton::callback("+100", add_callback),
            InlineKeyboardButton::callback("-100", minus_callback)
        ],
        vec![InlineKeyboardButton::callback("apply", apply_callback)],
        vec![InlineKeyboardButton::callback("back", "account")]
    ];
    InlineKeyboardMarkup::new(kb)
}
