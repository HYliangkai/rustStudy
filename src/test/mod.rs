/** ## 测试
用rust时常用测试来替代项目调试,一个很重要的原因是rust编译的速度实在太慢了,而单元测试能只跑一个函数从而验证结果的样式实在令人向往
rust核心包中有3个过程宏: assert!()  assert_eq!() assert_ne!() 来进行结果验证,如果结果验证失败就会panic
 */

//0. 标记 #[cfg(xxx)]表示在xxx条件下进行条件编译,而#[cfg(test)]表示只在测试环境下编译下面的代码
#[cfg(test)]
//1. 标记  #[test的函数是测试函数]普通函数不能直接调| 使用`cargo test`能运行所有的测试函数
#[test]
pub fn test_one() {
    let users = vec![12, 123, 1222, 1024];
    for item in users {
        if item > 200 {
            println!("大于100!");
        } else {
            println!("小于100!");
        }
    }
}

#[test]
pub fn test_two() {
    //断言宏 : 结果为true就不会panic
    assert!(true);
    assert!(1 == 1);
    fn you(name: &str) -> &str {
        return "sb";
    }
    let ad = "sb";
    //错误信息写在第二个参数后,错误信息格式和format!()宏的格式一样(内部就是调用这个宏)
    assert!(you("are") == "sb", "你不是{}", ad);

    //相等断言,相等就不会panic
    assert_eq!("1", "1");
    //不等断言,不想等就不会panic
    assert_ne!("12", "11");
}

//异常断言宏,用于标识该测试函数可能会异常,但是  允许在测试函数异常的情况下运行其他测试函数
#[test]
#[should_panic]
fn i_will_panic() {
    // panic!("我一定会panic");
}
