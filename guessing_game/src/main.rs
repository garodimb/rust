fn main()
{
  println!("Guessing Game!");
  let mut guess = String::new();
  println!("Enter your guess:");
  std::io::stdin().read_line(&mut guess).expect("Unable to read input");
  println!("You guessed {}", guess);
}