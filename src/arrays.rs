use std::mem;

pub fn run() {
    let numbers: [i32; 4] = [1, 2, 3, 4];

    println!("{:?}", numbers);

    println!("Array occupies {} bytes", mem::size_of_val(&numbers));

    let slice: &[i32] = &numbers[0..2];

    println!("Slice: {:?}", slice);
}
