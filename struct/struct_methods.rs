struct Rectangle
{
  height: u32,
  width: u32
}

impl Rectangle {
  fn area(&self) -> u32
  {
    self.width * self.height
  }

  fn perimeter(&self) -> u32
  {
    2 * (self.width + self.height)
  }
}


fn main()
{
  let rectangle = Rectangle{ height: 10, width: 20};
  println!("Area: {}", rectangle.area());
  println!("Perimeter: {}", rectangle.perimeter());
}