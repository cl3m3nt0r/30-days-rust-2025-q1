use rand::Rng;
use std::io;

#[derive(PartialEq, Eq)]
enum Move {
    ROCK,
    PAPER,
    SCISSORS,
}
impl Move {
    fn compare_moves(&mut self, other_move: Move) {
        match self {
            Move::ROCK => match other_move {
                Move::ROCK => println!("It's a tie!"),
                Move::PAPER => println!("You lose! Paper beats Rock."),
                Move::SCISSORS => println!("You win! Rock beats Scissors."),
            },
            Move::PAPER => match other_move {
                Move::ROCK => println!("You win! Paper beats Rock."),
                Move::PAPER => println!("It's a tie!"),
                Move::SCISSORS => println!("You lose! Scissors beats Paper."),
            },
            Move::SCISSORS => match other_move {
                Move::ROCK => println!("You lose! Rock beats Scissors."),
                Move::PAPER => println!("You win! Scissors beats Paper."),
                Move::SCISSORS => println!("It's a tie!"),
            },
        }
    }
}
fn computer_move() -> Move {
    let mut range = rand::thread_rng();
    let random_number = range.gen_range(1..=3);
    match random_number {
        1 => Move::ROCK,
        2 => Move::PAPER,
        3 => Move::SCISSORS,
        _ => unreachable!(),
    }
}
fn main() {
    let mut no_of_times = 5;

    println!("Welcome to Rock-Paper-Scissors!");

    while no_of_times > 0 {
        let mut player_move = String::new();

        println!("\nEnter your move (rock, paper, or scissors):");
        io::stdin()
            .read_line(&mut player_move)
            .expect("Error reading your input");

        let mut player_move = match player_move.trim().to_lowercase().as_str() {
            "rock" => Move::ROCK,
            "paper" => Move::PAPER,
            "scissors" => Move::SCISSORS,
            _ => {
                eprintln!("Invalid move. Please try again.");
                continue;
            }
        };
        let computer_move = computer_move();
        player_move.compare_moves(computer_move);
        no_of_times -= 1;
    }
}
