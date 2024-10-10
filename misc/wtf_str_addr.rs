fn main() {
    let string1 = String::from("green");
    let string2 = String::from("green");
    
    // 内容比较
    if string1 == string2 {
        println!("内容相同");
    } else {
        println!("内容不同");
    }

    // 地址比较
    if std::ptr::eq(&string1, &string2) {
        println!("地址相同");
    } else {
        println!("地址不同");
    }
}
