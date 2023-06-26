use std::time::{self, SystemTime, UNIX_EPOCH};

/** ## Box<T>
表示指向堆上的数据
最简单直接的智能指针是 box，其类型是 Box<T>。 box 允许你将一个值放在堆上而不是栈上。留在栈上的则是指向堆数据的指针。
除了数据被储存在堆上而不是栈上之外，box 没有性能损失。不过也没有很多额外的功能。它们多用于如下场景：
+ 当有一个在编译时未知大小的类型，而又想要在需要确切大小的上下文中使用这个类型值的时候  -->嵌套结构体
+ 当有大量数据并希望在确保数据不被拷贝的情况下转移所有权的时候 -->不希望数据拷贝
+ 当希望拥有一个值并只关心它的类型是否实现了特定 trait 而不是其具体类型的时候 -->类型安全
*/
#[test]
pub fn use_box() {
    let abc = Box::new(321);
    println!("{:?}", abc);

    //使用递归的场景时是使用Box的一个经典场景,因为rust需要在编译的时候知道所有数据的大小,而递归是在运行的时候才知道数据大小的,所以需要使用Box<T>给出一个明切的数据大小(就是指针的大小嘛)
}
/**rust为数据分配内存时是采用 内存最大情况值  来分配内存的 -- 这个很好理解,因为内存大值可以兼容下内存小值,所以只要分配最大情况值来分配就行了
譬如下面情况,rust就不知道内存该分配多少
```rust
enum NSee {
    Cons(i32, NSee),
    Nil,
}
```
因为为NSee分配空间的时候取最大值Cons(i32,NSee),而Cons(i32,NSee)的内存大小又是重回NSee,呈无限递归的大小
因此要告诉编译器这边的大小只是一个指针的大小即Box<T>即可通过内存划分判断
*/
enum List {
    Cons(i32, Box<List>),
    Nil,
}
//创建单向链表
fn listting() {
    use List::{Cons, Nil};
    // 1 --> 2 --> nil
    let list_one = Cons(1, Box::new(Cons(2, Box::new(Nil))));
}

#[test]
fn pbnq() {
    //函数递归居然一点事没有,这是因为函数的大小是确定的?(函数指针)
    println!(
        "{:?}",
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
    );
    fn fb(n: i32) -> i32 {
        return if n == 1 || n == 2 {
            1
        } else {
            fb(n - 2) + fb(n - 1)
        };
    }

    println!(
        "{:?}--{:?}",
        fb(35),
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
    );
}
