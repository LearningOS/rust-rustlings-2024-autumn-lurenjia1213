
fn is_even(num: i64) -> bool {
    num % 2 == 0
}

// TODO: Fix the function signature.
fn sale_price(price: i64) -> i64{//使用fn func_name()->type的形式声明返回类型
    if is_even(price) {
        price - 10
    } else {
        price - 3
    }
}
fn ret_str()-> &'static str{//声明周期。
    return "test";
}

fn main() {
    let original_price = 51;
    println!("Your sale price is {}", sale_price(original_price));//这不需要操心类型啊
    println!("ret strings {}",ret_str());
}
