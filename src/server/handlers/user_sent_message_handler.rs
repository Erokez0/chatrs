use axum::extract::ws::Message;
use tokio::sync::mpsc;

use crate::{
    message::{RequestMessage, ServerChatMessage, UserChatMessage},
    server::state::ServerState,
    storage::Storage,
    utils::IdentifierGenerator,
};

pub async fn user_sent_message_handler<S, G>(
    author_id: String,
    tx: mpsc::UnboundedSender<Message>,
    message: &UserChatMessage,
    mut state: ServerState<S, G>,
) where
    S: Storage,
    G: IdentifierGenerator,
{
    log::info!("got message: {message}");
    let server_message: ServerChatMessage = match state
        .storage
        .save_message(author_id.clone(), message.clone())
        .await
    {
        Ok(msg) => {
            log::info!("saved message");
            log::info!("{msg}");
            msg
        }
        Err(err) => {
            log::error!("{err}");
            let error_message: Message = RequestMessage::ServerResponseStatus {
                ok: false,
                text: format!("could not save your message: {err}"),
            }
            .try_into()
            .expect("expected server response status message to convert into ws::Message");
            tx.send(error_message).unwrap();
            return;
        }
    };

    let request_message: RequestMessage = RequestMessage::ServerSentMessage {
        message: server_message,
    };
    log::info!("{request_message}");

    let ws_message: Message = request_message.try_into().unwrap();

    log::info!("CONNECTION IDS");
    for (id, tx) in state.connections().await.iter() {
        log::info!("{id}");
        if id.clone() != author_id {
            log::info!("sending message from {0} to {1}", author_id, id.clone());
            match tx.send(ws_message.clone()) {
                Ok(_) => (),
                Err(err) => {
                    log::error!("error converting websocket message to request message");
                    true;
                    err.to_string();
                }
            }
        }
    }

    let ok_response: RequestMessage = RequestMessage::ServerResponseStatus {
        ok: true,
        text: "message sent successfully".to_string(),
    };
    let ws_message_response: Message = match ok_response.try_into() {
        Ok(msg) => msg,
        Err(err) => {
            log::error!("{err}");
            return;
        }
    };
    tx.send(ws_message_response).unwrap();
}
