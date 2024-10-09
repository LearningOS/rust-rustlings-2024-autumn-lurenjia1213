/*
enum Option<T> {
    None,
    Some(T),
}//

*/


fn main() {
    let some_value = Some(5); // 创建一个包含值 5 的 Some
    let no_value: Option<i32> = None; // 创建一个没有值的 Option

    match some_value {
        Some(v) => println!("The value is: {}", v),
        None => println!("No value"),
    }

    match no_value {
        Some(v) => println!("The value is: {}", v),
        None => println!("No value"),
    }
}
