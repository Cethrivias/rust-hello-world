pub fn run() {
    let mut count = 0;
    // loop {
    //     count += 1;
    //     println!("Number: {}", count);

    //     if count == 20 {
    //         break;
    //     }
    // }

    // while count <= 100 {
    //     if count % 15 == 0 {
    //         println!("fizzbuzz");
    //     } else if count % 3 == 0 {
    //         println!("fizz")
    //     } else if count % 5 == 0 {
    //         println!("buzz");
    //     } else {
    //         println!("{}", count)
    //     }
    //     count += 1;
    // }

    for it in 0..100 {
        if it % 15 == 0 {
            println!("fizzbuzz");
        } else if it % 3 == 0 {
            println!("fizz")
        } else if it % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", it)
        }
    }
}
