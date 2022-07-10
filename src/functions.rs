pub fn run() {
    greet("Hello", "World");

    let sum = add(5, 8);

    println!("Sum: {}", sum);

    let add_closure = |a: i32, b: i32| a + b + sum;

    println!("Closure Sum: {}", add_closure(2, 3))
}

fn greet(greeting: &str, name: &str) {
    println!("{} {}", greeting, name);
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}
