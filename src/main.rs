#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

use log::{debug, info};
use rand::seq::SliceRandom;
use rocket::http::Status;
use rocket_contrib::json::{Json, JsonValue};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
    json!({
            "apiversion": "1",
            "author": "",
            "color": "#888888",
            "head": "default",
            "tail": "default",
    })
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
    info!("received start request for game {}", start_req.game.id);
    debug!("payload: {:?}", start_req.0);

    Status::Ok
}

#[post("/move", format = "json", data = "<move_req>")]
fn handle_move(move_req: Json<BattleSnakeRequest>) -> JsonValue {
    info!("received move request for game {}", move_req.game.id);
    debug!("payload: {:?}", move_req.0);

    let my_head = &move_req.you.head;
    let my_body = &move_req.you.body;
    let mut possible_moves: HashMap<_, _> = vec![
        ("up", true),
        ("down", true),
        ("left", true),
        ("right", true),
    ]
    .into_iter()
    .collect();

    filter_neck_moves(my_head, my_body, &mut possible_moves);

    let moves = possible_moves
        .into_iter()
        .filter(|&(_, v)| v == true)
        .map(|(k, _)| k)
        .collect::<Vec<_>>();

    info!("moves allowed {:?}", moves);

    // TODO: Step 1 - Don't hit walls.
    // Use board information to prevent your Battlesnake from moving beyond the boundaries of the board.
    // board_width = move_req.board.width
    // board_height = move_req.board.height

    // TODO: Step 2 - Don't hit yourself.
    // Use body information to prevent your Battlesnake from colliding with itself.
    // body = move_req.body

    // TODO: Step 3 - Don't collide with others.
    // Use snake vector to prevent your Battlesnake from colliding with others.
    // snakes = move_req.board.snakes

    // TODO: Step 4 - Find food.
    // Use board information to seek out and find food.
    // food = move_req.board.food

    // Finally, choose a move from the available safe moves.
    // TODO: Step 5 - Select a move to make based on strategy, rather than random.

    let chosen = moves.choose(&mut rand::thread_rng()).unwrap();

    info!("picked {}", chosen);

    return json!({ "move": chosen });
}

fn filter_neck_moves(
    my_head: &Coord,
    my_body: &Vec<Coord>,
    possible_moves: &mut HashMap<&str, bool>,
) {
    let my_neck = &my_body[1]; // The segment of body right after the head is the 'neck'

    if my_neck.x < my_head.x {
        // my neck is left of my head
        possible_moves.insert("left", false);
    } else if my_neck.x > my_head.x {
        // my neck is right of my head
        possible_moves.insert("right", false);
    } else if my_neck.y < my_head.y {
        // my neck is below my head
        possible_moves.insert("down", false);
    } else if my_neck.y > my_head.y {
        // my neck is above my head
        possible_moves.insert("up", false);
    }
}

#[post("/end", format = "json", data = "<end_req>")]
fn handle_end(end_req: Json<BattleSnakeRequest>) -> Status {
    info!("received end request for game {}", end_req.game.id);
    debug!("payload: {:?}", end_req.0);

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
