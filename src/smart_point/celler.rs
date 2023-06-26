use std::{
    cell::{RefCell, RefMut},
    rc::Rc,
};

/** ## Cell 和 RefCell
显著特点是具有 内部可变性
内部可变性的使用场景是 : 当数据的整体不需要可变,而只有数据的某一部分需要改变时,这时可用内部可变性将这一部分先从外部声明为不可变,但内部是可变的
例子:
```rs
struct A{
  name:String,
  age:i32,
  address:String
}
```
如果这个structA我们只要改变address,而不需要改变其他属性,但是我们想要修改address只能做到整体可变
即 let mut aa=A{....}
这样就增加了修改数据的风险,而且没有一种更细分的可变不可变划分
所以这时就需要内部可变性数据
```rs
struct A{
  name:String,
  age:i32,
  address:Cell<String>
}
let aaa=A{....}
```
这样整体不可变,但是局部的内部是可变的
内部可变性的具体实现是调用unsafe-rust来实现的
### 区别
由于Cell要实现copy trait , 所以还是RefCell使用多一些

RefCell和Cell的区别在于：

RefCell可以存储任意类型的值，而Cell只能存储Copy类型的值。

RefCell在运行时进行借用检查，如果违反了借用规则，会引发panic。而Cell不进行借用检查，因此不会引发panic。

RefCell可以通过borrow方法获取可变引用，允许在不可变引用存在的情况下修改数据。而Cell只能通过set方法修改数据，不需要获取引用。

需要注意的是，RefCell和Cell都是在单线程环境下使用的。在多线程环境下，应该使用Rust提供的其他线程安全的类型，如Mutex和RwLock。

### 应用场景:
+ 数据小部分需要改变,整体大部分不需要改变
*/

struct User {
    name: String,
    age: i32,
    address: RefCell<String>,
}
impl User {
    fn set_address(&self, ch: String) {
        //修改地址,在不声明 &mut self的情况下

        //获取不可变数据(示例)
        // let adr1 = self.address.borrow();
        // println!("目前的地址是 : {:?}", adr1);

        //注意无论是borrow()还是borrow_mut(),都要遵循所有权规则 ; 即let adr1 = self.address.borrow();之后所有权会进行转移,没办法下一步操作
        *self.address.borrow_mut() = ch;
        println!("修改后的地址是 : {:?}", self.address.borrow())
    }
}
#[test]
fn use_refcell() {
    let user = User {
        name: String::from("jiojio"),
        age: 18,
        address: RefCell::new(String::from("小何")),
    };
    user.set_address("呵呵呵".to_string());
}

//Rc + RefCell == 允许多人借用并且内部可变  == 允许多数据修改
#[test]
fn more_modify() {
    struct People {
        name: Rc<RefCell<String>>,
    }
    let p1 = People {
        name: Rc::new(RefCell::new("jiojio".to_owned())),
    };
    //同时获得所有权
    let p2 = Rc::clone(&p1.name);
    let p3 = Rc::clone(&p1.name);

    //同时修改数据
    println!("修改前 : {:?}", Rc::clone(&p1.name).borrow());
    *p2.borrow_mut() = "213".to_owned();
    println!("第一次修改 : {:?}", Rc::clone(&p1.name).borrow());
    *p3.borrow_mut() = "222".to_owned();
    println!("第二次修改 : {:?}", Rc::clone(&p1.name).borrow());
}
/**
+ Box<T> : 包转一组数据进堆内存,方便编译器计算出内存大小
+ Rc<T> : 允许不使用借用规则的情况下数据拥有多所有权
+ RefCell<T> : 允许在不可变的情况下修改内部数据
注意以上的智能指针只是拥有一部分能力,想要获取多种能力必须结合在一起使用
 */
struct 总结 {}
