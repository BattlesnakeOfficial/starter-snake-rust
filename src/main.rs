#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

use log::{debug, info};
use rocket::http::Status;
use rocket_contrib::json::{Json, JsonValue};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mod logic;

// Types derived from https://docs.battlesnake.com/references/api#object-definitions
#[derive(Deserialize, Serialize, Debug)]
pub struct Game {
    id: String,
    ruleset: HashMap<String, String>,
    timeout: u32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Board {
    height: u32,
    width: u32,
    food: Vec<Coord>,
    snakes: Vec<Battlesnake>,
    hazards: Vec<Coord>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Battlesnake {
    id: String,
    name: String,
    health: u32,
    body: Vec<Coord>,
    head: Coord,
    length: u32,
    latency: String,

    // Used in non-standard game modes
    shout: Option<String>,
    squad: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Coord {
    x: u32,
    y: u32,
}

#[get("/")]
fn handle_index() -> JsonValue {
    // Personalize the look of your snake per https://docs.battlesnake.com/references/personalization
    logic::snake_info()
}

#[derive(Deserialize, Serialize, Debug)]
pub struct BattleSnakeRequest {
    game: Game,
    turn: u32,
    board: Board,
    you: Battlesnake,
}

#[post("/start", format = "json", data = "<start_req>")]
fn handle_start(start_req: Json<BattleSnakeRequest>) -> Status {
    debug!("payload: {:?}", start_req.0);

    logic::start(
        &start_req.game,
        &start_req.turn,
        &start_req.board,
        &start_req.you,
    );

    Status::Ok
}

#[post("/move", format = "json", data = "<move_req>")]
fn handle_move(move_req: Json<BattleSnakeRequest>) -> JsonValue {
    info!("received move request for game {}", move_req.game.id);
    debug!("payload: {:?}", move_req.0);

    let chosen = logic::get_move(
        &move_req.game,
        &move_req.turn,
        &move_req.board,
        &move_req.you,
    );

    return json!({ "move": chosen });
}

#[post("/end", format = "json", data = "<end_req>")]
fn handle_end(end_req: Json<BattleSnakeRequest>) -> Status {
    debug!("payload: {:?}", end_req.0);

    logic::end(&end_req.game, &end_req.turn, &end_req.board, &end_req.you);

    Status::Ok
}

fn main() {
    env_logger::init();

    rocket::ignite()
        .mount(
            "/",
            routes![handle_index, handle_start, handle_move, handle_end],
        )
        .launch();
}
