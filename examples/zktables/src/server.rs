use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::Serialize;
use zktables_core::crypto::{self, Privkey, Pubkey};

type Data = Vec<u8>;

#[derive(Debug, Clone, Serialize)]
struct ServerState {
    rounds: Vec<Round>,
    keys: (Pubkey, Privkey),
}

impl Default for ServerState {
    fn default() -> Self {
        let (pubkey, privkey) = crypto::generate_keypair();
        Self {
            rounds: Vec::new(),
            keys: (pubkey, privkey),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
struct Round {
    inputs: HashMap<Pubkey, Data>,
    outputs: HashMap<Pubkey, Data>,
    proof: u8, // TODO
}

impl Round {
    // fn new(inputs: HashMap<Pubkey, Data>, outputs: HashMap<Pubkey, Data>) -> Self
    // {     Self {
    //         inputs,
    //         outputs,
    //         proof: todo!(),
    //     }
    // }
}

#[derive(Clone, Serialize)]
struct UserInput {
    pubkey: Pubkey,
    data: Data, // encrypted
}

type DB = Arc<RwLock<ServerState>>;

#[tokio::main]
pub async fn server(host: String) {
    // Prepare default server state
    let (pubkey, privkey) = crypto::generate_keypair();
    let db = DB::default();
    db.write().unwrap().keys = (pubkey, privkey);

    let app = Router::new()
        .route("/", get(index))
        .route("/vote", post(vote))
        .with_state(db);

    println!("Listening on http://{}", host);
    let addr = host.parse().expect("Invalid host address");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn vote(
    State(db): State<DB>,
    Json(input): Json<UserInput>,
) -> impl IntoResponse {
    let UserInput { pubkey, data } = input;
    println!("Received data from {:?}", pubkey);
    println!("data: {:?}", data);

    // let (pubkey, privkey) = db.read().unwrap().keys.clone();
    // if round complete,
    // TODO
    // if false {
    //     let error_response = serde_json::json!({
    //         "status": "fail",
    //         "message": format!("Input with pubkey: '{}' already exists in the
    // current round", "TODO"),     });
    //     return Err((StatusCode::CONFLICT, Json(error_response)));
    // }
    Json("OK")
}

async fn keys(State(db): State<DB>) -> impl IntoResponse {
    let (pubkey, privkey) = db.read().unwrap().keys.clone();
    Json((pubkey, privkey))
}

async fn index(State(db): State<DB>) -> impl IntoResponse {
    println!("Received request at index");
    let state = db.read().unwrap().clone();
    Json(state)
}
