use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    println!("welcome to my first rust mini project!");

    let secret_num: u32 = rand::thread_rng().gen_range(0..=420);

    loop // loop allows to keep this code running within loop
    {
        println!("\n Enter your guess:");

        let mut guess: String = String::new();
        let mut some_num:u8;
        
        //in this below implementation, expect panics and quits the program, to avoid this, imma use match

        // io::stdin().read_line(&mut guess).expect("No line read!");

        // let guess: u32 = guess.trim().parse().expect("Please enter a number!");
        println!("this num wont do anything just taking input: ");

        match io::stdin().read_line(some_num).trim().parse() // this doesnt work in rust. we cant directly take an it input 
        {
            Ok(_) => (),
            Err(_) => 
            {
                print!("Invalid number size, try again");
                continue;
            }    
        }

        match io::stdin().read_line(&mut guess) // we have to type in &mut because of ownership
        {
            Ok(_) => (),
            Err(_) => 
            {
                println!("Failed to read line. Please try again:");
                continue;
            }   
        }

        let guess: u32 = match guess.trim().parse()
        {
            Ok(num) => num,
            Err(_) =>
            {
                println!("The entry was not valid, please enter a valid number:");
                continue;
            }    
        };

        //if else implement

        // if guess == secret_num //.into converts the type from u16 to u32
        // {
        //     println!("you win!");
        // }
        // else
        // {
        //     print!("you lose! :( ");
        // }

        match guess.cmp(&secret_num) {
            Ordering::Equal => {
                print!("you win! ");
                println!("You guessed {guess}, the number was {secret_num}");
                break;
            }
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
        }
    }

    //println!("You guessed {guess}, the number was {secret_num}");
}
