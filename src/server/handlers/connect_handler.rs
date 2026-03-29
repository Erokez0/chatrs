use axum::extract::ws::Message;
use tokio::sync::mpsc;

use crate::{
    message::RequestMessage,
    server::{state::ServerState, timestamp::Timestamp},
    storage::Storage,
    utils::IdentifierGenerator,
};

pub async fn welcome_new_user<S, G>(
    tx: mpsc::UnboundedSender<Message>,
    user_id: String,
    _state: ServerState<S, G>,
) where
    S: Storage,
    G: IdentifierGenerator,
{
    log::info!("got new user id: {user_id}");
    let request_message = RequestMessage::ServerGaveId {
        user_id,
        at: Timestamp::now(),
    };

    let websocket_message: Message = match request_message.try_into() {
        Ok(ws_msg) => ws_msg,
        Err(error) => {
            log::error!("could not convert RequestMessage::Welcome to websocket message: {error}");
            return;
        }
    };

    match tx.send(websocket_message) {
        Ok(()) => log::info!("successfully sent user welcome message"),
        Err(error) => log::error!("could not send user welcome message: {error}"),
    };
}
