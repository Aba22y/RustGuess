use std::io;
use rand::Rng;
fn main() {
    println!("Feeling Lucky?");
    println!("What do you think the random number is?");
    
    let secret_num = rand::thread_rng().gen_range(1..=100);

    //empty instance
    let mut guess = String::new();

    // & is a refrence to the actual variable so we avoid copying it
    // anything that returns an enumerated result "Ok"/"Err" can use expect
    io::stdin().read_line(&mut guess).expect("Failed to read");
    
    println!("You guessed {}", guess);
    println!("{guess} is quite the number")
}
