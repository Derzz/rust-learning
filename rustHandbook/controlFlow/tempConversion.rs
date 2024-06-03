// Convert temperatures based on user request
use std::io;

fn main() {
    let mut org;

    println!("Input C to convert from Celsius to Fahrenheit, F to convert from Fahrenheit to Celsius");
    loop {
        org = String::new();

        io::stdin().read_line(&mut org).expect("Failed to read line");
        org = org.trim().to_string();
        if org != "C" && org != "F" {
            println!("Incorrect command inputted, please try again.");
        } else { break; }
    }

    println!("input your temperature!");
    let mut temp = String::new();
    io::stdin().read_line(&mut temp).expect("Failed to read the temperature!");
    let mut new_temp: f32 = temp.trim().parse().expect("Please type a number!");

    if org == "C" {
        new_temp = new_temp * (9.0 / 5.0) + 32.0;
    } else {
        new_temp = (new_temp - 32.0) * (5.0 / 9.0);
    }

    println!("Your new temperature is {} degrees {}", new_temp, if org == "C" { "Fahrenheit" } else { "Celsius" });
}