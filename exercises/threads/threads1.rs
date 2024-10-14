use std::thread;
use std::time::{Duration, Instant};

fn main() {
    let mut handles = vec![];
    for i in 0..10 {
        handles.push(thread::spawn(move || {
            let start = Instant::now();
            thread::sleep(Duration::from_millis(250));
            println!("thread {} is complete", i);
            start.elapsed().as_millis() // 返回经过的毫秒数
        }));
    }

    let mut results: Vec<u128> = vec![];

    for handle in handles {
        // 使用 join() 获取线程的返回值
        let result = handle.join().unwrap();
        results.push(result); // 将结果添加到向量中
    }

    if results.len() != 10 {
        panic!("Oh no! All the spawned threads did not finish!");
    }

    println!();
    for (i, result) in results.into_iter().enumerate() {
        println!("thread {} took {}ms", i, result);
    }
}
