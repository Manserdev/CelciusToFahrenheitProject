use std::io;

fn main() {
    let mut celcius = String::new();

    while celcius != "exit"  {
        println!("Please enter celsius to calculate Fahrenheit: ");
        println!("Or type 'exit' to end.");


        io::stdin()
            .read_line(&mut celcius)
            .expect("incorrect try again");

        let celcius: f32 = celcius
            .trim()
            .parse()
            .expect("That is not a number!");

        let converted_value = (celcius * 1.8) + 32.0;

        println!("{celcius} converted to Fahrenheit is: {converted_value}");


    }
}


