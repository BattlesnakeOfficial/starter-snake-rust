// Welcome to
//
//             ____                ____              _
//            | __ )  ___   __ _  / ___| _ __   __ _| | _____
//            |  _ \ / _ \ / _` | \___ \| '_ \ / _` | |/ / _ \
//            | |_) | (_) | (_| |  ___) | | | | (_| |   <  __/
//            |____/ \___/ \__,_| |____/|_| |_|\__,_|_|\_\___|
//
//
// For more info see docs.battlesnake.com

use std::collections::HashMap;
use std::collections::VecDeque;
use std::time::Instant;

use log::info;
use par_map::ParMap;
use rocket::form::validate::Contains;
use serde_json::{json, Value};

use crate::{Battlesnake, Board, Coord, Game, Move};

#[derive(Debug)]
struct MinimaxNode {
    you: Battlesnake,
    turn: u32,
}

// move is called on every turn and returns your next move
// Valid moves are "up", "down", "left", or "right"
// See https://docs.battlesnake.com/api/example-move for available data
pub fn get_move(game: &Game, turn: &u32, board: &Board, you: &Battlesnake) -> Value {
    let start_time = Instant::now();

    debug!("turn: {} - Game: {:?}", turn, game);
    debug!("turn: {} - board: {:?}", turn, board);
    debug!("turn: {} - you: {:?}", turn, you);

    let board = board.clone();
    let mut you = you.clone();
    let timeout = game.timeout as u128;

    if you.length > 2 && you.body[you.length as usize - 2] == you.body[you.length as usize - 1usize]
    {
        you.body.pop();
        you.length = you.body.len() as u32;
    }

    let safe_moves = available_moves(&board, &you);
    let scored_moves: HashMap<Move, i32> = safe_moves
        .iter()
        .cloned()
        .par_map(move |mv| {
            let moved_snake = move_snake(&board, &you, &mv);
            // debug!("ROOT move: {:?}", mv);
            (
                mv,
                maximise(&board, &moved_snake, &start_time, timeout - 100),
            )
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

fn move_snake(board: &Board, snake: &Battlesnake, direction: &Move) -> Battlesnake {
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

    // set_moves_has_collided_with_tail(&mut is_move_safe, you);

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

fn score_position(board: &Board, you: &Battlesnake) -> i32 {
    let food_value = 1;
    let space_value = 2;
    let mut score = 0;
    let safe_moves = available_moves(board, you);
    let will_eat = board.food.contains(you.head);
    let health_threshold = 5; // Adjust this threshold based on your game rules
    let killing_ratio = 20;

    score += space_value * safe_moves.len() as i32;

    if will_eat && you.health <= health_threshold {
        score += food_value; // Add extra score for eating when health is low
    } else if will_eat {
        score -= if score + food_value > i32::MIN {
            food_value
        } else {
            0
        }
    }

    // check head collisions
    for snake in &board.snakes {
        if snake.id != you.id {
            if snake.head == you.head && you.length > snake.length {
                score += killing_ratio
            } else if snake.head == you.head && you.length > snake.length && board.snakes.len() == 2
            {
                score = i32::MAX;
            }
            // else if snake.body.contains(you.head) {
            //     score -= killing_ratio;
            // }
        }
    }

    // if you.health > health_threshold && you.health < health_threshold + 10 {
    //     score += 20;
    // }

    if Some(&you.head) == you.body.last() || safe_moves.is_empty() || you.health == 0 {
        score = i32::MIN;
    }

    score
}

fn maximise(board: &Board, you: &Battlesnake, start_date: &Instant, max_duration: u128) -> i32 {
    let possible_moves = available_moves(board, you);
    let mut max_score = score_position(board, you);

    if possible_moves.is_empty() || max_score == i32::MIN {
        return max_score;
    }

    let mut stack: VecDeque<MinimaxNode> = VecDeque::new();
    for mv in possible_moves {
        stack.push_back(MinimaxNode {
            you: move_snake(board, you, &mv),
            turn: 1,
        });
    }

    let mut duration = start_date.elapsed().as_millis();

    while let Some(node) = stack.pop_front() {
        if duration >= max_duration {
            break;
        }
        let score = score_position(board, &node.you);
        max_score = max_score.max(score + node.turn as i32);

        if score != i32::MIN {
            for mv in available_moves(board, &node.you) {
                let moved_snake = move_snake(board, &node.you, &mv);
                stack.push_back(MinimaxNode {
                    you: moved_snake,
                    turn: node.turn + 1,
                });
            }
        }
        duration = start_date.elapsed().as_millis();
    }

    max_score
}

// fn maximise(board: &Board, you: &Battlesnake, start_date: &Instant, max_duration: u128) -> i32 {
//     let possible_moves = available_moves(board, you);
//     let mut max_score = score_position(board, you);

//     if possible_moves.is_empty() || max_score == i32::MIN {
//         return max_score;
//     }

//     // Limit the number of threads to 4
//     let num_threads = 4;
//     let thread_pool = Arc::new(Mutex::new(VecDeque::new()));

//     let mut stack: VecDeque<MinimaxNode> = VecDeque::new();
//     for mv in possible_moves {
//         stack.push_back(MinimaxNode {
//             you: move_snake(board, you, &mv),
//             turn: 1,
//         });
//     }

//     let duration = start_date.elapsed().as_millis();

//     while !stack.is_empty() {
//         if duration >= max_duration {
//             break;
//         }

//         let node = stack.pop_front().unwrap();
//         let thread_pool_clone = Arc::clone(&thread_pool);
//         let board_clone = board.clone();
//         let start_date_clone = *start_date;

//         let handle = thread::spawn(move || {
//             let score = score_position(&board_clone, &node.you);
//             let mut max_score = score + node.turn as i32;

//             if score != i32::MIN {
//                 for mv in available_moves(&board_clone, &node.you) {
//                     let moved_snake = move_snake(&board_clone, &node.you, &mv);
//                     thread_pool_clone.lock().unwrap().push_back(MinimaxNode {
//                         you: moved_snake,
//                         turn: node.turn + 1,
//                     });
//                 }
//             }
//         });

//         handle.join().unwrap();

//         // Update duration after each thread finishes
//         let duration = start_date_clone.elapsed().as_millis();
//     }

//     // Get the maximum node from the thread pool based on score
//     let max_node = thread_pool
//         .lock()
//         .unwrap()
//         .iter()
//         .cloned()
//         .max_by_key(|node| node.turn)
//         .unwrap_or(MinimaxNode {
//             you: you.clone(),
//             turn: 0,
//         });

//     max_node.turn as i32
// }

// fn maximise(board: &Board, you: &Battlesnake, start_date: &Instant, max_duration: u128, turn: u32) -> i32 {
//     let possible_moves = available_moves(board, you);
//
//     if possible_moves.is_empty() || score_position(board, you) == i32::MIN || start_date.elapsed().as_millis() >= max_duration {
//         return score_position(board, you) + turn as i32;
//     }
//
//     // let mut max_score = i32::MIN;
//     // for mv in possible_moves {
//     //     let moved_snake = move_snake(board, you.clone(), &mv); // Update the snake's position
//     //     let score = maximise(board, moved_snake, start_date, max_duration, turn + 1); // Pass the updated snake to the recursive call
//     //     // debug!("move: {:?}, score: {}, level: {}", mv, score, 10 - depth);
//     //     max_score = max_score.max(score);
//     // }
//     // max_score
//
//     possible_moves
//         .iter()
//         .map(|mv| {
//             let moved_snake = move_snake(board, you, &mv);
//             maximise(board, &moved_snake, start_date, max_duration, turn + 1)
//         })
//         .max()
//         .unwrap_or(i32::MIN)
// }

// use rayon::prelude::*;

// fn minimax(board: &Board, you: Battlesnake, depth: u32, maximizing_player: bool) -> i32 {
//     let possible_moves = available_moves(board, &you);

//     if depth == 0 || possible_moves.is_empty() || score_position(board, &you) == i32::MIN {
//         return score_position(board, &you);
//     }

//     if maximizing_player {
//         // Maximize using parallel processing
//         possible_moves
//             .into_par_iter()
//             .map(|mv| {
//                 let moved_snake = move_snake(board, you.clone(), &mv);
//                 minimax(board, moved_snake, depth - 1, false)
//             })
//             .max()
//             .unwrap_or(i32::MIN)
//     } else {
//         // Minimize for each opponent snake using parallel processing
//         board
//             .snakes
//             .par_iter()
//             .filter(|opponent_snake| opponent_snake.id != you.id)
//             .flat_map(|opponent_snake| {
//                 available_moves(board, opponent_snake)
//                     .into_par_iter()
//                     .map(move |opponent_move| {
//                         let moved_opponent_snake = move_snake(board, opponent_snake.clone(), &opponent_move);
//                         -1 * minimax(board, moved_opponent_snake, depth - 1, true)
//                     })
//                     .min()
//                     .unwrap_or(i32::MAX)
//             })
//             .max()
//             .unwrap_or(i32::MIN)
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

        let new_snake = move_snake(&board, &you, &Move::Right);

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

        let new_snake = move_snake(&board, &you, &Move::Left);

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

        let start = Instant::now();
        let left_moved_snake = move_snake(&board, &you, &Move::Left);
        println!("left_moved_snake {:?}", left_moved_snake);

        let right_moved_snake = move_snake(&board, &you, &Move::Right);
        println!("right_moved_snake {:?}", right_moved_snake);

        println!(
            "maximise_left: {}",
            maximise(&board, &left_moved_snake, &start, 200000)
        );
        println!(
            "maximise_right: {}",
            maximise(&board, &right_moved_snake, &start, 200000)
        );
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

        let start_time = Instant::now();
        println!(
            "score for move_left {}",
            maximise(
                &board,
                &move_snake(&board, &you, &Move::Left),
                &start_time,
                20000,
            )
        );
        println!(
            "score for move_down {}",
            maximise(
                &board,
                &move_snake(&board, &you, &Move::Down),
                &start_time,
                20000,
            )
        );
    }
}
