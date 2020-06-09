use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");


    let answer = rand::thread_rng().gen_range(0, 101);
    loop {

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("didn't read line");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };


        match guess.cmp(&answer) {
           Ordering::Less => println!("too small!"),
           Ordering::Greater => println!("too big!"),
           Ordering::Equal => {
               println!("You win!");
               break;
            }
        }
    }
}


