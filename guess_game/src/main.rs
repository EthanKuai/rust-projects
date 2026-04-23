use rand::Rng;
use std::io;

fn hint(ans: i32, guess: i32) -> String {
    if guess < ans {
        "Too small".to_string()
    } else if guess > ans {
        "Too big".to_string()
    } else {
        String::new()
    }
}

fn main() {
    println!("Welcome to number guessing game!");

    let ans: i32 = rand::random_range(1..=100);
    // rand::thread_rng().gen_range(1..=100);

    let mut input: String = String::new();
    let mut guess: i32 = 0;
    let mut attempts: i32 = 0;

    while guess != ans {
        attempts += 1;

        input.clear();
        while input.trim().parse::<i32>().is_err() {
            println!("Guess a number [1-100]:");
            input.clear();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
        }
        guess = input.trim().parse::<i32>().unwrap();
        // println!("Your guess is {}", guess);
        println!("{}", hint(ans, guess));
    }

    println!("You got it!");
    println!("The answer is {}", ans);
    println!("You took {attempts} attempts");
}
