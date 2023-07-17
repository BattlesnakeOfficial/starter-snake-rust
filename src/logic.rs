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
// use par_map::ParMap;
use rocket::form::validate::Contains;
use serde_json::{json, Value};

use crate::{Battlesnake, Board, Coord, Game, Move};

// move is called on every turn and returns your next move
// Valid moves are "up", "down", "left", or "right"
// See https://docs.battlesnake.com/api/example-move for available data
pub fn get_move(_game: &Game, turn: &u32, board: &Board, you: &Battlesnake) -> Value {
    debug!("turn: {} - board: {:?}", turn, board);
    debug!("turn: {} - you: {:?}", turn, you);

    let board = board.clone();
    let mut you = you.clone();

    if you.length > 3
        && you.body[you.length as usize - 2] == you.body[you.length as usize - 1 as usize]
    {
        you.body.pop();
        you.length = you.body.len() as u32;
    }

    let safe_moves = available_moves(&board, &you);

    let scored_moves: HashMap<Move, i32> = safe_moves
        .iter()
        .cloned()
        .map(move |mv| {
            let moved_snake = move_snake(&board, you.clone(), &mv);
            // debug!("ROOT move: {:?}", mv);
            (mv, maximise(&board, moved_snake, 8, i32::MIN))
        })
        .collect();

    debug!("scored moves {:?}", scored_moves);

    // Choose a random move from the safe ones
    let chosen: (Option<Move>, i32) =
        scored_moves
            .into_iter()
            .fold((None, i32::MIN), |(prev_mv, prev_score), (mv, score)| {
                if prev_mv.is_none() || score > prev_score {
                    (Some(mv), score)
                } else {
                    (prev_mv, prev_score)
                }
            });

    // TODO: Step 4 - Move towards food instead of random, to regain health and survive longer
    // let food = &board.food;

    match chosen {
        (Some(mv), _) => {
            info!("MOVE {}: {:?}", turn, mv);
            json!({ "move": mv })
        }
        _ => {
            info!("no move selected, going up");
            json!({"move" : "up"})
        }
    }
}

fn move_snake(board: &Board, snake: Battlesnake, direction: &Move) -> Battlesnake {
    let mut new_snake = Battlesnake {
        id: snake.id.clone(),
        name: snake.name.clone(),
        health: if snake.health == 0 {
            0
        } else {
            snake.health - 1
        },
        body: snake.body.clone(),
        head: snake.head,
        length: snake.length,
        latency: snake.latency.clone(),
        shout: None,
    };

    let new_head = match direction {
        Move::Up => Coord {
            x: snake.head.x,
            y: snake.head.y + 1,
        },
        Move::Down => Coord {
            x: snake.head.x,
            y: snake.head.y - 1,
        },
        Move::Left => Coord {
            x: snake.head.x - 1,
            y: snake.head.y,
        },
        Move::Right => Coord {
            x: snake.head.x + 1,
            y: snake.head.y,
        },
    };

    new_snake.head = new_head;
    new_snake.body.insert(0, new_head);

    if board.food.contains(&new_snake.head) {
        new_snake.health = 100;
    }

    if snake.health == 100 {
        new_snake.length += 1;
    } else {
        new_snake.body.pop();
    }

    // debug!("moved {:?}: new moved snake {:?}", direction, new_snake);
    new_snake
}

fn available_moves(board: &Board, you: &Battlesnake) -> Vec<Move> {
    let mut is_move_safe = get_initial_moves();

    // We've included code to prevent your Battlesnake from moving backwards
    let my_head = &you.body[0]; // Coordinates of your head
    let my_neck = &you.body[1]; // Coordinates of your "neck"

    if my_neck.x < my_head.x {
        // Neck is left of head, don't move left
        is_move_safe.insert(Move::Left, false);
    } else if my_neck.x > my_head.x {
        // Neck is right of head, don't move right
        is_move_safe.insert(Move::Right, false);
    } else if my_neck.y < my_head.y {
        // Neck is below head, don't move down
        is_move_safe.insert(Move::Down, false);
    } else if my_neck.y > my_head.y {
        // Neck is above head, don't move up
        is_move_safe.insert(Move::Up, false);
    }

    // Step 1 - Prevent your Battlesnake from moving out of bounds
    set_moves_inbound(&mut is_move_safe, my_head, &board.width, &board.height);

    // Step 2 - Prevent your Battlesnake from colliding with itself
    set_moves_collide_self(&mut is_move_safe, you);

    // Step 3 - Prevent your Battlesnake from colliding with other Battlesnakes
    // let opponents = &board.snakes;

    set_moves_has_collided_with_tail(&mut is_move_safe, you);

    // Are there any safe moves left?
    is_move_safe
        .into_iter()
        .filter(|&(_, v)| v)
        .map(|(k, _)| k)
        .collect::<Vec<_>>()
}

fn get_initial_moves() -> HashMap<Move, bool> {
    vec![
        (Move::Up, true),
        (Move::Down, true),
        (Move::Left, true),
        (Move::Right, true),
    ]
    .into_iter()
    .collect()
}

// info is called when you create your Battlesnake on play.battlesnake.com
// and controls your Battlesnake's appearance
// TIP: If you open your Battlesnake URL in a browser you should see this data
pub fn info() -> Value {
    info!("INFO");

    json!({
        "apiversion": "1",
        "author": "ponchoalv", // Your Battlesnake Username
        "color": "#888888", // Choose color
        "head": "default", // Choose head
        "tail": "default", // Choose tail
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
    safe_moves: &mut HashMap<Move, bool>,
    head: &Coord,
    width: &u32,
    height: &u32,
) {
    if head.y == height - 1 {
        safe_moves.insert(Move::Up, false);
    }
    if head.y == 0 {
        safe_moves.insert(Move::Down, false);
    }
    if head.x == 0 {
        safe_moves.insert(Move::Left, false);
    }
    if head.x == width - 1 {
        safe_moves.insert(Move::Right, false);
    }
}

fn set_moves_collide_self(safe_moves: &mut HashMap<Move, bool>, you: &Battlesnake) {
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
        safe_moves.insert(Move::Left, false);
    }
    if you.body.contains(&head_moved_right) && you.body.last() != Some(&head_moved_right) {
        safe_moves.insert(Move::Right, false);
    }
    if you.body.contains(&head_moved_up) && you.body.last() != Some(&head_moved_up) {
        safe_moves.insert(Move::Up, false);
    }
    if you.body.contains(&head_moved_down) && you.body.last() != Some(&head_moved_down) {
        safe_moves.insert(Move::Down, false);
    }
}

fn set_moves_has_collided_with_tail(safe_moves: &mut HashMap<Move, bool>, you: &Battlesnake) {
    if you.body.last() == Some(&you.head) {
        safe_moves.insert(Move::Up, false);
        safe_moves.insert(Move::Down, false);
        safe_moves.insert(Move::Left, false);
        safe_moves.insert(Move::Right, false);
    }
}

fn score_position(board: &Board, you: &Battlesnake, current_score: i32) -> i32 {
    let food_value = 3;
    let space_value = 2;
    let mut score = current_score;
    let safe_moves = available_moves(board, you);
    let will_eat = board.food.contains(you.head);
    let health_threshold = 10; // Adjust this threshold based on your game rules

    score += space_value * safe_moves.len() as i32;

    if will_eat {
        if you.health <= health_threshold {
            score += food_value; // Add extra score for eating when health is low
        } else if score > i32::MIN + food_value {
            score -= food_value; // Reduce the score for eating when health is high
        }
    }

    if Some(&you.head) == you.body.last() || safe_moves.is_empty() || you.health == 0 {
        score = i32::MIN;
    }

    score
}

fn maximise(board: &Board, you: Battlesnake, depth: u32, current_score: i32) -> i32 {
    let possible_moves = available_moves(board, &you);

    if depth == 0 || possible_moves.is_empty() {
        return score_position(board, &you, current_score);
    }

    let mut max_score = i32::MIN;
    for mv in possible_moves {
        let moved_snake = move_snake(board, you.clone(), &mv); // Update the snake's position
        let score = maximise(board, moved_snake, depth - 1, current_score); // Pass the updated snake to the recursive call
                                                                            // debug!("move: {:?}, score: {}, level: {}", mv, score, 10 - depth);
        max_score = max_score.max(score);
    }
    max_score
}

// fn minimax(state: &Game, depth: i32, maximizing_player: bool) -> i32 {
//     if depth == 0 || state.is_game_over() {
//         // Base case: evaluate the current state
//         return evaluate_state(state);
//     }
//
//     if maximizing_player {
//         let mut max_score = i32::MIN;
//         for mv in state.available_moves() {
//             let score = minimax(&state, depth - 1, false);
//             max_score = max_score.max(score);
//         }
//         max_score
//     } else {
//         let mut min_score = i32::MAX;
//         for mv in state.available_moves() {
//             let score = minimax(&state, depth - 1, true);
//             min_score = min_score.min(score);
//         }
//         min_score
//     }
// }

#[cfg(test)]
mod test_helpers {
    use crate::{Battlesnake, Coord};

    pub fn test_get_battlesnake_with_health(health: u32) -> Battlesnake {
        Battlesnake {
            id: "you".to_string(),
            name: "you".to_string(),
            health,
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
        }
    }
}

#[cfg(test)]
mod tests_boundaries {
    use super::*;

    #[test]
    fn test_upper_boundary() {
        let mut is_move_safe = get_initial_moves();
        let width: u32 = 11;
        let height: u32 = 11;

        let expected: HashMap<_, _> = vec![
            (Move::Up, false),
            (Move::Down, true),
            (Move::Left, true),
            (Move::Right, true),
        ]
        .into_iter()
        .collect();

        set_moves_inbound(&mut is_move_safe, &Coord { x: 5, y: 10 }, &width, &height);

        assert_eq!(is_move_safe, expected)
    }

    #[test]
    fn test_bottom_boundary() {
        let mut is_move_safe = get_initial_moves();
        let width: u32 = 11;
        let height: u32 = 11;

        let expected: HashMap<_, _> = vec![
            (Move::Up, true),
            (Move::Down, false),
            (Move::Left, true),
            (Move::Right, true),
        ]
        .into_iter()
        .collect();

        set_moves_inbound(&mut is_move_safe, &Coord { x: 5, y: 0 }, &width, &height);

        assert_eq!(is_move_safe, expected)
    }

    #[test]
    fn test_right_boundary() {
        let mut is_move_safe = get_initial_moves();
        let width: u32 = 11;
        let height: u32 = 11;

        let expected: HashMap<_, _> = vec![
            (Move::Up, true),
            (Move::Down, true),
            (Move::Left, true),
            (Move::Right, false),
        ]
        .into_iter()
        .collect();

        set_moves_inbound(&mut is_move_safe, &Coord { x: 10, y: 5 }, &width, &height);

        assert_eq!(is_move_safe, expected)
    }

    #[test]
    fn test_left_boundary() {
        let mut is_move_safe = get_initial_moves();
        let width: u32 = 11;
        let height: u32 = 11;

        let expected: HashMap<_, _> = vec![
            (Move::Up, true),
            (Move::Down, true),
            (Move::Left, false),
            (Move::Right, true),
        ]
        .into_iter()
        .collect();

        set_moves_inbound(&mut is_move_safe, &Coord { x: 0, y: 5 }, &width, &height);

        assert_eq!(is_move_safe, expected)
    }
}

#[cfg(test)]
mod tests_self_collitions {
    use crate::logic::test_helpers::test_get_battlesnake_with_health;

    use super::*;

    #[test]
    fn test_no_collide_with_tail() {
        let you = test_get_battlesnake_with_health(40);

        let mut is_move_safe = get_initial_moves();

        let expected: HashMap<_, _> = vec![
            (Move::Up, false),
            (Move::Down, false),
            (Move::Left, true),
            (Move::Right, true),
        ]
        .into_iter()
        .collect();

        set_moves_collide_self(&mut is_move_safe, &you);

        assert_eq!(is_move_safe, expected)
    }
}

#[cfg(test)]
mod tests_snake_moves {
    use crate::logic::test_helpers::test_get_battlesnake_with_health;

    use super::*;

    #[test]
    fn test_move_no_food_right() {
        let you = test_get_battlesnake_with_health(40);

        let expected = Battlesnake {
            id: "you".to_string(),
            name: "you".to_string(),
            health: 39,
            body: vec![
                Coord { x: 2, y: 0 },
                Coord { x: 1, y: 0 },
                Coord { x: 1, y: 1 },
                Coord { x: 1, y: 2 },
                Coord { x: 2, y: 2 },
                Coord { x: 2, y: 1 },
            ],
            head: Coord { x: 2, y: 0 },
            length: 6,
            latency: "100".to_string(),
            shout: None,
        };

        let board = Board {
            height: 11,
            width: 11,
            food: vec![],
            snakes: vec![test_get_battlesnake_with_health(40)],
            hazards: vec![],
        };

        let new_snake = move_snake(&board, you, &Move::Right);

        assert_eq!(new_snake, expected)
    }

    #[test]
    fn test_move_with_food_left() {
        let you = test_get_battlesnake_with_health(100);

        let expected = Battlesnake {
            id: "you".to_string(),
            name: "you".to_string(),
            health: 99,
            body: vec![
                Coord { x: 0, y: 0 },
                Coord { x: 1, y: 0 },
                Coord { x: 1, y: 1 },
                Coord { x: 1, y: 2 },
                Coord { x: 2, y: 2 },
                Coord { x: 2, y: 1 },
                Coord { x: 2, y: 0 },
            ],
            head: Coord { x: 0, y: 0 },
            length: 7,
            latency: "100".to_string(),
            shout: None,
        };

        let board = Board {
            height: 11,
            width: 11,
            food: vec![Coord { x: 1, y: 0 }],
            snakes: vec![test_get_battlesnake_with_health(100)],
            hazards: vec![],
        };

        let new_snake = move_snake(&board, you, &Move::Left);

        assert_eq!(new_snake, expected)
    }
}

#[cfg(test)]
mod tests_maximise {
    use super::*;

    #[test]
    fn test_double_tail() {
        let board = Board {
            height: 5,
            width: 5,
            food: vec![Coord { x: 0, y: 0 }, Coord { x: 2, y: 2 }],
            snakes: vec![Battlesnake {
                id: "4bcd1e18-c05c-43d6-98fa-2a6b5eea9a2a".to_owned(),
                name: "Rust Starter Project".to_owned(),
                health: 100,
                body: vec![
                    Coord { x: 1, y: 2 },
                    Coord { x: 1, y: 1 },
                    Coord { x: 1, y: 0 },
                    Coord { x: 2, y: 0 },
                    Coord { x: 2, y: 1 },
                    Coord { x: 3, y: 1 },
                    Coord { x: 3, y: 0 },
                    Coord { x: 4, y: 0 },
                    Coord { x: 4, y: 1 },
                    Coord { x: 4, y: 2 },
                    Coord { x: 3, y: 2 },
                    Coord { x: 3, y: 3 },
                    Coord { x: 4, y: 3 },
                    Coord { x: 4, y: 4 },
                    Coord { x: 3, y: 4 },
                    Coord { x: 2, y: 4 },
                    Coord { x: 2, y: 3 },
                    Coord { x: 1, y: 3 },
                    Coord { x: 1, y: 4 },
                    Coord { x: 0, y: 4 },
                    Coord { x: 0, y: 3 },
                ],
                head: Coord { x: 1, y: 2 },
                length: 21,
                latency: "3".to_owned(),
                shout: None,
            }],
            hazards: vec![],
        };

        let you = Battlesnake {
            id: "4bcd1e18-c05c-43d6-98fa-2a6b5eea9a2a".to_owned(),
            name: "Rust Starter Project".to_owned(),
            health: 100,
            body: vec![
                Coord { x: 1, y: 2 },
                Coord { x: 1, y: 1 },
                Coord { x: 1, y: 0 },
                Coord { x: 2, y: 0 },
                Coord { x: 2, y: 1 },
                Coord { x: 3, y: 1 },
                Coord { x: 3, y: 0 },
                Coord { x: 4, y: 0 },
                Coord { x: 4, y: 1 },
                Coord { x: 4, y: 2 },
                Coord { x: 3, y: 2 },
                Coord { x: 3, y: 3 },
                Coord { x: 4, y: 3 },
                Coord { x: 4, y: 4 },
                Coord { x: 3, y: 4 },
                Coord { x: 2, y: 4 },
                Coord { x: 2, y: 3 },
                Coord { x: 1, y: 3 },
                Coord { x: 1, y: 4 },
                Coord { x: 0, y: 4 },
                Coord { x: 0, y: 3 },
            ],
            head: Coord { x: 1, y: 2 },
            length: 21,
            latency: "3".to_owned(),
            shout: None,
        };

        let left_moved_snake = move_snake(&board, you.clone(), &Move::Left);
        println!("left_moved_snake {:?}", left_moved_snake);

        let right_moved_snake = move_snake(&board, you, &Move::Right);
        println!("right_moved_snake {:?}", right_moved_snake);

        println!("maximise: {}", maximise(&board, left_moved_snake, 20));
        println!("maximise: {}", maximise(&board, right_moved_snake, 20));
    }

    #[test]
    fn test_shouldn_eat() {
        let board = Board {
            height: 5,
            width: 5,
            food: vec![
                Coord { x: 0, y: 0 },
                Coord { x: 0, y: 1 },
                Coord { x: 3, y: 3 },
            ],
            snakes: vec![Battlesnake {
                id: "5b08e492-e453-42c5-94c1-8e56277126c8".to_owned(),
                name: "Rust Starter Project".to_owned(),
                health: 64,
                body: vec![
                    Coord { x: 1, y: 1 },
                    Coord { x: 1, y: 2 },
                    Coord { x: 0, y: 2 },
                    Coord { x: 0, y: 3 },
                    Coord { x: 0, y: 4 },
                    Coord { x: 1, y: 4 },
                    Coord { x: 1, y: 3 },
                    Coord { x: 2, y: 3 },
                    Coord { x: 2, y: 4 },
                    Coord { x: 3, y: 4 },
                    Coord { x: 4, y: 4 },
                    Coord { x: 4, y: 3 },
                    Coord { x: 4, y: 2 },
                    Coord { x: 4, y: 1 },
                    Coord { x: 4, y: 0 },
                    Coord { x: 3, y: 0 },
                    Coord { x: 3, y: 1 },
                    Coord { x: 3, y: 2 },
                    Coord { x: 2, y: 2 },
                    Coord { x: 2, y: 1 },
                    Coord { x: 2, y: 0 },
                ],
                head: Coord { x: 1, y: 1 },
                length: 21,
                latency: "5".to_owned(),
                shout: None,
            }],
            hazards: vec![],
        };

        let you = Battlesnake {
            id: "5b08e492-e453-42c5-94c1-8e56277126c8".to_owned(),
            name: "Rust Starter Project".to_owned(),
            health: 64,
            body: vec![
                Coord { x: 1, y: 1 },
                Coord { x: 1, y: 2 },
                Coord { x: 0, y: 2 },
                Coord { x: 0, y: 3 },
                Coord { x: 0, y: 4 },
                Coord { x: 1, y: 4 },
                Coord { x: 1, y: 3 },
                Coord { x: 2, y: 3 },
                Coord { x: 2, y: 4 },
                Coord { x: 3, y: 4 },
                Coord { x: 4, y: 4 },
                Coord { x: 4, y: 3 },
                Coord { x: 4, y: 2 },
                Coord { x: 4, y: 1 },
                Coord { x: 4, y: 0 },
                Coord { x: 3, y: 0 },
                Coord { x: 3, y: 1 },
                Coord { x: 3, y: 2 },
                Coord { x: 2, y: 2 },
                Coord { x: 2, y: 1 },
                Coord { x: 2, y: 0 },
            ],
            head: Coord { x: 1, y: 1 },
            length: 21,
            latency: "5".to_owned(),
            shout: None,
        };

        println!(
            "score for move_left {}",
            maximise(&board, move_snake(&board, you.clone(), &Move::Left), 20)
        );
        println!(
            "score for move_down {}",
            maximise(&board, move_snake(&board, you, &Move::Down), 20)
        );
    }
}
