fn main() {
    let mut s1 = String::from("hello");

    let len = calculate_length(&s1);
    let mut x=calculate_length(&s1);

    //很合理，上面是不可变引用，不至于数据竞争

    println!("The length of '{s1}' is {len}.");
    modify_wtf(&mut s1);
    println!("{s1}");

    let ref1=&mut s1;
    let ref2=&mut s1;
    //println!("{}  {}",ref1,ref2);//可能存在竞争，无法通过编译
    //println!("{}",ref1);//https://kaisery.github.io/trpl-zh-cn/ch04-02-references-and-borrowing.html
    //println!("{}",ref2);
    let ref3=&s1;
    let ref4=&s1;
    println!("3 {}",ref3);println!("4 {}",ref4);//不可变，没问题

    //垂悬引用
    let reference_to_nothing = dangle();
}
fn modify_wtf(s:&mut String){
    s.push_str(",rust");
}
fn calculate_length(s: & String) -> usize {
    s.len()
}
fn dangle() -> &String {
    let s = String::from("hello");

    &s//引用了s，但是s会在最后被释放
}