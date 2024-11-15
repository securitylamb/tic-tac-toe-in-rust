Tic-Tac-Toe in Rust
A simple command-line implementation of Tic-Tac-Toe in Rust, where two players alternate placing "X" and "O" on a 3x3 board. The program accepts user input for selecting positions and handles errors like invalid input or out-of-range positions.

Features
Players take turns to input their move by specifying the position on a 3x3 grid.
The game alternates between two players, "X" and "O".
The input is validated to ensure:
The position is between 1 and 9 (inclusive).
The position is not already occupied.
If the user types quit, the game will end.
Simple error handling for invalid input.

Getting Started
Prerequisites
Ensure you have the following installed:

Rust (with cargo)
Installation
Clone the repository:

bash
Copy code
git clone https://github.com/securitylamb/tic-tac-toe-in-rust.git
cd tic-tac-toe-in-rust
Running the Game
To run the game, use the following command:

bash
Copy code
cargo run
This will compile the program and start the game in your terminal. Players will be prompted to enter their moves. To quit the game, type quit and press enter.

Example Gameplay
arduino
Copy code
     + - - - + - - - + - - - +
     |   ' '   |   ' '   |    ' '  |
     + - - - + - - - + - - - +
     |   ' '   |   ' '   |    ' '  |
     + - - - + - - - + - - - +
     |   ' '   |   ' '   |    ' '  |
     + - - - + - - - + - - - +

Enter position for X: 1
     + - - - + - - - + - - - +
     |   X   |   ' '   |    ' '  |
     + - - - + - - - + - - - +
     |   ' '   |   ' '   |    ' '  |
     + - - - + - - - + - - - +
     |   ' '   |   ' '   |    ' '  |
     + - - - + - - - + - - - +

Enter position for O: 5
     + - - - + - - - + - - - +
     |   X   |   ' '   |    ' '  |
     + - - - + - - - + - - - +
     |   ' '   |   O   |    ' '  |
     + - - - + - - - + - - - +
     |   ' '   |   ' '   |    ' '  |
     + - - - + - - - + - - - +

... and so on
Input Format
The board positions are numbered 1 to 9 as follows:
markdown
Copy code
1 | 2 | 3
---------
4 | 5 | 6
---------
7 | 8 | 9
Players must enter a number between 1 and 9 to make a move.
If a player tries to place a mark in an already occupied spot, they will be prompted to choose a different position.
Error Handling
Invalid input (e.g., letters or numbers outside the 1-9 range) will result in an error message, and the player will be asked to try again.
If the user types quit, the game will end.****
