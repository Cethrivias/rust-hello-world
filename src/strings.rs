pub fn run() {
  let mut hello = String::from("Hello");

  println!("Length: {}", hello.len());

  hello.push(',');
  hello.push(' ');
  hello.push_str("World!");

  println!("{}", hello);

  println!("Is empty: {}", hello.is_empty())
}