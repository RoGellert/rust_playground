enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("You got a lucky one");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {
    let coin1 = Coin::Penny;
    let coin2 = Coin::Nickel;
    let coin3 = Coin::Dime;
    let coin4 = Coin::Quarter;

    let value1 = value_in_cents(coin1);
    let value2 = value_in_cents(coin2);
    let value3 = value_in_cents(coin3);
    let value4 = value_in_cents(coin4);

    println!("Value of coin is {}", value1);
    println!("Value of coin is {}", value2);
    println!("Value of coin is {}", value3);
    println!("Value of coin is {}", value4);
}