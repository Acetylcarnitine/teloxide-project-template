use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};

pub fn start_kb() -> InlineKeyboardMarkup {
    let kb = vec![
        vec![InlineKeyboardButton::callback("let's go", "account")]
    ];
    InlineKeyboardMarkup::new(kb)
}