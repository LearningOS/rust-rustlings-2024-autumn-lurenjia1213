#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(String), // Quarter 变体包含一个字符串
}

fn main() {
    //let coin = Coin::Quarter(String::from("California")); // 示例 coin
    let mut count = 0;
    let coin=Coin::Penny;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {state:?}!");
    } else {
        count += 1;
    }

    println!("Count of non-quarter coins: {}", count);
}
