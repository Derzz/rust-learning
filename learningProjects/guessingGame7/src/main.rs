// Guessing game from a number between 1 to 100
use rand::Rng;

fn main() {
    println!("==== GUESSING GAME ====");
    let mut generator = rand::thread_rng();
    let random_number = generator.gen_range(1..101);
    println!("A number has been chosen. Guess a number between 1 and 100.");
    let mut guess: u8 = 0;

    while guess != random_number{
        let mut guess_str = String::new();
        std::io::stdin().read_line(&mut guess_str).expect("Couldn't not read number!");
        guess = guess_str.trim().parse().expect("Error converting number!");
        if guess == random_number{
            println!("You found it!");
        }

        else if guess < random_number{
            println!("Too low!");
        }
        else{
            println!("Too high!");
        }
    }

}
