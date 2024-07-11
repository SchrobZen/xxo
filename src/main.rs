use std::fmt::{Display, Formatter, Result};
use std::io;

//add a board to display the state of the game
#[derive(Debug)]
struct Board {
    place_holder: [char; 9],
}

//add a struct to output all information that needs to be updated after a turn
struct Turn {
    turn_save: [u32;5],
    win_condition_save: [u32;8],
    board_save : Board
}

//function to add the values in two arrays position by position
fn array_add(l: [u32; 8], r: [u32; 8]) -> [u32; 8] {
    let mut t = [0; 8];
    for n in 0..8 {
        t[n] += l[n] + r[n];
    }
    t
}

//checking if someone won, it's a tie or the game should continue 
fn end_game_check(win_condition_x: [u32; 8], win_condition_o: [u32; 8]) -> bool {
    let txt_x_win = "x wins";
    let txt_o_win = "o wins";
    let txt_tie = "it's a tie";
    let check_win: Option<char> = if win_condition_x.iter().copied().max().unwrap() == 3 {
                                      Some('x')
                                    } else if win_condition_o.iter().copied().max().unwrap() == 3 {
                                      Some('o')
                                    } else if array_add(win_condition_x, win_condition_o) == [3;8] {
                                      Some('t')
                                    } else {
                                      None
                                    };
    if check_win != None {
        match check_win {
            Some('x') => println!("{}", txt_x_win),
            Some('o') => println!("{}", txt_o_win),
            Some('t') => println!("{}", txt_tie),
            _ => println!("error 5")
        }
    };
    check_win !=None
}

//a function that handels a turn beeing taken and puts out a Turn with the data to update the state of the game
fn take_turn(mut win_condition_player: [u32; 8], mut turn_player: [u32;5], turn_opponent: [u32;5], player_name: &str, mut board: Board, round: usize) -> Turn {
    
    //massages to the player
    let txt_pos_taken = "This position is already taken. Choose a different one.";
    let txt_move = "Your move, ";
    let txt_invalid_input = "Only numbers 0 through 8 are valid input. You put ";
    
    //list of positions and the impact of each position to the possible win scenarios
    //position = [is_top_row, is_mid_row, is_bott_row, is_left_column, is_mid_column, is_right_column, is_dia_top_left_bott_right, is_dia_top_right_bott_left]
    let top_left = [1, 0, 0, 1, 0, 0, 1, 0];
    let top_mid = [1, 0, 0, 0, 1, 0, 0, 0];
    let top_right = [1, 0, 0, 0, 0, 1, 0, 1];
    let mid_left = [0, 1, 0, 1, 0, 0, 0, 0];
    let mid_mid = [0, 1, 0, 0, 1, 0, 1, 1];
    let mid_right = [0, 1, 0, 0, 0, 1, 0, 0];
    let bott_left = [0, 0, 1, 1, 0, 0, 0, 1];
    let bott_mid = [0, 0, 1, 0, 1, 0, 0, 0];
    let bott_right = [0, 0, 1, 0, 0, 1, 1, 0];
    let win_condition =[
        top_left, top_mid, top_right, mid_left, mid_mid, mid_right, bott_left, bott_mid, bott_right,
    ];
    //define allowed input 
    let allowed_input: [String;9] = ["0\n".to_string(), "1\n".to_string(), "2\n".to_string(), "3\n".to_string(), "4\n".to_string(), "5\n".to_string(), "6\n".to_string(), "7\n".to_string(), "8\n".to_string()];
    
    //stores the input
    let mut input;
    println!("{}{}", txt_move, player_name);
        
        //gets the input and checks validity
        loop {
            input = String::new();
            io::stdin().read_line(&mut input).expect("input error 1");
            
            //is the input allowed?
            if allowed_input.contains(&input) {
                let input_clone = input.clone();
                let input_clone_num: u32 = input_clone.trim().parse().expect("input error 2");
                //is the position already taken?
                if turn_player.contains(&input_clone_num) || turn_opponent.contains(&input_clone_num) {
                    println!("{}",txt_pos_taken);
                    continue
                } else {
                    break
                }
            } else {
                println!("{}{}", txt_invalid_input, input);
                continue
            }
        }
        //if valid turn into integer and store 
        turn_player[round]= input.trim().parse().expect("input error 3");
        let position = turn_player[round] as usize;
        //store move on the board
        board.place_holder[position] = if player_name == "x" {'x'} else {'o'};
        //store in array for win conditions
        win_condition_player = array_add(win_condition_player, win_condition[position]);
        //print the new state of the game
        println!("{}", board);
        //store the data in the Turn struct
        let turn = Turn {
            turn_save: turn_player,
            win_condition_save: win_condition_player,
            board_save: board
        };
        //make that Turn struct the output 
        turn
}

//implementing display method to correctly show the state of the game in the console
impl Display for Board {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "{} {} {}\n{} {} {}\n{} {} {}",
            self.place_holder[0],
            self.place_holder[1],
            self.place_holder[2],
            self.place_holder[3],
            self.place_holder[4],
            self.place_holder[5],
            self.place_holder[6],
            self.place_holder[7],
            self.place_holder[8]
        )
    }
}

fn main() {

    //parts of the Turn struct:
    //arrays where the turns will be stored
    let mut turn_x = [9; 5];
    let mut turn_o = [9; 5];
    
    //arrays where the win conditions will be stored
    let mut win_condition_x = [0; 8];
    let mut win_condition_o = [0; 8];
    
    //empty board 
    let mut board: Board = Board {
        place_holder: ['•'; 9],
    };
    //visual guide to what input means what position 
    let show_positions = Board {
        place_holder: ['0', '1', '2', '3', '4', '5', '6', '7', '8'],
    };
    //additional massages to the player not in the turn
    let txt_show_pos = "Choose from the following positions:";

    // start with visual guide
    println!("{}\n{}",txt_show_pos, show_positions);


    //start the game each for-loop is both players
    for n in 0..5 {
        //store the taken turn
        let take_turn_x = take_turn(win_condition_x, turn_x, turn_o, "x", board, n);
        
        //update the data outside this turn
        turn_x = take_turn_x.turn_save;
        win_condition_x = take_turn_x.win_condition_save;
        board = take_turn_x.board_save;
        
        //check of the turn ended the game
        if end_game_check(win_condition_x, win_condition_o) {
            break
        }
        
        //repeat the same for player 2 
        //player 2 can only take 4 turns player one takes 5 turns if all 9 positions are being filled
        if n < 4 {
        let take_turn_o = take_turn(win_condition_o, turn_o, turn_x, "o", board, n);
            
            turn_o = take_turn_o.turn_save;
            win_condition_o = take_turn_o.win_condition_save;
            board = take_turn_o.board_save;
            
        if end_game_check(win_condition_x, win_condition_o) {
            break
        }
        }
    }
}
