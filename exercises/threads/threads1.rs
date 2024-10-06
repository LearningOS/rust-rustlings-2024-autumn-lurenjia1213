// threads1.rs
//
// This program spawns multiple threads that each run for at least 250ms, and
// each thread returns how much time they took to complete. The program should
// wait until all the spawned threads have finished and should collect their
// return values into a vector.
//
// Execute `rustlings hint threads1` or use the `hint` watch subcommand for a
// hint.

//

use std::thread;
use std::time::{Duration, Instant};

fn main() {
    let mut handles = vec![];
    for i in 0..10 {
        handles.push(thread::spawn(move || {//线程结构体
            let start = Instant::now();
            thread::sleep(Duration::from_millis(250));//卧槽，会在线程没结束前退出整个程序
            println!("thread {} is complete", i);
            start.elapsed().as_millis() //
        }));
    }
    //thread::sleep(Duration::from_millis(25000));//搞点鬼试试？
    let mut results: Vec<u128> = vec![];
    //let mut count =0;
    for handle in handles {
        // TODO: a struct is returned from thread::spawn, can you use it?
        let result = handle.join().unwrap();//主线程会为handle.join阻塞,直到这个线程结束，https://kaisery.github.io/trpl-zh-cn/ch16-01-threads.html 
        results.push(result);//保存线程信息~需要注意
        //println!("{count}");count+=1;

    }

    if results.len() != 10 {
       panic!("Oh no! All the spawned threads did not finish!");
    }
    /* 
    for wtf in &results{
        println!("{wtf}");
    }
error[E0382]: use of moved value: `results`
   --> exercises/threads/threads1.rs:50:24
    |
27  |     let mut results: Vec<u128> = vec![];
    |         ----------- move occurs because `results` has type `Vec<u128>`, which does not implement the `Copy` trait
...
39  |     for wtf in results{
    |                ------- `results` moved due to this implicit call to `.into_iter()`
...
50  |     for (i, result) in results.into_iter().enumerate() {
    |                        ^^^^^^^ value used here after move
    |
note: `into_iter` takes ownership of the receiver `self`, which moves `results`
   --> /home/codespace/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/traits/collect.rs:346:18
    |
346 |     fn into_iter(self) -> Self::IntoIter;
    |                  ^^^^
help: consider iterating over a slice of the `Vec<u128>`'s content to avoid moving into the `for` loop
    |
39  |     for wtf in &results{
    |                +

error: aborting due to 1 previous error

*/






 
    for (i, result) in results.into_iter().enumerate() {
        println!("thread {} took {}ms", i, result);
    }
/*results：这是一个集合（如 Vec、HashMap 等），包含了一些元素，通常是线程的返回值或其他数据。

into_iter()：这个方法会消耗 results 集合，并返回一个迭代器。这个迭代器会逐个访问集合中的元素。使用 into_iter() 后，原集合将不再可用。

enumerate()：这个方法会将迭代器中的每个元素与其索引配对，返回一个新的迭代器。每个元素会变成一个元组 (index, value)，其中 index 是元素的索引，value 是元素的值。 
这部分由GPT解答~

*/

}



/*

非常有意思的结果
thread 0 is complete
0
thread 2 is complete
thread 3 is complete
thread 4 is complete
thread 1 is complete
thread 6 is complete
thread 7 is complete
thread 5 is complete
1
2
3
4
5
6
7
thread 9 is complete
thread 8 is complete
8
9
thread 0 took 250ms
thread 1 took 250ms
thread 2 took 250ms
thread 3 took 250ms
thread 4 took 250ms
thread 5 took 250ms
thread 6 took 250ms
thread 7 took 250ms
thread 8 took 250ms
thread 9 took 250ms
*/
