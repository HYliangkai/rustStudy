use std::{cell::RefCell, rc::Weak};

/** ## 引用记数和Weak<T>
+ 实现了 Deref 和 Drop 两种trait : 从而获取无需手动引用解引用就可以使用指针的特征
当引用记数为0时,数据就会自动释放掉,但是如果存在逻辑上的循环引用,这时引用记数不会为0,就会导致数据不被释放,最终导致内存泄露
因此需要存在一种数据当引用计数不为0也会被清除的弱引用数据,这就是 Weak<T>
Weak<T>的使用规则和Rc<T>是基本一致的
 */

//声明树结构数据
struct Tree {
    value: i32, //采用弱引用,避免出现头连尾的情况下不释放内存
    node: RefCell<Vec<Weak<Tree>>>,
    //设计思路: tree和node是一对多,并且只有node需要改变,所以用RefCell<Vec>   |  Tree存在多个node引用的情况,并且每个KidNode都可以修改FatherNode,所以要么用Rc<T>要么用Weak<T>

    //用Rc<T>会有两种情况出现:
    //1.当FatherNode丢失时KidNode的引用计数变为0,KidNode就会消失---这是正常情况
    //2.当kidNode丢失时FatherNode不丢失,但是KidNode此时的引用计数却不为0,KidNode不能消失 -- 所以会造成内存无法释放的问题

    //因此这个时候就要用Weak<T>:当引用计数不为0的时候也能准确释放内存
}
