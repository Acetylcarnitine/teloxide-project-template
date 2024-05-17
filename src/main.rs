mod handlers;
mod keyboards;
mod utils;
mod database;

use std::sync::Arc;
use dotenv::dotenv;
use teloxide::prelude::{
    Bot, Dispatcher, dptree, LoggingErrorHandler
};

use crate::handlers::branches::{
    base_command_branch,
    callback_query_branch
};

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok();
    pretty_env_logger::init();
    log::info!("Starting rust telegram bot");

    // init database pool that we will share with handlers
    let pool = Arc::new(sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(std::env::var("DATABASE_URL").unwrap().as_str())
        .await?
    );

    let bot = Bot::from_env();
    let handler = dptree::entry()
        .branch(base_command_branch())
        .branch(callback_query_branch());

    // remove dptree::deps![] if you haven't got any shared state
    Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![pool])
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

    Ok(())
}