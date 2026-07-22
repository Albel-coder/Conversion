use std::io;

fn converter(input_base: usize, output_base: usize, number: String) -> String {
    let digits: String = String::from("0123456789abcdef");
    let mut decimal: usize = 0;
    let number_bytes: std::str::Chars<'_> = number.trim().chars();

    // convert to decimal number
    for digit in number_bytes {
        let digit: Option<usize> = digits.find(digit);
        decimal = decimal * input_base + digit.unwrap_or(0);
    }

    // convert to user base system
    if (decimal == 0) {
        return "0".to_string();
    } else {
        let mut result: String = String::new();
        while (decimal > 0) {
            result.push(
                digits
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
    loop {
        println!("Please enter your system number (a number from 2 to 16):");
        input_base.clear();
        while io::stdin().read_line(&mut input_base).is_err() {
            println!("Error reading your number, please try again!");
        }

        let input_base: usize = match input_base.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Your input is not valid! Please enter the number!");
                continue;
            },
        };

        let mut number: String = String::new();
        println!("Enter number in your system:");
        while io::stdin().read_line(&mut number).is_err() {
            println!("Error reading your number, please try again!");
            number.clear();
        }

        let mut output_base: String = String::new();
        println!("Enter the number system in which you want to obtain the result");
        loop {
            output_base.clear();
            while io::stdin().read_line(&mut output_base).is_err() {
                println!("Error reading your number, please try again!");
            }

            let output_base: usize = match output_base.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Your input is not valid. Please enter the number!");
                    continue;
                }
            };

            let result: String = converter(input_base, output_base, number);
            println!("Conversion result: {result}");
            break;
        }

        println!("Thank you for using my application!");
        println!("Enter 'continue' to continue converting new numbers or enter anything to quit the application");
        let mut input: String = String::new();
        while io::stdin().read_line(&mut input).is_err() {
            println!("Error reading your number, please try again!");
            input.clear();
        }
        if (input.to_lowercase() == "continue") {
            continue;
        }
        else {
            println!("Thank you for using my application!");
            println!("Press enter to quit the application");
            let mut quit: String = String::new();
            io::stdin().read_line(&mut quit).expect("");
            return;
        }
    }
}
