fn main() 
{
  let numbers = [10, 30, 23, 89, 12, 38];

  println!("For Loop");
  for number in numbers.iter() {
    if number % 2 == 0 {
      println!("{} is even", number);
    }
    else {
      println!("{} is odd", number);
    }
  }

  println!("Prime Numbers in the range: [1-100]");
  let mut number = 1;
  while number <= 100 {
    if is_prime(&number) {
      println!("{} is prime", number);
    }
    number += 1;
  }
}


fn is_prime(number : &u32) -> bool {
  let mut divider = 2;
  while divider <= (number/2) {
    if number % divider == 0 {
      return false;
    }
    divider += 1;
  }

  return true;
}
