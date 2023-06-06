/**
函数式编程特征:迭代器
 */

//手动创建可迭代struct-->只要实现Iterator - triait 即可
pub struct can_iterator {
    the_var: i32,
}
impl can_iterator {
    fn new(var: i32) -> can_iterator {
        return can_iterator { the_var: var };
    }
}
impl Iterator for can_iterator {
    type Item = i32;
    //完成自定义的迭代行为
    fn next(&mut self) -> Option<Self::Item> {
        //&mut self  代表可变值的 struct
        self.the_var += 1;
        if self.the_var <= 20 {
            return None;
        } else {
            return Some(self.the_var);
        }
    }
}

//使用迭代器
pub fn use_iterator() {
    // 1.直接使用可迭代数组
    let iter_one = can_iterator::new(0);

    //2.将类迭代数组迭代器化,只需要调用 iter()函数即可
    let like_arr = vec![1, 3, 2, 1, 1, 2, 1, 20];
    let iter_two = like_arr.iter();
}
