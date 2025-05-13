use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    println!("Feeling Lucky?");
    
    let secret_num = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("What do you think the random number is?");
        //empty instance
        let mut guess = String::new();
        // & is a refrence to the actual variable so we avoid copying it
        // anything that returns an enumerated result "Ok"/"Err" can use expect
        io::stdin().read_line(&mut guess).expect("Failed to read");
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed {guess}");

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too large!"),
            Ordering::Equal => {
                println!("Winner!");
                break;
            }
        }
    }

}
