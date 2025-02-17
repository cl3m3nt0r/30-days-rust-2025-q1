# My Rust Guessing Game Journey

## Introduction

This document captures my journey extending the basic guessing game from Chapter 2 of "The Rust Programming Language" book. While the original game taught me fundamental Rust concepts, I wanted to challenge myself by adding more features to make the game more engaging and to deepen my understanding of Rust.

## Starting Point: The Basic Game

The guessing game from the Rust book was my first real Rust program. It taught me:
- Basic input/output with `println!` and `read_line`
- Basic variable handling with `let` and `mut`
- Error handling with `match` and `expect`
- Use of external crates (`rand`)
- Comparison operations

## My Extension Goals

After completing the basic game, I set out to add several features:
1. Input validation to handle non-numeric inputs more gracefully
2. A persistent high score system using file I/O
3. A menu interface to start a game or view high scores
4. A "play again" feature

Each of these extensions presented its own learning opportunities and challenges.

## Challenge 1: Input Validation

### Learning Experience

The original game would crash if you entered non-numeric input. My first enhancement was implementing proper input validation.

```rust
let guess: u32 = match guess.trim().parse() {
    Ok(num) => num,
    Err(_) => {
        println!("Invalid input! Please enter a valid number.");
        continue;
    }
};
```

This taught me about:
- Rust's `Result` type for error handling
- Pattern matching with `match`
- Flow control with `continue`
- The `parse()` method and type annotations

### Reflections

The elegance of Rust's error handling became clear to me here. Instead of try/catch blocks or checking return codes, the `match` expression made handling different outcomes intuitive and forced me to consider the error case.

## Challenge 2: High Score System

### Learning Experience

Implementing a high score system required learning about file I/O in Rust:

```rust
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
```

This taught me about:
- The `fs` module
- Reading from and writing to files
- Error handling in I/O operations
- Converting between different data types

### Struggles and Solutions

Initially, I faced challenges when the high score file didn't exist. I solved this by:
1. Using `match` to handle the case where the file doesn't exist
2. Creating the file with a default value when needed
3. Learning about file permissions and paths

## Challenge 3: Menu Interface

### Learning Experience

Adding a menu system taught me about managing program flow:

```rust
fn main() {
    let mut user_selection = String::new();
    println!("Welcome to my guess the number game");
    println!(
        "What would you like to do? \n Enter \n 1. To play a new game \n 2. To view highscore"
    );
    io::stdin()
        .read_line(&mut user_selection)
        .expect("Error reading your input");
    // ...
}
```

This taught me about:
- Program organization
- User experience design in CLI programs
- Function separation and program structure

### Reflections

Creating a menu made me think about the overall user flow. I realized good programs need both good code and good user experience.

## Challenge 4: Play Again Feature

### Learning Experience

Implementing the "play again" feature required understanding loop control and state management:

```rust
if first_char == 'y' {
    println!("Resetting tries");
    secret_number = rand::thread_rng().gen_range(1..=100);
    no_tries = 3;
    continue;
} else {
    write_high_score(high_score);
    println!("Thanks for playing");
    break;
}
```

This taught me about:
- Character handling in Rust
- Loop control with `break` and `continue`
- State management across iterations

### Struggles and Solutions

A challenge I faced was ensuring the secret number reset properly. I learned that:
1. Variable scopes in loops require careful consideration
2. It's important to reset all relevant state variables
3. Consistent prompting improves user experience

## Lessons Learned

### Rust Concepts Mastered

Through this project, I gained confidence with:
- Ownership and borrowing
- Error handling patterns
- Type safety and conversions
- Program flow and organization
- File I/O operations

### Programming Best Practices

I also learned broader programming concepts:
- Separating concerns (game logic vs. file I/O)
- User input validation
- Graceful error handling
- User experience considerations

## Future Improvements

As I continue my Rust journey, I'd like to further enhance this game with:
1. Difficulty levels (varying number ranges or attempts)
2. A hint system that activates after certain number of wrong guesses
3. User profiles to track multiple players' high scores
4. A more sophisticated scoring system based on attempts used

## Conclusion

Extending the basic guessing game significantly deepened my understanding of Rust. Each feature I added required learning new concepts, and the process of incrementally improving the game was both challenging and rewarding. This project has given me the confidence to tackle more complex Rust applications and a solid foundation in Rust's core concepts.