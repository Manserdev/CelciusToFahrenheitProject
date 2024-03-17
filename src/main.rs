use std::io;

fn main() {


    loop  {
        println!("Please enter celsius to calculate Fahrenheit: ");
        println!("Or type 'exit' to end.");

        let mut celcius = String::new();

        //takes user input
        io::stdin()
            .read_line(&mut celcius)
            .expect("incorrect try again");

        if celcius.trim() == "exit" {
            break; // exits the loop if the user types 'exit'
        }

        //parses the string into a float
        let celcius: f32 = celcius
            .trim()
            .parse()
            .expect("That is not a number!");

        let converted_value = (celcius * 1.8) + 32.0;

        println!("{celcius} converted to Fahrenheit is: {converted_value}");

    }
}


