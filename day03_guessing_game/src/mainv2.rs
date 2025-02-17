use rand::Rng;
use std::cmp::Ordering;
use std::fs;
use std::io;
use std::io::Write;

fn write_high_score(high_score: i32) {
    let system_score = fs::read_to_string("highscore").expect("Error reading file");
    if system_score
        .trim()
        .parse::<i32>()
        .expect("Unexpected value")
        <= high_score
    {
        fs::write("highscore", high_score.to_string()).expect("Error writing to file");
    };
}
fn read_high_score() -> i32 {
    let score = match fs::read_to_string("highscore") {
        Ok(n) => n,
        Err(_e) => {
            let mut file = fs::File::create("highscore").expect("Error creating file");
            file.write_all(b"0").expect("Error writing default score"); // Optionally write a default value
            "0".to_string() // Return default value
        }
    };
    match score.trim().parse() {
        Ok(n) => return n,
        Err(_e) => return 0,
    };
}
fn play() {
    // let mut high_score = read_high_score();
    let mut high_score = 0;
    let mut secret_number = rand::thread_rng().gen_range(1..=100);
    println!("Secret number is {secret_number}");
    println!("Guess the number!");

    let mut no_tries = 3;
    loop {
        if no_tries > 0 {
            println!("Please input your guess.");
            println!("You have {no_tries} guess(es) left");

            let mut guess = String::new();
            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line");

            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Invalid input! Please enter a valid number.");
                    continue;
                }
            };

            println!("You guessed: {}", guess);

            match guess.cmp(&secret_number) {
                Ordering::Less => println!("Too small!"),
                Ordering::Greater => println!("Too big!"),
                Ordering::Equal => {
                    println!("You win!");
                    high_score = high_score + 1;
                    println!("Your score is {high_score}");
                    let mut play_again = String::new();
                    println!("Do you want to play again (y/n)");
                    io::stdin()
                        .read_line(&mut play_again)
                        .expect("Failed to read your choice");

                    let first_char = play_again.trim().chars().next().expect("Invalid input");
                    if first_char == 'y' {
                        println!("Resetting tries");
                        secret_number = rand::thread_rng().gen_range(1..=100);
                        println!("Secret number is {secret_number}");
                        no_tries = 3;
                        continue;
                    } else {
                        write_high_score(high_score);
                        println!("Thanks for playing");
                        break;
                    }
                }
            }
        }
        no_tries = no_tries - 1;
        if no_tries == 0 {
            println!("You have used up your guesses.");
            let mut play_again = String::new();
            println!("Do you want to play again (y/n)");
            io::stdin()
                .read_line(&mut play_again)
                .expect("Failed to read your choice");

            let first_char = play_again.trim().chars().next().expect("Invalid input");
            if first_char == 'y' {
                println!("Resetting tries");
                no_tries = 3;
                continue;
            } else {
                println!("Thanks for playing");
                break;
            }
        }
    }
}

fn main() {
    let mut user_selection = String::new();
    println!("Welcome to my guess the number game");
    println!(
        "What would you like to do? \n Enter \n 1. To play a new game \n 2. To view highscore"
    );
    io::stdin()
        .read_line(&mut user_selection)
        .expect("Error reading your input");
    let value = user_selection
        .trim()
        .parse::<i32>()
        .expect("Unexpected value please enter a valid number");
    if value == 1 {
        play();
    } else if value == 2 {
        let high_score = read_high_score();
        println!("Your highscore is: {high_score}");
    } else {
        println!("Please enter a valid option");
    }
}
