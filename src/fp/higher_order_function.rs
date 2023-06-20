/** ## 高阶函数
所谓高阶函数是指函数的参数可以是函数,或者函数的返回值可以是一个函数
高阶函数的本质是将函数作为函数指针进行传递
函数在rust中属于头等公民,即函数可在运行时创建,而不是在编译时创建
 */

#[test]
fn use_heiger_order_function() {
    //1.函数签名 : 函数的表达形式
    //语法为 :  (参数类型)->返回值类型

    //2.函数类型的表达形式: Fn 和 fn
    // Fn是一个trait,表示闭包函数,用于函数定义类型上,可以做到回调函数调用
    // fn表示函数指针 , 用于表示一个函数
    fn apply_twice<F>(f: F, x: i32) -> i32
    where
        F: Fn(i32) -> i32,
    {
        f(f(x))
    }
    let double = |x| x * 2;
    let result = apply_twice(double, 3);
    println!("Result: {}", result);

    fn use_fn(obs: i32) -> i32 {
        return obs + 22;
    }
    // 由于rust的语法,想要表达一个函数必须分两部写,表达一个闭包只要写一步
    let func: fn(i32) -> i32 = use_fn;
    println!("res is : {:?}", func(23))
}
