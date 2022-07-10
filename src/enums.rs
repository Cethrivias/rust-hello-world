use std::default;


enum Movement {
  Up,
  Down,
  Left,
  Right
}

fn move_char(m: Movement) {
  match m {
      Movement::Up => println!("Moving Up"),
      Movement::Down => println!("Moving Down"),
      Movement::Left => println!("Moving Left"),
      Movement::Right => println!("Moving Right"),
  }
}

pub fn run() {
  let ch = Movement::Up;

  move_char(ch);
}