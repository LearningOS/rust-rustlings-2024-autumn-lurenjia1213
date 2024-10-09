// macros1.rs
//
// Execute `rustlings hint macros1` or use the `hint` watch subcommand for a
// hint.

//

macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}
macro_rules! lmao{
    ()=>{
        println!("原神，启动")
    };
    ($a:expr)=>{
        println!("{}",$a);
    };
    ($a:expr,$b:expr,$c:expr)=>{
        println!("{}{}}}",$a,$b);
    }
}
fn main() {
    my_macro!();
    lmao!(1);
    lmao!();
    lmao!(1,2,3);
}
