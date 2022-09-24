fn main() {
    println!("Hello, world!");

    let random_str = "Why, hello there!";
    another_function(true, random_str);

    let five = five();
    println!("the number five: {five}");
}

fn another_function(yes_or_no: bool, some_string: &str) {
    println!("Another function. {yes_or_no} {some_string}");
}

fn five() -> i32 {
    5
}
