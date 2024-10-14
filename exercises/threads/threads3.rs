use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct Queue {
    length: u32,
    first_half: Vec<u32>,
    second_half: Vec<u32>,
}

impl Queue {
    fn new() -> Self {
        Queue {
            length: 10,
            first_half: vec![1, 2, 3, 4, 5],
            second_half: vec![6, 7, 8, 9, 10],
        }
    }
}

fn send_tx(q: Arc<Queue>, tx: Arc<Mutex<mpsc::Sender<u32>>>) {
    let qc1 = Arc::clone(&q);
    let qc2 = Arc::clone(&q);

    let tx1 = Arc::clone(&tx);
    thread::spawn(move || {
        for val in &qc1.first_half {
            println!("sending {:?}", val);
            let sender = tx1.lock().unwrap(); // 获取锁
            sender.send(*val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    let tx2 = Arc::clone(&tx);
    thread::spawn(move || {
        for val in &qc2.second_half {
            println!("sending {:?}", val);
            let sender = tx2.lock().unwrap(); // 获取锁
            sender.send(*val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
}

fn main() {
    let (tx, rx) = mpsc::channel();
    let queue = Arc::new(Queue::new()); // 使用 Arc 包装 Queue
    let queue_length = queue.length;

    let tx = Arc::new(Mutex::new(tx)); // 使用 Arc 和 Mutex 包装 Sender

    send_tx(queue, tx);

    let mut total_received: u32 = 0;
    for received in rx {
        println!("Got: {}", received);
        total_received += 1;
    }

    println!("total numbers received: {}", total_received);
    assert_eq!(total_received, queue_length);
}
