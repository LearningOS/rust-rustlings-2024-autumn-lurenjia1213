// options1.rs
//
// Execute `rustlings hint options1` or use the `hint` watch subcommand for a
// hint.
//https://rustwiki.org/zh-CN/rust-by-example/std/option.html
//

// This function returns how much icecream there is left in the fridge.
// If it's before 10PM, there's 5 pieces left. At 10PM, someone eats them
// all, so there'll be no more left :(
fn maybe_icecream(time_of_day: u16) -> Option<u16> {
    // We use the 24-hour system here, so 10PM is a value of 22 and 12AM is a
    // value of 0 The Option output should gracefully handle cases where
    // time_of_day > 23.
    // TODO: Complete the function body - remember to return an Option!
    match time_of_day{
        wtf if wtf>24=>None,
        wtf if wtf>=22=>Some(0),
        _ =>Some(5) 
    }/*
    wtf 是一个变量名，用于在 match 表达式中捕获 time_of_day 的值。
    
    看看enums.
     */

}
/*
enum Option<T> {
    None,
    Some(T),
}

*/
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_icecream() {
        assert_eq!(maybe_icecream(9), Some(5));
        assert_eq!(maybe_icecream(10), Some(5));
        assert_eq!(maybe_icecream(23), Some(0));
        assert_eq!(maybe_icecream(22), Some(0));
        assert_eq!(maybe_icecream(25), None);
    }

    #[test]
    fn raw_value() {
        // TODO: Fix this test. How do you get at the value contained in the
        // Option?
        let icecreams = maybe_icecream(12);
        assert_eq!(icecreams, Some(5));/*
        在 Rust 中，Some 是 Option 枚举的一部分，用于表示一个值的存在。Option 枚举有两个变体：
        Some(T)：表示一个包含值 T 的选项。
        None：表示没有值。 
        */
        
    }
}
