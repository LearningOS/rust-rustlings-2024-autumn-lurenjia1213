use std::sync::{Arc,Mutex};
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}
fn main() {
    let words = vec!["Hello", "World", "!"];
    let result = words.join(" "); // 使用空格作为分隔符 把这堆东西给拼起来
    println!("{}", result); // 输出: Hello World !


    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
    let mut handles = vec![];//Vec<std::thread::JoinHandle<()>>
    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));
            // TODO: You must take an action before you update a shared value
            status_shared.lock().unwrap().jobs_completed+=1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();//https://doc.rust-lang.org/std/thread/struct.JoinHandle.html
        // TODO: Print the value of the JobStatus.jobs_completed. Did you notice
        // anything interesting in the output? Do you have to 'join' on all the
        // handles?
        println!("jobs completed {}",status.lock().unwrap().jobs_completed);
    }
}
