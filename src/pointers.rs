pub fn run() {
    let arr1 = [1, 2, 3];
    let arr2 = arr1;

    println!("Arr1: {:?}", arr1);
    println!("Arr2: {:?}", arr2);

    let vec1 = vec![1, 2, 3];
    let vec2 = &vec1;

    println!("Vec1: {:?}", vec1);
    println!("Vec2: {:?}", vec2);
}
