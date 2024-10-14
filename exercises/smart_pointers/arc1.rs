#![forbid(unused_imports)]
use std::sync::Arc;
use std::thread;

fn main() {
    let numbers: Vec<_> = (0..100u32).collect();
    // 使用 Arc 包装 numbers，使其可以在线程间共享
    let shared_numbers = Arc::new(numbers);
    let mut joinhandles = Vec::new();

    for offset in 0..8 {
        // 克隆 Arc，使得每个线程都能访问相同的共享数据
        let child_numbers = Arc::clone(&shared_numbers);
        joinhandles.push(thread::spawn(move || {
            // 计算偏移量对应的和
            let sum: u32 = child_numbers.iter().filter(|&&n| n % 8 == offset).sum();
            println!("Sum of offset {} is {}", offset, sum);
        }));
    }
    
    // 等待所有线程结束
    for handle in joinhandles.into_iter() {
        handle.join().unwrap();
    }
}
