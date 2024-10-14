use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    // 使用 Arc 和 Mutex 来共享 JobStatus
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
    let mut handles = vec![];

    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));
            // 获取锁并更新共享值
            let mut status = status_shared.lock().unwrap();
            status.jobs_completed += 1; // 更新 jobs_completed
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap(); // 等待所有线程完成
    }

    // 在这里获取并打印总的 jobs_completed
    let status = status.lock().unwrap();
    println!("jobs completed {}", status.jobs_completed);
}
