use std::rc::Rc;

/** ## Rc<T> 引用计数智能指针

Rc<T> 允许在程序的多个部分之间只读地共享数据,而不改变其所有权
Rc<T> 用于当我们希望在堆上分配一些内存供程序的多个部分读取，而且无法在编译时确定程序的哪一部分会最后结束使用它的时候。如果确实知道哪部分是最后一个结束使用的话，就可以令其成为数据的所有者，正常的所有权规则就可以在编译时生效。
只在单线程下安全
应用场景:
+ 需要多所有权
*/
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

#[test]
fn use_rc() {
    use List::{Cons, Nil};
    // Rc(34 -> Nil)
    let rc_list = Rc::new(Cons(34, Rc::new(Nil)));
    //23 -> rc_list  ::> 使用Rc:clone不失去rc_list的所有权
    let rc_list1 = Cons(23, Rc::clone(&rc_list));
    //12 -> rc_list
    let rc_list2 = Cons(12, Rc::clone(&rc_list));

    //数据结构是 : a->c | b->c

    //观察引用记数
    println!("引用记数 is {:?}", Rc::strong_count(&rc_list))
}
