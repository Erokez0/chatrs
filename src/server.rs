use std::marker::PhantomData;
use std::net::SocketAddr;

use axum::Router;
use axum::extract::State;
use axum::extract::WebSocketUpgrade;
use axum::extract::ws::Message;
use axum::extract::ws::WebSocket;
use axum::response::IntoResponse;
use axum::routing::any;
use futures::SinkExt;
use futures::StreamExt;
use tokio::sync::mpsc;

use crate::message::RequestMessage;
use crate::server::state::ServerState;

mod handlers;
mod state;
pub mod timestamp;

use crate::storage::Storage;
use crate::utils::IdentifierGenerator;
use crate::utils::UserId;

pub struct Server<S, G>
where
    S: Storage,
    G: IdentifierGenerator,
{
    _s: PhantomData<S>,
    _g: PhantomData<G>,
}

impl<S, G> Server<S, G>
where
    S: Storage,
    G: IdentifierGenerator,
{
    pub fn new() -> Self {
        Self {
            _s: PhantomData,
            _g: PhantomData,
        }
    }

    pub async fn start(self) {
        let state = ServerState::<S, G>::new();
        let app: Router = Router::new()
            .route("/", any(Server::websocket_handler))
            .with_state(state);

        const ADDRESS: &'static str = "0.0.0.0:1337";
        let listener = tokio::net::TcpListener::bind(ADDRESS).await.unwrap();

        axum::serve(
            listener,
            app.into_make_service_with_connect_info::<SocketAddr>(),
        )
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
    }

    async fn websocket_handler(
        State(state): State<ServerState<S, G>>,
        ws: WebSocketUpgrade,
    ) -> impl IntoResponse {
        ws.on_upgrade(move |web_socket| async { Server::upgrade_handler(state, web_socket).await })
    }

    async fn upgrade_handler(mut state: ServerState<S, G>, ws: WebSocket) {
        let (mut sender, mut receiver) = ws.split();
        let (tx, mut rx) = mpsc::unbounded_channel::<Message>();

        let user_id: UserId = state.generator.new_id();

        handlers::welcome_new_user(tx.clone(), user_id.clone(), state.clone()).await;
        state.add_connection(user_id.clone(), tx.clone());

        let forward_task = tokio::spawn(async move {
            while let Some(msg) = rx.recv().await {
                // The sink may fail if the client disconnects; ignore errors here
                let _ = sender.send(msg).await;
            }
        });

        let recv_task = tokio::spawn(async move {
            while let Some(request) = receiver.next().await {
                let message: Message = match request {
                    Ok(msg) => msg,
                    Err(err) => {
                        log::error!("could not obtain websocket message: {err}");
                        return;
                    }
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

                for (id, tx) in state.connections() {
                    if *id == user_id {
                        continue;
                    }
                    match tx.send(request_message.clone().try_into().unwrap()) {
                        Ok(_) => (),
                        Err(err) => {
                            log::error!("error converting websocket message to request message");
                            RequestMessage::ServerResponseStatus {
                                ok: true,
                                text: err.to_string(),
                            };
                        }
                    }
                }

                let response: RequestMessage = request_message;

                let response_message: Message = match response.try_into() {
                    Ok(message) => message,
                    Err(err) => {
                        log::error!(
                            "got an error converting response to a websocket message: {err}"
                        );
                        match (RequestMessage::ServerResponseStatus {
                            ok: false,
                            text: err.to_string(),
                        })
                        .try_into()
                        {
                            Ok(status_message) => status_message,
                            Err(_) => Message::Text("there is a lot going on".to_string().into()),
                        }
                    }
                };

                let send_result = tx.clone().send(response_message);
                match send_result {
                    Ok(()) => (),
                    Err(err) => log::error!("got an error sending websocket message: {err}"),
                };
            }
        });

        tokio::select! {
            _ = forward_task => {},
            _ = recv_task => {},
        };
    }
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("could not bind ctrl+c")
}
