use rand::Rng;
use std::io;
use colored::*;

type GameState = [i8; 9];

const X: i8 = -1;
const O: i8 = 1;
const EMPTY: i8 = 0;

const MAX: f32 = 1000.0;
const MIN: f32 = -1.0 * MAX;


struct Node {
    game_state: GameState,
    value: f32,
    children: Vec<Node>
}

fn minimax_rec(
    node: &mut Node,
    player: i8,
    is_max: bool,
    level: u8,
    a: f32,
    b: f32
) -> f32 {
    let mut a = a;
    let mut b = b;
    let children = get_next_moves(node.game_state);

    // base case
    if children.len() == 0 {
        let winner = check_winner(node.game_state).unwrap();
        let mut val = -1.0 / level as f32;
        if winner == player {
            val = 1.0 / level as f32;
        } else if winner == EMPTY {
            val = 0.0;
        }
        return val;
    }

    let mut value = MAX;
    if is_max {
        value = MIN;
    }

    for child_state in children {
        let mut child = Node{
            game_state: child_state,
            value: 0.0,
            children: vec![]
        };

        let eval = minimax_rec(
            &mut child, 
            player, 
            !is_max, 
            level + 1,
            a,
            b
        );

        if is_max && eval > value {
			value = eval
		} else if !is_max && eval < value {
			value = eval
		}

        child.value = eval;
        node.children.push(child);

        if is_max {
			if value > a {
				a = value
			}
			if value >= b {
				break
			}
		} else {
			if value < b {
				b = value
			}
			if value <= a {
				break
			}
		}
    }

    value
}

fn minimax(game_state: GameState) -> Option<GameState> {
    let player = whos_turn(game_state);
    let mut node = Node{
        game_state,
        value: 0.0,
        children: vec![]
    };
    let best = minimax_rec(&mut node, player, true, 0, MIN, MAX);

    for child in node.children {
        if child.value == best {
            return Some(child.game_state);
        }
    }

    None
}

fn is_winnable(game_state: GameState) -> bool {
    let winner = check_winner(game_state);

    if winner != None && winner != Some(EMPTY) {
        return true;
    }

    let children = get_next_moves(game_state);
    
    if children.len() == 0 {
        return false;
    }

    for child in children {
        if is_winnable(child) {
            return true;
        }
    }

    false
}

fn print_game(game_state: GameState) {
    clear_console();

    for (i, num) in game_state.iter().enumerate() {
        let str = match num {
            &X => String::from("X"),
            &O => String::from("O"),
            _ => i.to_string().purple().to_string(),
        };

        if (i+1) % 3 == 0 {
            print!("{}\n", str);
        } else {
            print!("{}|", str);
        }
    }
}

fn whos_turn(game_state: GameState) -> i8 {
    let mut i = 0;

    for num in game_state {
        if num != EMPTY {
            i += 1;
        }
    }

    if i == 9 {
        // game over
        return EMPTY;
    } else if i % 2 == 0 {
        // X on even turns
        return X;
    } else {
        // O on odd turns
        return O;
    }
}

fn get_next_moves(game_state: GameState) -> Vec<GameState> {
    let mut rng = rand::thread_rng();
    
    let mut a: Vec<GameState> = vec![];
    let mut b: Vec<GameState> = vec![];

    if check_winner(game_state) != None {
        return a;
    }

    let player = whos_turn(game_state);

    for (i, num) in game_state.iter().enumerate() {
        if *num == EMPTY {
           let mut new_game_state = game_state.clone();
           new_game_state[i] = player;

           if rng.gen_range(0..=1) == 0 {
            a.push(new_game_state);
           } else {
            b.push(new_game_state);
           }
        }
    }

    a.append(&mut b);
    a
}

fn check_winner(game_state: GameState) -> Option<i8> {
    let winning_cell_combinations = [
		// horizontal
		[0, 1, 2],
		[3, 4, 5],
		[6, 7, 8],
		// vertical
		[0, 3, 6],
		[1, 4, 7],
		[2, 5, 8],
		// diagnol
		[0, 4, 8],
		[2, 4, 6],
    ];

    for combination in winning_cell_combinations {
        let mut won = true;
        let player = game_state[combination[0]];

        for index in combination {
            if game_state[index] != player {
                won = false;
                break;
            }
        }

        if won && player != EMPTY {
            // someone won
            return Some(player);
        }
    }

    for cell in game_state {
        if cell == EMPTY {
            // game not finished
            return None;
        }
    }

    // draw
    Some(EMPTY)
}

fn play_move(game_state: GameState, index: usize) -> GameState {
    let mut game_state = game_state;
    let player = whos_turn(game_state);
    game_state[index] = player;
    game_state
}

fn get_move(game_state: GameState) -> GameState {
    println!("Enter index for next move: ");
    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index = index.trim().parse::<usize>();

    if let Err(_) = &index {
        println!("Invalid input");
        return get_move(game_state);
    }

    let index = index.unwrap();

    if index > 8 || game_state[index] != EMPTY {
        println!("Illegal move");
        return get_move(game_state);
    } else {
        return play_move(game_state, index);
    }
}

fn clear_console() {
    print!("{}[2J", 27 as char);
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

fn get_player() -> i8 {
    clear_console();
    println!("Enter player (X,O)");
    let mut player = String::new();

    io::stdin()
        .read_line(&mut player)
        .expect("Failed to read line"); 

    let player = player.trim().to_uppercase();

    if player == "O" {
        return O;
    }

    X
}

fn main() {
    let mut game: GameState = [EMPTY,EMPTY,EMPTY,EMPTY,EMPTY,EMPTY,EMPTY,EMPTY,EMPTY];

    let player = match get_player() {
        O => 1,
        _ => 0
    };

    print_game(game);

    let mut i = 0;
    while check_winner(game) == None && is_winnable(game) {
        if i % 2 == player {
            game = get_move(game);
        } else {
            game = minimax(game).unwrap();
        }
        print_game(game);
        i += 1;
    }

    let winner = match check_winner(game) {
        Some(X) => "X wins",
        Some(O) => "O wins",
        Some(EMPTY) => "draw",
        _ => "draw (not winnable)"
    };

    println!("{}", winner);
}