/*
 * Guessing Game
 */

use rand::Rng;
use std::cmp::Ordering;

fn main()
{
  let secret_number = rand::thread_rng().gen_range(1, 11);

  println!("Guessing Game!");
  let mut guess = String::new();
  
  println!("Enter your guess:");
  
  std::io::stdin().read_line(&mut guess).expect("Unable to read input");
  println!("You guessed {}", guess);

  println!("Secret Number: {}", secret_number);

  let guess : u32 = guess.trim().parse().expect("Unable to parse number!");
  match guess.cmp(&secret_number)
  {
    Ordering::Less => println!("Too small!"),
    Ordering::Greater => println!("Too large!"),
    Ordering::Equal => println!("You won!"),
  }
}