use std::io;

fn main() {
    println!("Welcome to my simple conversion application!");
    let mut input_base: String = String::new();

    println!("Please enter your system number:");
    io::stdin().read_line(&mut input_base).expect("Failed to read line");

    let mut number: String = String::new();
    println!("Enter your number in your system");
    io::stdin().read_line(&mut number).expect("Failed to read line");

    let mut output_base: String = String::new();
    println!("Enter the numeral system in which you want to obtain the result");
    io::stdin().read_line(&mut output_base).expect("Failed to read line");

    println!("Thank you for using my application!");
    println!("Press enter to quit the application");
}
