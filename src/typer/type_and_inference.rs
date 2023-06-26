/** ## 类型系统
+ rust中的never类型是: `!`  <-->  如果一个函数的返回类型是!，那么它可以返回任何其他类型的值，因为它永远不会返回。
+ rust可以起类型别名
*/

#[test]
pub fn run_typer() {
    // @ 定义类型别名:为一种类型起别名,类似于ts,但是是简单版的
    type Many = (f32, i32);
    enum More {
        One,
        Two,
    }
    type Moreer = More;
    //类型别名的一个主要作用是避免类型代码冗余<you known>
    type Boxer = Box<dyn Fn() + Send + 'static>;
}
