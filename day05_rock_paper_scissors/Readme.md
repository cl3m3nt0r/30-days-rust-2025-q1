# Rock-Paper-Scissors Game Documentation for Learners

## Introduction
In this project, I built a simple Rock-Paper-Scissors game in Rust. Along the way, I learned several key Rust concepts, especially the use of the `unreachable!()` macro and nested `match` statements. Below is an explanation of how these concepts helped me structure and handle the game’s logic.

## Key Concepts Learned

### 1. **The `unreachable!()` Macro**
One of the features I used in this game is the `unreachable!()` macro. This macro is used to indicate code that, under normal circumstances, should never be reached. It’s particularly useful for error handling and ensuring the game logic is working as expected.

- **Why it’s helpful**: In my game, there are certain situations that should be impossible—like if the choices somehow don’t match up correctly. By using `unreachable!()`, I can catch any unexpected behavior during development. If such an error happens, the game will immediately panic and display a clear error message.

- **What I learned**: The `unreachable!()` macro is a great way to enforce logic and ensure that the program behaves as intended. It helps me spot bugs or unexpected cases early on.

### 2. **Nested `match` Statements**
Another important concept I learned is how to use nested `match` statements in Rust. This is a powerful tool for handling multiple possible outcomes based on different conditions. In the game, I used `match` statements to compare both the player’s choice and the computer’s choice.

- **Why it’s helpful**: A `match` statement in Rust allows me to write clean, readable code that handles multiple cases (like Rock vs. Paper, Scissors vs. Rock, etc.). But when I had to compare the player’s and computer’s choices together, I learned that I could nest `match` statements. This let me easily cover all the different possibilities for who wins, who loses, or if it’s a tie.

- **What I learned**: Nested `match` statements help break down complex logic into smaller, manageable parts. By matching on multiple values at once (the player’s choice and the computer’s choice), I can handle many combinations more efficiently.

## How It All Comes Together
1. **Choice Handling**: The player makes a choice—Rock, Paper, or Scissors.
2. **Computer’s Turn**: The computer picks its choice randomly.
3. **Game Outcome**: Using nested `match` statements, the game compares the player's and computer's choices and determines the result:
   - If the choices are the same, it’s a tie.
   - If one choice beats the other (e.g., Rock beats Scissors), the winner is declared.
4. **Error Prevention**: If an invalid state or unexpected situation arises, the `unreachable!()` macro stops the game and alerts me that something went wrong.

## Conclusion
Building this game helped me understand how to use Rust’s error handling and control flow features like `unreachable!()` and `match` statements. These concepts make the code more robust and efficient, allowing me to handle various game outcomes easily while catching errors early. By the end of this project, I feel more confident in using Rust for logic-heavy applications and error handling!
