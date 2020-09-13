fn main()
{
  struct User
  {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
  };

  let user1 = User {
    email: String::from("abc@xyz.com"),
    username: String::from("abc"),
    sign_in_count: 1,
    active: true,
  };

  println!("Email Address: {}", user1.email);
}