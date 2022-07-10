use std::mem;

pub fn run() {
    let mut numbers = vec![1, 2, 3, 4];

    numbers.push(5);
    numbers.push(6);

    println!("{:?}", numbers);

    println!("Array occupies {} bytes", mem::size_of_val(&numbers));

    let slice: &[i32] = &numbers[0..2];

    println!("Slice: {:?}", slice);

    for it in numbers.iter() {
      println!("Number: {}", it);
    }

    for it in numbers.iter_mut() {
      *it += *it;
    }

    println!("Numbers vec: {:?}", numbers);
}
