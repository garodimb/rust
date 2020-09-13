struct User
{
  username: String,
  email: String,
  sign_in_count: u64,
  active: bool
}

fn main()
{
  let user1 = User{
    username: String::from("Gopal"),
    email: String::from("gopal@xyz.com"),
    active: true,
    sign_in_count: 1
  };

  // Constructing struct using partial members of another instance
  let user2 = User{
    username: String::from("Amol"),
    email: String::from("amol@gmail.com"),
    ..user1
  };

  display_user(&user1); // Copy Semantics
  println!("------------------------");
  display_user(&user2);

  // construct struct using function paramater having same name as struct
  let user3 = build_user(String::from("Ajay"), String::from("ajay@gmail.com"));
  println!("------------------------");
  display_user(&user3);
}


fn display_user(user: &User)
{
  println!("Username: {}", user.username);
  println!("Email: {}", user.email);
  println!("Sign-In Count: {}", user.sign_in_count);
  println!("Active: {}", user.active);
}

// construct struct using function paramater having same name as struct
fn build_user(username: String, email: String) -> User
{
  User{
    username,
    email,
    sign_in_count: 1,
    active: false
  }
}