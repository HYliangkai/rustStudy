/** ## string
rust 核心库中只有str这一种数据--字面量
而String其实是标准库提供的 , 它表示*可增长*的、*可变的*、*有所有权的*、*UTF-8 编码*的字符串类型
&str:表示的是对str的借用:它们是一些储存在别处的 UTF-8 编码字符串数据的引用。比如字符串字面量被储存在程序的二进制输出中，
*/
pub fn string_one() {
    //新建字符串支持多种创建方式
    //1.先new()后push()
    let mut str1 = String::new();
    str1.push('1');
    str1.push_str("歇会吧");

    //str转
    let str2 = "iam sllh 's dog ".to_string();

    //from() -- 其实 from()函数是to_xxx()赠送的,作为序列化的两面
    let str3 = String::from("I am you");

    // String可以叠加,但是会失去所有权 -- 注意除了第一个后面的都要用 &String
    let fp_str = str1 + &str2 + &str3;
    // 可以用fromat!()宏进行拼接
    let fm_str = format!("{}", fp_str);
}
pub fn string_two() {}
