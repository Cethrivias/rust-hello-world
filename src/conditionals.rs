pub fn run() {
  let age = 18;
  let check_id = false;
  
  if age >= 21 && check_id {
    println!("Oke")
  } else if age < 21 && check_id {
    println!("Not oke")
  } else {
    println!("Very not oke")
  }

  let is_of_age = if age >= 21 { true } else { false };

  println!("Is of age: {}", is_of_age);
}