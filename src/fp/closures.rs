/**
闭包的概念是指下级代码块能使用上级代码快而上级代码块无法使用下级代码块的遮蔽行为
一个持有外部环境变量的函数就是闭包
 */

//1.闭包的语法
pub fn clos_fn1() {
    let mut A = 0;
    //1.定义闭包函数
    let _clofn = |num| {
        //闭包函数可以任意使用环境变量中的数据,譬如上方函数中的 A
        A += 1;
        println!("num is {:?}", num);
        return num;
    };
    //普通函数无法使用环境变量中的数据
    fn _norfn() {
        //Error --> 只能使用本代码块中的数据或者全局const 数据
        // A += 1;
    }
}

/**
在 rust中闭包函数和普通函数是区分开来的,是因为 rust 本身无 gc,只能通过区分函数的行为来做不同的优化
而在 js中因为有gc的存在,所有的函数都能秒变闭包函数
*/

#[test]
pub fn clos_fn2() {
    //返回类型 A: FnMut 获取可变的借用值所以可以改变其环境 | 可以多次调用
    fn do_this() -> impl FnMut() {
        let mut A = 0;
        //闭包语法 2:如果闭包内有 修改mut值的行为 闭包函数必须声明为  mut . 外部调用就在外部声明,内部调用就在内部声明
        //==>这样做的目的是为了将闭包函数的优化行为做再一步的区分

        //闭包语法 3:如果闭包内有必须的所有权转移行为(譬如如果不转移所有权do_this函数生命周期过了A就释放了,所以必须要所有权转移,注意转移后的所有权归闭包内,如果有下文是无法访问的) 使用  move 关键字来表示有所有权转移
        //==>可以看得出来 rust 通过不同的关键字来表示不同行为函数,以此做到不同的编译器优化
        let a_add = move || {
            A += 1;
            println!("a is {:?}", A);
        };
        return a_add;
    }
    let mut this = do_this();
    this();
    this();

    //返回类型B: FnOnce ,和 FnMut区别是这个一次性闭包调用,调用完之后就会销毁,相当于消费型闭包
    fn do_that() -> impl FnOnce() {
        let mut B = 0;
        let b_add = move || {
            B += 1;
            println!("{:?}", B);
        };
        return b_add;
    }
    //只能使用一次,所以也不需要声明 mut
    let that = do_that();
    that();

    //返回类型C: Fn ,只是[使用]了环境值,并没有做[修改]
    fn no_do() -> impl Fn() {
        let C = 0;
        //C的生命周期延生到外部,所以需要 move来移动所有权 ; 所以 move 行为和返回的类型没有关系
        let use_c = move || {
            println!("{:?}", C);
        };
        return use_c;
    }
    let nd = no_do();
    nd();
}
