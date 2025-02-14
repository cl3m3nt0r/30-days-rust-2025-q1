use std::io;

enum Direction {
    TOFAHR,
    TOCELC,
}
fn convert(direction: Direction, value: f64) -> f64 {
    match direction {
        Direction::TOFAHR => value * 9.0 / 5.0 + 32.0,
        Direction::TOCELC => (value - 32.0) * 5.0 / 9.0,
    }
}
fn main() {
    println!("Hello, world!");
    convert(Direction::TOFAHR, 2.0);
    convert(Direction::TOCELC, 2.0);

    // Prompt the user to enter the conversion direction
    println!("Enter The conversion direction: \n (1) to convert from Celcius => Fahrenheit \n (2) to convert from Fahrenheit => Celcius");
    // This user variable returns the number of bytes read from the user but the twist is that it
    // then passes the actual value read to the &mut value
    let mut direction_input_value = String::new();
    io::stdin()
        .read_line(&mut direction_input_value)
        .expect("Failed to read input");
    // let direction_input = io::stdin().read_line(&mut direction_input_value);
    let direction = match direction_input_value.trim() {
        "1" => Direction::TOFAHR,
        "2" => Direction::TOCELC,
        _ => {
            println!("Invalid input. Please enter 1 or 2.");
            return;
        }
    };

    println!("Enter the value that you want to convert");
    let mut temperature_value = String::new();
    io::stdin()
        .read_line(&mut temperature_value)
        .expect("Failed to read input");
    let tempertature: f64 = match temperature_value.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Invalid temperature value. Please enter a number.");
            return;
        }
    };
    let conversion_result = convert(direction, tempertature);
    println!("Converted tempertature: {conversion_result}")
}
