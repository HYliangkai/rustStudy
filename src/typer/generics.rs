/** ## 泛型
从语法层面上看泛型和生命周期的注解位置是一致的
rust的泛型采取单态化策略,即编译时确定泛型的具体类型,所以说白了泛型是给用户使用的 : 避免用户因为类型的区别写更多的代码,而实际上更多的代码还是要写的,只不过是编译器帮你写
而rust的生命周期注解是给编译器使用的,为了确保编译器能在不借用gc的情况下能正确回收资源,所以需要我们手动打标注来来替代gc的功能
 */

// 定义泛型: 在名称后面(可用范围:需要用到类型定义的地方 )用 < > 表示,接下来就可以在中里面使用
pub fn use_generics_first<T>(cod: T) {
    //此时的T可以表示任何类型,但是其实没啥用,因为any类型上能调用的实例方法很少,所以这时要用trait的多态能力进行泛型收窄
}

trait Foo {}
trait Fcc {}

//收窄到只有实现
fn generice_contract<T: Foo + Fcc>(cod: T) {
    //只有实现了Foo+Fcc才能调用
}
//由于rust使用组合替代继承,所以涉及到多个trait的时候泛型签名会很长,因此使用where关键字来使得代码易读点
trait Fdd {}
trait Fee {}

fn use_where_keyword<T>()
where
    T: Fdd,
{
    //相当于把T的限制条件搬到where上,增加可读性
}

//可定义 [默认泛型类型] ,使用的时候如果没有定义就使用默认泛型类型
struct DefaultGenerics<T = i32, R = String> {
    age: T,
    kid: R,
}

struct A {
    age: i32,
}
trait Run {
    fn running(&self);
}
impl Run for A {
    fn running(&self) {
        println!("我在跑步");
    }
}
//dyn关键字,强调类型是依赖于某个trait动态分发的,因此不会进行泛型的单态化,避免包膨胀
// dyn的使用有两种形式 :
//  第一种是传入 &dyn trait作为类型,(输入的时候要输入指针)
//  第二种是传入装箱数据 Box<dyn trait>  ;其实两种都是传入地址
fn hello_world(a: &dyn Run) {
    a.running()
}

#[test]
fn test_dyn() {
    let aa = A { age: 18 };
    hello_world(&aa)
}
