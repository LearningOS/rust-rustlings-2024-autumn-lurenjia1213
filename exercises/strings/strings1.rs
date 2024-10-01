// strings1.rs
//
// Make me compile without changing the function signature!
//
// Execute `rustlings hint strings1` or use the `hint` watch subcommand for a
// hint.

fn main() {
    let answer = current_favorite_color();
    println!("My current favorite color is {}", answer);
}

fn current_favorite_color() -> String {
    //return "blue".to_string();
    return String::from("blue");
}
/*
"blue" 默认是一个字符串切片，所以这里转换一下
或者使用String::from,前面有用到过
*/
