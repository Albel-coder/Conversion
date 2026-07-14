use std::io;

fn converter(input_base: usize, output_base: usize, number: String) -> String {
    let DIGITS: String = String::from("0123456789abcdef");
    let mut decimal: usize = 0;
    let number_bytes: std::str::Chars<'_> = number.trim().chars();

    // convert to decimal number
    for digit in number_bytes {
        let digit: Option<usize> = DIGITS.find(digit);
        decimal = decimal * input_base + digit.unwrap_or(0);
    }

    // convert to user base system
    if (decimal == 0) {
        return "0".to_string();
    } else {
        let mut result: String = String::new();
        while (decimal > 0) {
            result.push(
                DIGITS
                    .chars()
                    .nth(decimal % output_base)
                    .unwrap_or_default(),
            );
            decimal /= output_base;
        }
        let result: String = result.chars().rev().collect();
        return result;
    }
}

fn main() {
    println!("Welcome to my simple conversion application!");

    let mut input_base: String = String::new();
    println!("Please enter your system number:");
    io::stdin()
        .read_line(&mut input_base)
        .expect("Failed to read line");
    let input_base: usize = input_base.trim().parse().expect("Please enter a number");

    let mut number: String = String::new();
    println!("Enter number in your system");
    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line");

    let mut output_base: String = String::new();
    println!("Enter the numeral system in which you want to obtain the result");
    io::stdin()
        .read_line(&mut output_base)
        .expect("Failed to read line");
    let output_base: usize = output_base.trim().parse().expect("Please enter a number");

    let result: String = converter(input_base, output_base, number);
    println!("Conversion result: {result}");

    println!("Thank you for using my application!");
    println!("Press enter to quit the application");
    let mut quit: String = String::new();
    io::stdin().read_line(&mut quit).expect("");
}
