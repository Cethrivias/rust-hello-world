// struct Color {
//     red: u8,
//     green: u8,
//     blue: u8,
// }

// struct ColorT(u8, u8, u8);

struct User {
    first_name: String,
    last_name: String,
}

impl User {
    fn new(first: &str, last: &str) -> User {
        User {
            first_name: first.to_string(),
            last_name: last.to_string(),
        }
    }

    fn get_full_name(&self) -> String {
      format!("{} {}", self.first_name, self.last_name)
    }
}

pub fn run() {
    // let mut red = Color {
    //   red: 255,
    //   green: 0,
    //   blue: 0
    // // };

    // red.red = 200;

    // println!("Color: {} {} {}", red.red, red.green, red.blue);

    // let red = ColorT(255, 0, 0);

    // println!("Color: {} {} {}", red.0, red.1, red.2);
    let user = User::new("John", "Doe");

    println!("User: {} {}", user.first_name, user.last_name);

    println!("Full name: {}", user.get_full_name());
}
