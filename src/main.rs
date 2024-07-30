use std::io;
use rand::Rng;
fn main() 
{
    println!("welcome to my first rust mini project!");

    let secret_num:u16 = rand::thread_rng().gen_range(0..=420);
    //let mut is_win:bool = false;
    println!("\n Enter your guess:");

    let mut guess:String = String::new();
    io::stdin().read_line(&mut guess).expect("no line read");
    // if(integer == secret_num){
    //     println!("Do something");
    // }
    println!("You guessed {guess}");
}
