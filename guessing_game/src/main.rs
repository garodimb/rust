/*
 * Guessing Game
 */

use rand::Rng;
use std::cmp::Ordering;

fn main()
{
  println!("Guessing Game!");
  loop
  {
    let secret_number = rand::thread_rng().gen_range(1, 3);
    let mut guess = String::new();
      
    println!("Enter your guess:");
    
    std::io::stdin().read_line(&mut guess).expect("Unable to read input");
    let guess : u32 = match guess.trim().parse() {
      Ok(num) => num,
      Err(_) => {
        println!("Enter valid number!");
        continue;
      }
    };

    println!("You guessed {}", guess);
    println!("Secret number {}", secret_number);

    match guess.cmp(&secret_number)
    {
      Ordering::Less => println!("Too small!"),
      Ordering::Greater => println!("Too large!"),
      Ordering::Equal => {
         println!("You won!");
         break;
      }
    };
  }
}