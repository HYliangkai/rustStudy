/** ## 智能指针
指针（Pointer）是一个包含内存地址的变量的通用概念，这个地址引用，或"指向（Point At）"一些其他数据。Rust中最常见的指针是引用（Reference），引用以"&"符号为标记借用了他们所指向的值，除了引用数据没有任何其他特殊功能，无额外开销
智能指针（Smart Pointer）是一类数据结构，他们不仅表现像指针，并且拥有额外的元数据与功能。智能指针起源于C++并存在于其他编程语言中。
在 Rust 中，普通引用和智能指针的一个额外的区别是引用是一类只借用数据的指针；相反，在大部分情况下，智能指针 拥有 它们指向的数据。
### rust标准库提供的智能指针:
String，Vec<T>，Box<T>，Rc<T>，Arc<T>，Weak<T>，Cell<T>，RefCell<T>，UnsafeCell<T>
### 智能指针具有的特征:
+ 实现了 Deref 和 Drop 两种trait : 从而获取无需手动引用解引用就可以使用指针的特征
+ 具有内部可变性 : 所谓内部可变性是指在*不申明mut的情况下内部的数据是可变的*,而一般的数据无论是指针或是非指针数据想要变化必须声明mut
 */

/** ###String -- 数据结构其实是 Vec<u8> : utf8的向量 */
pub struct LikeString {
    vec: Vec<u8>,
}

/** ## 使用场景
+ Rc<T> 允许相同数据有多个所有者；Box<T> 和 RefCell<T> 有单一所有者。
+ Box<T> 允许在编译时执行不可变或可变借用检查；Rc<T>仅允许在编译时执行不可变借用检查；RefCell<T> 允许在运行时执行不可变或可变借用检查。
+ 因为 RefCell<T> 允许在运行时执行可变借用检查，所以我们可以在即便 RefCell<T> 自身是不可变的情况下修改其内部的值。
*/
pub mod boxer;
pub mod weeker;

pub mod celler;
pub mod rcer;
