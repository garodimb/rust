fn main()
{
  let months = ["January", "February", "March", "April", "May", "June",
                "July", "August", "September", "Octomber", "November", "December"];
  let mut month_number = String::new();
  println!("Enter month number: ");
  
  std::io::stdin().read_line(&mut month_number).expect("Unable to read input!");
  let month_number : usize = month_number.trim().parse().expect("Invalid month number!");
  
  println!("Month number {} is {}", month_number, months[month_number-1]);
}