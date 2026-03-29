mod connect_handler;
mod user_sent_message_handler;

use axum::extract::ws::Message;
pub use connect_handler::*;
use tokio::sync::mpsc;
pub use user_sent_message_handler::*;

use crate::{
    message::RequestMessage, server::state::ServerState, storage::Storage,
    utils::IdentifierGenerator,
};

pub async fn handle<S, G>(
    user_id: String,
    message: Message,
    tx: mpsc::UnboundedSender<Message>,
    state: ServerState<S, G>,
) where
    S: Storage,
    G: IdentifierGenerator,
{
    match message {
        Message::Close(_) => {
            state.remove_connection(user_id).await;
            return;
        }
        _ => {}
    };

    let request_message: RequestMessage = match message.try_into() {
        Ok(req_msg) => req_msg,
        Err(err) => {
            log::error!("error converting websocket message to request message");
            RequestMessage::ServerResponseStatus {
                ok: true,
                text: err.to_string(),
            }
        }
    };

    if !request_message.allowed_from_client() {
        log::info!("{request_message}");
        let kind: String = request_message.kind();
        let text: String = format!("client is not allowed to send messages of kind [{kind}]");
        let reponse: Message = RequestMessage::ServerResponseStatus { ok: false, text }
            .try_into()
            .unwrap_or(Message::Text("oops".into()));
        let send_result = tx.send(reponse);
        match send_result {
            Ok(()) => return,
            Err(error) => {
                log::error!("could not send message to user: {error}")
            }
        }
        return;
    }

    match request_message.clone() {
        RequestMessage::UserSentMessage { message } => {
            user_sent_message_handler(user_id.clone(), tx.clone(), &message, state).await;
        }
        _ => (),
    };
}
