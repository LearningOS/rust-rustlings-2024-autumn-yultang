// options3.rs
//
// Execute `rustlings hint options3` or use the `hint` watch subcommand for a
// hint.


struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });

    match y {
        Some(p) => println!("Co-ordinates are {},{} ", p.x, p.y),
        None => panic!("no match!"), // 处理 None 情况
    }

    // 在这里可以安全地使用 y，因为 match 语句保证了 y 不是 None
    let _ = y; // 这行代码可以防止未使用变量的警告
}
