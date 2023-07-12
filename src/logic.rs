// Welcome to
// __________         __    __  .__                               __
// \______   \_____ _/  |__/  |_|  |   ____   ______ ____ _____  |  | __ ____
//  |    |  _/\__  \\   __\   __\  | _/ __ \ /  ___//    \\__  \ |  |/ // __ \
//  |    |   \ / __ \|  |  |  | |  |_\  ___/ \___ \|   |  \/ __ \|    <\  ___/
//  |________/(______/__|  |__| |____/\_____>______>___|__(______/__|__\\_____>
//
// This file can be a nice home for your Battlesnake logic and helper functions.
//
// To get you started we've included code to prevent your Battlesnake from moving backwards.
// For more info see docs.battlesnake.com

use std::collections::HashMap;

use log::info;
use rand::seq::SliceRandom;
use serde_json::{json, Value};

use crate::{Battlesnake, Board, Coord, Game};

// move is called on every turn and returns your next move
// Valid moves are "up", "down", "left", or "right"
// See https://docs.battlesnake.com/api/example-move for available data
pub fn get_move(_game: &Game, turn: &u32, board: &Board, you: &Battlesnake) -> Value {
    let mut is_move_safe: HashMap<_, _> = vec![
        ("up", true),
        ("down", true),
        ("left", true),
        ("right", true),
    ]
        .into_iter()
        .collect();

    // We've included code to prevent your Battlesnake from moving backwards
    let my_head = &you.body[0]; // Coordinates of your head
    let my_neck = &you.body[1]; // Coordinates of your "neck"

    if my_neck.x < my_head.x {
        // Neck is left of head, don't move left
        is_move_safe.insert("left", false);
    } else if my_neck.x > my_head.x {
        // Neck is right of head, don't move right
        is_move_safe.insert("right", false);
    } else if my_neck.y < my_head.y {
        // Neck is below head, don't move down
        is_move_safe.insert("down", false);
    } else if my_neck.y > my_head.y {
        // Neck is above head, don't move up
        is_move_safe.insert("up", false);
    }

    info!("HEAD {:?}", you.head);
    // TODO: Step 1 - Prevent your Battlesnake from moving out of bounds
    set_moves_inbound(
        &mut is_move_safe,
        my_head,
        &board.width,
        &board.height
    );

    // TODO: Step 2 - Prevent your Battlesnake from colliding with itself
    set_moves_collide_self(
        &mut is_move_safe,
        you
    );

    // TODO: Step 3 - Prevent your Battlesnake from colliding with other Battlesnakes
    // let opponents = &board.snakes;

    // Are there any safe moves left?
    let safe_moves = is_move_safe
        .into_iter()
        .filter(|&(_, v)| v)
        .map(|(k, _)| k)
        .collect::<Vec<_>>();

    // Choose a random move from the safe ones
    let chosen = safe_moves.choose(&mut rand::thread_rng()).unwrap();

    // TODO: Step 4 - Move towards food instead of random, to regain health and survive longer
    // let food = &board.food;

    info!("MOVE {}: {}", turn, chosen);
    json!({ "move": chosen })
}

// info is called when you create your Battlesnake on play.battlesnake.com
// and controls your Battlesnake's appearance
// TIP: If you open your Battlesnake URL in a browser you should see this data
pub fn info() -> Value {
    info!("INFO");

    json!({
        "apiversion": "1",
        "author": "ponchoalv", // TODO: Your Battlesnake Username
        "color": "#888888", // TODO: Choose color
        "head": "default", // TODO: Choose head
        "tail": "default", // TODO: Choose tail
    })
}

// start is called when your Battlesnake begins a game
pub fn start(_game: &Game, _turn: &u32, _board: &Board, _you: &Battlesnake) {
    info!("GAME START");
}

// end is called when your Battlesnake finishes a game
pub fn end(_game: &Game, _turn: &u32, _board: &Board, _you: &Battlesnake) {
    info!("GAME OVER");
}

fn set_moves_inbound(
    safe_moves: &mut HashMap<&str, bool>,
    head: &Coord,
    width: &u32,
    height: &u32,
) {
    if head.x == width - 1 {
        safe_moves.insert("right", false);
    }
    if head.x == 0 {
        safe_moves.insert("left", false);
    }

    if head.y == height - 1 {
        safe_moves.insert("up", false);
    }

    if head.y == 0 {
        safe_moves.insert("down", false);
    }
}

fn set_moves_collide_self(safe_moves: &mut HashMap<&str, bool>, you: &Battlesnake) {
    let head = &you.head;
    let head_moved_left = Coord {
        x: if head.x == 0 { 0 } else { head.x - 1 },
        y: head.y,
    };
    let head_moved_right = Coord {
        x: head.x + 1,
        y: head.y,
    };
    let head_moved_down = Coord {
        x: head.x,
        y: if head.y == 0 { 0 } else { head.y - 1 },
    };
    let head_moved_up = Coord {
        x: head.x,
        y: head.y + 1,
    };

    if you.body.contains(&head_moved_left) && you.body.last() != Some(&head_moved_left) {
        safe_moves.insert("left", false);
    }
    if you.body.contains(&head_moved_right) && you.body.last() != Some(&head_moved_right) {
        safe_moves.insert("right", false);
    }
    if you.body.contains(&head_moved_up) && you.body.last() != Some(&head_moved_up) {
        safe_moves.insert("up", false);
    }
    if you.body.contains(&head_moved_down) && you.body.last() != Some(&head_moved_down) {
        safe_moves.insert("down", false);
    }
}

#[cfg(test)]
mod tests_boundaries {
    use super::*;

    #[test]
    fn test_upper_boundary() {
        let mut is_move_safe: HashMap<_, _> = vec![
            ("up", true),
            ("down", true),
            ("left", true),
            ("right", true),
        ]
            .into_iter()
            .collect();
        let width: u32 = 11;
        let height: u32 = 11;

        let expected: HashMap<_, _> = vec![
            ("up", false),
            ("down", true),
            ("left", true),
            ("right", true),
        ]
            .into_iter()
            .collect();

        set_moves_inbound(&mut is_move_safe, &Coord { x: 5, y: 10 }, &width, &height);

        assert_eq!(is_move_safe, expected)
    }

    #[test]
    fn test_bottom_boundary() {
        let mut is_move_safe: HashMap<_, _> = vec![
            ("up", true),
            ("down", true),
            ("left", true),
            ("right", true),
        ]
            .into_iter()
            .collect();
        let width: u32 = 11;
        let height: u32 = 11;

        let expected: HashMap<_, _> = vec![
            ("up", true),
            ("down", false),
            ("left", true),
            ("right", true),
        ]
            .into_iter()
            .collect();

        set_moves_inbound(&mut is_move_safe, &Coord { x: 5, y: 0 }, &width, &height);

        assert_eq!(is_move_safe, expected)
    }

    #[test]
    fn test_right_boundary() {
        let mut is_move_safe: HashMap<_, _> = vec![
            ("up", true),
            ("down", true),
            ("left", true),
            ("right", true),
        ]
            .into_iter()
            .collect();
        let width: u32 = 11;
        let height: u32 = 11;

        let expected: HashMap<_, _> = vec![
            ("up", true),
            ("down", true),
            ("left", true),
            ("right", false),
        ]
            .into_iter()
            .collect();

        set_moves_inbound(&mut is_move_safe, &Coord { x: 10, y: 5 }, &width, &height);

        assert_eq!(is_move_safe, expected)
    }

    #[test]
    fn test_left_boundary() {
        let mut is_move_safe: HashMap<_, _> = vec![
            ("up", true),
            ("down", true),
            ("left", true),
            ("right", true),
        ]
            .into_iter()
            .collect();
        let width: u32 = 11;
        let height: u32 = 11;

        let expected: HashMap<_, _> = vec![
            ("up", true),
            ("down", true),
            ("left", false),
            ("right", true),
        ]
            .into_iter()
            .collect();

        set_moves_inbound(&mut is_move_safe, &Coord { x: 0, y: 5 }, &width, &height);

        assert_eq!(is_move_safe, expected)
    }
}

#[cfg(test)]
mod tests_self_collitions {
    use super::*;

    #[test]
    fn test_no_collide_with_tail() {
        let you = Battlesnake {
            id: "you".to_string(),
            name: "you".to_string(),
            health: 100,
            body: vec![
                Coord { x: 1, y: 0 },
                Coord { x: 1, y: 1 },
                Coord { x: 1, y: 2 },
                Coord { x: 2, y: 2 },
                Coord { x: 2, y: 1 },
                Coord { x: 2, y: 0 },
            ],
            head: Coord { x: 1, y: 0 },
            length: 6,
            latency: "100".to_string(),
            shout: None,
        };

        let mut is_move_safe: HashMap<_, _> = vec![
            ("up", true),
            ("down", true),
            ("left", true),
            ("right", true),
        ]
            .into_iter()
            .collect();

        let expected: HashMap<_, _> = vec![
            ("up", false),
            ("down", false),
            ("left", true),
            ("right", true),
        ]
            .into_iter()
            .collect();

        set_moves_collide_self(&mut is_move_safe, &you);

        assert_eq!(is_move_safe, expected)
    }
}
