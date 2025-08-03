use std::cmp::Ordering;

use std::io;

use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Please input your guess.");
    loop {
        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };
           
        println!("You guessed: {guess}" );
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

fn new_statemet() { // uncalled function
    let _y = 6;
    // let x = (let y = 6); uncomiled code
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {y}")
}

fn control_flow() {
    // a condition must evaluate to bool
    // case in action, this next few lines

    // let number = 3;
    // if number {
    //     println!("Number was three")
    // }  // code will not compile


    // using if for variable declaration
    let condition = true;
    let number = if condition { 5} else {6} ; // note both condition blocks returns i32
    println!("The value of number is {number}")
}


