// strings1.rs
//
// Make me compile without changing the function signature!
//
// Execute `rustlings hint strings1` or use the `hint` watch subcommand for a
// hint.
fn cacl_len(s:&str)->usize{
    s.len()
}//我去，类型很复杂啊
/*
fn main() {
    // 创建一个 String
    let my_string = String::from("Hello, Rust!");//从rodata的字符串切片，转为string

    // 方法 1: 使用 as_str() 方法
    let my_str1: &str = my_string.as_str();
    println!("{}", my_str1);

    // 方法 2: 直接使用引用
    let my_str2: &str = &my_string;//引用(切片)
    println!("{}", my_str2);
}




*/
fn main() {
    let answer = current_favorite_color()+" and red";
    let len=cacl_len(&answer) as i32;//无聊来个类型转换
    println!("My current favorite color is {},wtf len is {}", answer,len);

    println!("{}",&answer[0..answer.len()-1]);
}

fn current_favorite_color() -> String {
    //return "blue".to_string();
    return String::from("blue");
}
/*
"blue" 默认是一个字符串切片，所以这里转换一下
或者使用String::from,前面有用到过
*/
