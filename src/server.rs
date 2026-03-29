use std::marker::PhantomData;
use std::net::SocketAddr;

use axum::Router;
use axum::extract::State;
use axum::extract::WebSocketUpgrade;
use axum::extract::ws::Message;
use axum::extract::ws::WebSocket;
use axum::response::IntoResponse;
use axum::routing::get;
use futures::SinkExt;
use futures::StreamExt;
use tokio::sync::mpsc;

use crate::server::handlers::handle;
use crate::server::state::ServerState;

mod handlers;
mod state;
pub mod timestamp;

use crate::storage::Storage;
use crate::utils::IdentifierGenerator;

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
            .route("/", get(Server::websocket_handler))
            .with_state(state);

        const ADDRESS: &str = "0.0.0.0:1337";
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

        for (id, _) in state.connections().await.iter() {
            log::info!("{id}");
        }

        let user_id: String = state.add_connection(tx.clone()).await;

        handlers::welcome_new_user(tx.clone(), user_id.clone(), state.clone()).await;

        let forward_task = tokio::spawn(async move {
            while let Some(msg) = rx.recv().await {
                // The sink may fail if the client disconnects; ignore errors here
                log::info!("{0}", msg.to_text().unwrap());
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
                handle(user_id.clone(), message, tx.clone(), state.clone()).await;
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
