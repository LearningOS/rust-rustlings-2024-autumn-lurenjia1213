// tests5.rs
//
// An `unsafe` in Rust serves as a contract.
//
// When `unsafe` is marked on an item declaration, such as a function,
// a trait or so on, it declares a contract alongside it. However,
// the content of the contract cannot be expressed only by a single keyword.
// Hence, its your responsibility to manually state it in the `# Safety`
// section of your documentation comment on the item.
//
// When `unsafe` is marked on a code block enclosed by curly braces,
// it declares an observance of some contract, such as the validity of some
// pointer parameter, the ownership of some memory address. However, like
// the text above, you still need to state how the contract is observed in
// the comment on the code block.
//
// NOTE: All the comments are for the readability and the maintainability of
// your code, while the Rust compiler hands its trust of soundness of your
// code to yourself! If you cannot prove the memory safety and soundness of
// your own code, take a step back and use safe code instead!
//
// Execute `rustlings hint tests5` or use the `hint` watch subcommand for a
// hint.

//

/// # Safety
///
/// The `address` must contain a mutable reference to a valid `u32` value.
unsafe fn modify_by_address(address: usize) {
    // TODO: Fill your safety notice of the code block below to match your
    // code's behavior and the contract of this function. You may use the
    // comment of the test below as your format reference.
    unsafe {
        let p =address as *mut u32;//设置指针类型为指向u32的指针（从usize）
        if !p.is_null(){*p=0xAABBCCDD;}          
        
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        let mut t: u32 = 0x12345678;
        // SAFETY: The address is guaranteed to be valid and contains
        // a unique reference to a `u32` local variable.
        unsafe { modify_by_address(&mut t as *mut u32 as usize) };/*

        &mut t：可变引用
         as *mut u32转换为可变指针，u32类型
        as usize:单纯的一个地址，貌似没影响？
         */
        assert!(t == 0xAABBCCDD);
    }
}
fn main() {
    let array = [10, 20, 30, 40, 50];
    
    // 获取数组的指针
    let ptr : *const [i32;5]= &array;//array.as_ptr();
    
    // 将指针转换为 usize
    let address: usize = ptr as usize;
    
    // 打印原始地址
    println!("Original address: {}", address);
    
    // 设置偏移量，假设我们要访问数组中的第三个元素
    let offset = 2 * std::mem::size_of::<i32>(); // 计算偏移量，2 是索引（从 0 开始）
    println!("size of i32 {}",std::mem::size_of::<i32>());
    // 计算新的地址
    let new_address = address + offset;
    
    // 打印新的地址
    println!("New address after offset: {}", new_address);
    
    // 将新的地址转换回原始指针
    let new_ptr = new_address as *const [i32;5];
    let new_ptr2 = new_address as *const i32;
    // 使用 unsafe 代码块来解引用新的指针
    unsafe {
        println!("Value at new address: {}", (*new_ptr)[3]);
        println!("Value at new address: {}", (*new_ptr)[2]);
    }
    unsafe {
        println!("Value at new address: {}", (*new_ptr2));
    }
}
