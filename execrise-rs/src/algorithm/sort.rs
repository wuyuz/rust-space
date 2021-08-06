
mod test {
    // 冒泡排序
    fn bubble_sort(a: &mut Vec<i32>) {
        let mut n = a.len();

        while n > 0 {
            let (mut i, mut max_ptr) = (1, 0);
            
            // 冒泡开始，如果前者大于后者则相互换位置，第一轮会得到最大值
            while i < n {
                if a[i-1] > a[i] {
                    a.swap(i-1, i);
                    max_ptr = i;
                }
                i += 1;
            }

            // 本次遍历的最大值位置也是下一轮的冒泡的终点
            n = max_ptr;
        }
    }

    #[test]
    fn bubble_test() {
        let mut a = vec![1,2,4,15,12,56,2];
        bubble_sort(&mut a);
        println!("{:?}",a);
    }
}
