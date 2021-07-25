use rand::seq::SliceRandom;
use rocket_contrib::json::JsonValue;
use std::collections::HashMap;

use log::info;

use crate::{Battlesnake, Board, Coord, Game};

pub fn snake_info() -> JsonValue {
    info!("INFO");

    return json!({
        "apiversion": "1",
        "author": "",
        "color": "#888888",
        "head": "default",
        "tail": "default",
    });
}

pub fn start(game: &Game, _turn: &u32, _board: &Board, _you: &Battlesnake) {
    info!("{} START", game.id);
}

pub fn end(game: &Game, _turn: &u32, _board: &Board, _you: &Battlesnake) {
    info!("{} END", game.id);
}

pub fn get_move(game: &Game, _turn: &u32, _board: &Board, you: &Battlesnake) -> &'static str {
    let my_head = &you.head;
    let my_body = &you.body;
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

    info!("{} MOVE {}", game.id, chosen);

    return chosen;
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
