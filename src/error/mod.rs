use std::{
    error::Error,
    fs::File,
    io::{Read, Write},
};

/**
## 错误处理
Rust 有一个分等级的错误处理方案：
1. 如果某些东西可能不存在，则使用 Option
2.如果出了问题并且可以合理地处理，则使用 Result
3.如果有什么东西出错了，而且不能合理地处理，线程就会 panic
4.如果发生了灾难性的事情，程序就会直接中止（abort）
--------
对应程序内部能解决的错误,采用Option和Result来进行处理
rust的错误处理是设计的优雅的地方,他不像其他语言使用error来抛出错误,使用null或者undefind来表示空的情况
正因为rust是一门很“安全”的语言,所以不允许null->即空指针的情况出现 ;
而rust采用Option和Result这两个enum来顶替null和error;由于rust中的enum允许使用函数,用enum来装载这两种情况再好不过
*/
#[test]
fn use_optipn() {
    //Option用于替代null存在的场景,一个Option<s>所代表的类型是 s|null
    //用Some(value)来表示值存在的情况 , 用 None表示值不存在的情况
    let mike: Option<String> = Some("home".to_string());
    let john: Option<String> = None;
    match &mike {
        Some(ss) => println!("{:?}", ss),
        None => println!("没回来"),
    }
    match &john {
        Some(ss) => println!("{:?}", ss),
        None => println!("john没回来"),
    }
    //对option的介绍主要是介绍其函数,常用函数玩的6,那自然是无敌

    //1.unwrap() : 如果是some值就返回值否则就panic
    let mike_live = mike.clone().unwrap();
    println!("mike is in {:?}", mike_live);

    //1.25.unwrap_or() : op1.unwarp_or(value) 如果unwarp的结果是None就用or里面的内容替代
    let john_bro = john.clone().unwrap_or(String::from("我是John大表哥"));
    println!("{:?}", john_bro);

    //1.5 unwarp_or_default() 用于将None值转为Some类型的default值,譬如string的default是 " "
    let john_default = john.clone().unwrap_or_default();
    println!("None值的默认{:?}", john_default);

    //1.75 unwarp_or_else(fn) 如果值为None则将闭包值作为值返回
    let john_else = john
        .clone()
        .unwrap_or_else(|| return String::from("杂七杂八"));
    println!("None值的改造{:?}", john_else);

    //2.expect() : 和unwrap一样只不过可以自定义错误信息
    // let john_live = john.expect("不在家");

    //3.is_some() : 如果值是some返回true否则返回false
    println!("john live in home ? >>> {:?}", john.is_some());
    //4.is_none() : 同上但取反
    println!("john live outside ? >>> {:?}", john.is_none());

    //5.map() : 将Option<A>通过map函数转化成Option<B>
    let jiojio = mike.clone().map(|s| {
        let mut a = s.clone();
        // push_str通过改变源字符串而进行追加
        a.push_str("我回来辣");
        return a;
    });
    println!("{:?}", jiojio.clone().unwrap());

    //6.and() : && 符号的函数化表现形式 op1.and(op2) : 如果有一个是None返回None ; 如果都是Some返回最后一个Some即op2
    let and_res = jiojio.clone().and(mike.clone()).unwrap();
    println!("and 的 结果是{:?}", and_res);

    //7.or() : ||符号的函数化表现形式 op1.or(op2) 如果op1为None返回op2的值,不论Some/None
    let or_res = john.clone().or(jiojio.clone()).unwrap();
    println!("or 的 结果是{:?}", or_res);

    //8.take() : 将Option中的值取出并返回一个新的Option，同时将原Option置为None。这个方法通常用于获取Option中的值并将其转移给其他变量，以 [ 避免所有权问题 ]。以下是一个使用take()函数的例子：
    //Tips: take()必须拥有mut权限,因为他是需要改变源数据的,
    let mut dio = jiojio.clone();
    let take_res = dio.take(); //只是改变源数据,并不转移所有权
    println!(
        "改变 {:?} -> {:?}",
        dio.unwrap_or_default(),
        take_res.unwrap()
    );
}

//Option的`?`语法糖 : 如果值为None就作为代码块的返回值,否则直接取值
fn uu_test() -> Option<()> {
    let aoc: Option<i32> = None;
    let boc = aoc.clone()?;
    //如果真的没有什么需要返回,就返回一个空的元组
    Some(())
}

#[test]
//Box<dyn Error>可以表示任何对象
fn use_result() -> Result<(), Box<dyn Error>> {
    //Result<T,E>有两个成员:Ok(T)和Err(E),代表 成功(信息) 和失败(信息)
    //关于Resule<T,E>还有一个语法糖`?`: 如果在一个Result<T,E>后面加上`?`,表示  如果当前Result的结果是Err的话则作为当前代码块的返回值Err(E)返回
    let res_ok: Result<i32, String> = Ok(1024);
    let res_err: Result<i32, String> = Err(String::from("错误"));
    //`?`语法糖
    let a = File::open("a.txt")?;
    //函数
    // A. unwrap() | B. expect() | C. unwrap_or_else() | D. map() | 功能和option一致

    //E. map_err(f: FnOnce(E) -> F): 对Err情况进行构造返回新值 如果Result是Err，则将包含在Err中的值应用函数f并返回新的Ok值，否则返回原始的Ok。

    //F. and_then(f: FnOnce(T) -> Result<U, E>): 如果Result是Ok，则将包含在Ok中的值应用函数f并返回新的Result，否则返回原始的Err。

    //G. or_else(f: FnOnce(E) -> Result<T, F>)：如果Result是Err，则将包含在Err中的值应用函数f并返回新的Result，否则返回原始的Ok。
    return Ok(());
}

#[test]
fn use_panic_and_abort() {
    //panic和abort的区别: panic相当于点击程序的“退出”按钮,由程序负责清理内存并且退出;而abort相当于电脑的“强制退出”按钮,由操作系统强制清理内存并且退出
    //很多情况会导致panic,譬如数组越界,手动panic云云
    panic!()
}
