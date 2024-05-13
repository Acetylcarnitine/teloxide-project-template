mod handlers;
mod keyboards;
mod utils;

use dotenv::dotenv;
use teloxide::{prelude::*, RequestError};
use crate::handlers::message::base_commands::{base_command_handler, BaseCommand};
use crate::utils::{
    callback_builder::CallbackBuilder,
    string_parser::is_digit
};
use crate::utils::callback_parsed::CallbackParsed;

#[tokio::main]
async fn main() {
    dotenv().ok();
    pretty_env_logger::init();
    log::info!("Starting rust telegram bot");

    let bot = Bot::from_env();
    let handler = dptree::entry()
        .branch(
            Update::filter_message()
                .filter_command::<BaseCommand>()
                .endpoint(base_command_handler)
        )
        .branch(
            Update::filter_callback_query()
                .filter(|query: CallbackQuery| query.data.is_some())
                .endpoint(parse_callback_query)
        );

    Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![])
        .default_handler(|upd| async move {
            log::warn!("Unknown update handled {:?}", upd);
        })
        .error_handler(LoggingErrorHandler::with_custom_text(
            "An error has occurred in the dispatcher"
        ))
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}

async fn parse_callback_query(
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

#[cfg(test)]
mod test {
    use crate::utils::callback_parsed::CallbackParsed;
    use super::*;

    #[test]
    fn check_callback_builder() {
        let callback = CallbackBuilder::new("users")
            .route("/deposit")
            .bind::<i32>(104000)
            .bind::<String>("rub".to_string())
            .build();
        assert_eq!("users/deposit?104000&rub".to_string(), callback);
    }

    #[test]
    fn is_digit_check() {
        assert!(is_digit(&"12309".to_string()))
    }

    #[test]
    fn check_non_digit_letter_first() {
        assert!(!is_digit(&"nigger1337".to_string()))
    }

    #[test]
    fn check_non_digit_number_first() {
        assert!(!is_digit(&"123abobus".to_string()))
    }

    #[test]
    fn check_is_digit_on_empty() {
        assert!(!is_digit(&"".to_string()))
    }

    #[test]
    fn check_callback_parser0() {
        let data = String::from("users?123");
        let parsed = CallbackParsed::parse(&data);
        assert_eq!(parsed.base, "users");
        assert_eq!(parsed.route, None);
        assert_eq!(parsed.params, vec!["123"]);
    }

    #[test]
    fn check_callback_parser1() {
        let data = String::from("users");
        let parsed = CallbackParsed::parse(&data);
        assert_eq!(parsed.base, "users");
        assert_eq!(parsed.route, None);
        assert_eq!(parsed.params, Vec::<String>::new());
    }

    #[test]
    fn check_callback_parser2() {
        let data = String::from("users/deposit?104");
        let parsed = CallbackParsed::parse(&data);
        assert_eq!(parsed.base, "users");
        assert_eq!(parsed.route, Option::from("/deposit".to_string()));
        assert_eq!(parsed.params, vec!["104"]);
    }

    #[test]
    fn check_callback_parser3() {
        let data = String::from("users/deposit?123&345.21&niggers");
        let parsed = CallbackParsed::parse(&data);
        assert_eq!(parsed.base, "users");
        assert_eq!(parsed.route, Option::from("/deposit".to_string()));
        assert_eq!(parsed.params, vec!["123", "345.21", "niggers"]);
    }

    #[test]
    fn check_callback_parser4() {
        let data = String::from("users?123");
        let id = CallbackParsed::parse(&data)
            .get_arg::<i32>(0)
            .unwrap();
        assert_eq!(123, id);
    }

    #[test]
    fn check_callback_parser5() {
        let data = String::from("users/rating?123.45");
        let id = CallbackParsed::parse(&data)
            .get_arg::<f32>(0)
            .unwrap();
        assert_eq!(123.45, id);
    }

    #[test]
    fn check_callback_parser6() {
        let data = String::from("users/status?admin");
        let callback = CallbackParsed::parse(&data);
        let status = callback.get_str_arg(0);
        assert_eq!("admin", status);
    }

    #[test]
    fn check_callback_parser7() {
        let data = String::from("users/mmo?player&145");
        let callback = CallbackParsed::parse(&data);
        let id = callback
            .get_arg::<i32>(1)
            .unwrap();
        let status = callback.get_str_arg(0);
        assert_eq!(145, id);
        assert_eq!("player", status);
    }
}