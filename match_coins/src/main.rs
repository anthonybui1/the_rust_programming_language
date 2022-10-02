#[derive(Debug)]
enum UsState {
    Texas,
    // California,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

impl Coin {
    fn value(&self) -> u8 {
        match self {
            Coin::Penny => {
                println!("Lucky penny!");
                1
            },
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quarter from {:?}!", state);
                25
            },
        }
    }
}

fn main() {
    println!("Hello, world!");
    let penny = Coin::Penny;
    let nickel = Coin::Nickel;
    let dime = Coin::Dime;
    let quarter = Coin::Quarter(UsState::Texas);
    println!("{}", penny.value());
    println!("{}", nickel.value());
    println!("{}", dime.value());
    println!("{}", quarter.value());
}
