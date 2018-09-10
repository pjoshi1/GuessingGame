
extern crate rand;

//Use Input output

use std::io;

use std::cmp::Ordering;

use rand::Rng;

fn main() {




    let random_number: u32 = rand::thread_rng().gen_range(1,101);

    loop{
        println!("Please enter the number you guessed");

        let mut guessed_number = String::new();

        io::stdin().read_line(& mut guessed_number)
            .expect("Failed to read a number");

        let guessed_number: u32 = match guessed_number.trim().parse() {

            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guessed_number);

        match guessed_number.cmp(&random_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal =>{
                println!("You win!");
                break;
            }
        }
    }

}
