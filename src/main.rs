#![allow(unused_variables)]
#![allow(dead_code)]

use std::io;

fn get_input_from_index() -> Result<Option<usize>, String> {
    let mut input = String::new();
    let _ = io::stdin()
        .read_line(&mut input)
        .map_err(|e| e.to_string())?;
    /*
       here we added ? so if the result is of type error this will return it
    */

    /*
     * here we are trimming the input
     * and the input variable is following shadowing concept
     */
    let input = input.trim();
    if input == "quit" {
        return Ok(None);
    }
    /*
        here we are converting the string to integer
        also here i am telling the input should be an integer of type usize
        .map_err(|_e| format!("input should be an integer"))
       is to say ignore the error and return the string
        input.parse::<usize>()
    */
    let index = input
        .parse::<usize>()
        .map_err(|_e| format!("input should be an integer"))?;
    if index < 1 || index > 9 {
        return Err(format!("input should be between 1 and 9"));
    }
    /*
    as the indexing starting from 0 and we taking value from 1 to 9
    so i will subtract 1 from the index
    */
    Ok(Some(index - 1))
    // todo!()
}
fn print_board(board: [char; 9]) {
    println!(
        "
     + - - - + - - - + - - - +
     |   {}   |   {}   |    {}  |
     + - - - + - - - + - - - +
     |   {}   |   {}   |    {}  |
     + - - - + - - - + - - - +
     |   {}   |   {}   |    {}  |
     + - - - + - - - + - - - +
    ",
        board[0], board[1], board[2], board[3], board[4], board[5], board[6], board[7], board[8]
    );
}
fn main() {
    // this board will contain X , O ,' '
    let mut board = [' '; 9];
    let players = ['X', 'O'];
    let mut turn = 0;
    // the updatable varibale use mut for initialistaion
    // board[0] = 'X';
    // board[4] = 'X';
    // board[8] = 'X';

    print_board(board);
    // here i am taking the input from the user
    loop {
        println!("enter position for {}", players[turn]);
        let index = get_input_from_index();
        if let Err(e) = index {
            println!("{}", e);
            continue;
        }
        // to use pl and none
        let index = index.unwrap();

        if let None = index {
            break;
        }
        let index = index.unwrap();
        /*
        to have alternamte values
        */
        board[index] = players[turn];
        print_board(board);
        turn = (turn + 1) % 2;
    }
}
