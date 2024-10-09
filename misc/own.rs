
fn take(wtf:String)->String{
    println!("{wtf}");
    wtf
}
fn take2(wtf:i32){
    println!("{wtf}");
}

fn main(){
    let mut x="1".to_string();//这种在堆上，，，
    x=take(x);//这次归还了所有权
    take(x);
    //take(x);//在这里就不行了，被第二个东西弄走了
    let y=2;
    take2(y);
    take2(y);//栈上，小，可以认为这玩意实现了Copy trait~？？
    let s = String::from("hello");  // s 进入作用域

    takes_ownership(s);             // s 的值移动到函数里 ...
                                    // ... 所以到这里不再有效
}
fn takes_ownership(some_string: String) { // some_string 进入作用域
    println!("{}", some_string);
} // 这里，some_string 移出作用域并调用 `drop` 方法。
  // 占用的内存被释放