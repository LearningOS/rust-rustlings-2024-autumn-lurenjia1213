// clippy2.rs
// 
// Execute `rustlings hint clippy2` or use the `hint` watch subcommand for a
// hint.

//
#[allow(for_loops_over_fallibles)]
fn main() {
    let mut res = 42;
    let option = Some(12);//`for` loops over Option values are more clearly expressed as an `if let`
    /*for x in option {
        res += x;
    }*/
    if let Some(x)=option{
        res+=x;
    }
    println!("{}", res);
}
