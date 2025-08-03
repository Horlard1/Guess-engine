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
    // let x = (let y = 6); uncomiled line of code
    let y = {
        let x = 3;
        x + 1
    }; // note that this statement takes the semicolon out of scope to return the last value implicitly
    println!("The value of y is: {y}");
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
    let number = if condition { 5} else {6}; // note both condition blocks returns i32
    println!("The value of number is {number}");


    // loop
    // care for an infinite loop, run the next line of code

    // loop {
    //     println!("another one!");
    // }

    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2; // returning a value after breaking the loop
        }
    };  // enabling the result to be assigned with the value implicitly

    // also loop can be labelled

    let mut count = 0;
    'counting_up: loop { // labelled the mother loop
        println!("count = {count}");
        let mut remaining = 10;
        loop {
              println!("remaining = {remaining}");
              if remaining == 9 {
                break;
              }
              if count == 2 {
                break 'counting_up; // breaking the labelled loop to exit all
              }
              remaining -= 1;
        }
        count += 1;

    }
    println!("End count = {count}");


    // while  can also be used for loops
    let mut num = 3;
    while num != 0 {
        println!("{num}!");

        num -= 1;
    }
    println!("Let's go!!!");


    // to use for..in on the above
    for numba in (1..4).rev() { // use .rev to reverse a range
           println!("{numba}!");
    }
     println!("Let's go!!!");


    // to loop through an array collection, use for..in loop
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is {element}");
    }
}


