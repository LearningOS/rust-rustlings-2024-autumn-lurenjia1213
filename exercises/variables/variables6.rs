// variables6.rs
//
// Execute `rustlings hint variables6` or use the `hint` watch subcommand for a
// hint.


const NUMBER: i32 = 3;//不会自动推导类型，这玩意，有点像#define？，可以编译时计算~
fn main() {
    println!("Number {}", NUMBER);
}
