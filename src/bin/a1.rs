// Topic: Functions
//
// Program requirements:
// * Displays your first and last name
//
// Notes:
// * Use a function to display your first name
// * Use a function to display your last name
// * Use the println macro to display messages to the terminal

fn first_name(n: String) {
    println!("First name: {}", n);
}

fn last_name(n: String) {
    println!("Last name: {}", n);
}

fn main() {
    first_name("Diego".to_string());
    last_name("Parra".to_string());
}
