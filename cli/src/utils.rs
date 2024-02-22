use std::io;

pub fn get_num_input(message: Option<&str>) -> i32 {
    println!("{}", message.unwrap_or("Enter a number:"));

    let mut input = String::new();

    if let Err(_) = io::stdin().read_line(&mut input) {
        return get_num_input(Some("Failed to read line! Try again:"));
    }

    input
        .trim()
        .parse()
        .unwrap_or_else(|_| get_num_input(Some("Guess must be a number! Try again:")))
}
