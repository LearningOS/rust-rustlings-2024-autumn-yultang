// 实现冒泡排序
fn sort<T: Ord>(array: &mut [T]) {
    let len = array.len();
    // 外层循环控制遍历的次数
    for i in 0..len {
        // 内层循环进行元素比较和交换
        for j in 0..len - 1 - i {
            if array[j] > array[j + 1] {
                array.swap(j, j + 1); // 交换相邻元素
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }

    #[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }

    #[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}
