use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // 创建两个互斥锁
    let lock1 = Arc::new(Mutex::new(0));
    let lock2 = Arc::new(Mutex::new(0));

    // 创建第一个线程
    let lock1_clone = Arc::clone(&lock1);
    let lock2_clone = Arc::clone(&lock2);
    let thread1 = thread::spawn(move || {
        let _guard1 = lock1_clone.lock().unwrap(); // 锁定第一个互斥锁
        println!("Thread 1 acquired lock1");

        // 模拟一些工作
        thread::sleep(std::time::Duration::from_millis(100));

        // 尝试锁定第二个互斥锁
        let _guard2 = lock2_clone.lock().unwrap(); // 这里可能会导致死锁
        println!("Thread 1 acquired lock2");
    });

    // 创建第二个线程
    let lock1_clone = Arc::clone(&lock1);
    let lock2_clone = Arc::clone(&lock2);
    let thread2 = thread::spawn(move || {
        let _guard2 = lock2_clone.lock().unwrap(); // 锁定第二个互斥锁
        println!("Thread 2 acquired lock2");

        // 模拟一些工作
        thread::sleep(std::time::Duration::from_millis(100));

        // 尝试锁定第一个互斥锁
        let _guard1 = lock1_clone.lock().unwrap(); // 这里可能会导致死锁
        println!("Thread 2 acquired lock1");
    });

    // 等待线程完成
    thread1.join().unwrap();
    thread2.join().unwrap();
}
