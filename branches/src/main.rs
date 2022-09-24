fn main() {
    let number = 3;

    // Conditions must return a bool
    if number < 5 { // This is an arm
        println!("The number was less than 5: {number}");
    } else { // This is another arm
        println!("The number was equal to or greater than 5: {number}");
    }

    let string = "Hello, world!";
    if string.len() < 5 {
        println!("string shorter than or equal to 5");
    } else if string.len() < 10 {
        println!("string shorter than or equal to 10");
    } else {
        println!("string longer than 10");
    }

    // let mut count = 0;
    // 'counting_up: loop {
    //     println!("count = {count}");
    //     let mut remaining = 10;

    //     loop {
    //         println!("remaining = {remaining}");
    //         if remaining == 2 {
    //             break 'counting_up;
    //         }

    //         remaining -= 1;

    //         if remaining == 9 {
    //             break;
    //         }

    //     }

    //     count += 1;
    // }

    let mut counter = 10;
    while counter >= 0 {
        println!("counter: {counter}");
        counter -= 1;
    }

    let arr = [10, 20, 30, 40, 50];
    for element in arr {
        println!("element: {element}");
    }

    // This is a Range. It would print 1 2 3 but in rev it prints 3 2 1 (last index not included)
    for number in (1..4).rev() {
        println!("{number}");
    }

    println!("LIFTOFF!");
}
