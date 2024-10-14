#[derive(PartialEq, Debug)]
pub enum List {
    Cons(i32, Box<List>), // 使用 Box 来包装递归的 List
    Nil,
}

fn main() {
    println!("This is an empty cons list: {:?}", create_empty_list());
    println!(
        "This is a non-empty cons list: {:?}",
        create_non_empty_list()
    );
}

pub fn create_empty_list() -> List {
    List::Nil // 空列表，直接返回 Nil
}

pub fn create_non_empty_list() -> List {
    // 非空列表，使用 Cons 构造，并递归构造下一个元素
    List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_empty_list() {
        assert_eq!(List::Nil, create_empty_list())
    }

    #[test]
    fn test_create_non_empty_list() {
        assert_ne!(create_empty_list(), create_non_empty_list())
    }
}
