use teloxide::dispatching::{DpHandlerDescription, HandlerExt, UpdateFilterExt};
use teloxide::prelude::{DependencyMap, Handler, Update};
use teloxide::RequestError;
use crate::handlers::callback_query::router::route_callback_query;
use crate::handlers::message::base_commands::{base_command_handler, BaseCommand};

pub fn base_command_branch()
    -> Handler<'static, DependencyMap, Result<(), RequestError>, DpHandlerDescription> {
    Update::filter_message()
        .filter_command::<BaseCommand>()
        .endpoint(base_command_handler)
}

pub fn callback_query_branch()
    -> Handler<'static, DependencyMap, Result<(), RequestError>, DpHandlerDescription> {
    Update::filter_callback_query()
        .endpoint(route_callback_query)
}