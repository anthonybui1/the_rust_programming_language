fn main() {
    let mut x = 5;
    println!("THe value of x is {x}");
    x = 6;
    println!("THe value of x is {x}");

    // Constants
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Three hours in seconds {THREE_HOURS_IN_SECONDS}");

    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}"); // 12
    } // Scope of shadowed x ends
    println!("The value of x is: {x}"); // 6

    // Shadowing a variable lets you change a variables type, while mutating a variable does not.
    let spaces = "   ";
    let spaces = spaces.len();
    println!("There are {spaces} spaces");

    // Floats
    let xfloat = 2.0; // f64
    let yfloat: f32 = 3.0; // f32
    println!("floats {xfloat}, {yfloat}");

    // Numeric Operations
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Results in 0
    let remainder = 43 % 5;
    println!("{sum} {difference} {product} {quotient} {floored} {remainder}");

    // The character type is enclosed with single quotes, as opposed to string literals
    let c = 'z';
    let z: char = 'ø';
    let heart_eyed_cat = '😻';
    println!("{c} {z} {heart_eyed_cat}");

    // Compound types group multiple values into one type
    // A tuple allows multiple types, while every element of an array must have the same type
    let tup: (i32, f64, u8) = (500, 5.0, 1);
    let (tup1, tup2, tup3) = tup;
    println!("{tup1}, {tup2}, {tup3}");
}
