use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    println!("welcome to my first rust mini project!");

    let secret_num: u32 = rand::thread_rng().gen_range(0..=420);
    //let mut is_win:bool = false;
    loop // loop allows to keep this code running within loop
    {
        println!("\n Enter your guess:");

        let mut guess: String = String::new();
        io::stdin().read_line(&mut guess).expect("No line read!");

        let guess: u32 = guess.trim().parse().expect("Please enter a number!");

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
