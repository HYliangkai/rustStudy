/** ## 模式匹配
模式匹配是rust中最爽的语法,作为switch的升级版本,和switch不同的是match本质上是一个表达式,用于匹配有限的情况(譬如enum),所以可以作为结果值进行返回
 */

//type one : 匹配enum
enum NeedToMatch {
    NSD,
    FFI,
    CPU,
    GPU,
}
#[test]
fn macth_enum() {
    let A = NeedToMatch::NSD;
    //match作为表达式的情况
    let res = match A {
        //语法: 情况  => 语句 ,
        NeedToMatch::CPU => {
            println!("我是cpu");
            1
        }
        NeedToMatch::FFI => {
            println!("我是FFI");
            2
        }
        // _ 表示其他情况(default)
        _ => {
            println!("我是其他");
            3
        }
    };

    //当只需要匹配一个值,而其他不进行匹配的时候使用 if let : 相当于直接匹配条件而忽略其他
    let vov = NeedToMatch::NSD;
    // if let 匹配值 = 条件值 {语句}
    if let NeedToMatch::NSD = vov {
        // *let a=b的本质原理是:将b的值绑定到a变量上,本质上也是属于一种模式匹配 .
        // 譬如元组的赋值也是一种模式匹配 let (x, y) = (1, 2, 3);
        // 那么 NeedToMatch::NSD = vov 就是将vov的值和NeedToMatch::NSD进行匹配;此时可能会匹配失败的赋值方式称之为refutable(可反驳的)
        println!("符合条件");
    } else {
        println!("不符合条件");
    }
    // white let 匹配值 = 条件值 {语句}  / 原理和上面的let原理一样
    while let NeedToMatch::NSD = vov {
        println!("符合跳出条件");
    }
}

//type two: 匹配tuple
#[test]
fn match_tuple() {
    //匹配tuple其实也是可辩驳模式的匹配,通过tuple的形态来进行匹配
    let the_tuple = (1, "hello", true);
    match the_tuple {
        //以第一个匹配上的为结果
        (1, _, _) => println!("匹配第一种"),
        (_, "hello", _) => println!("匹配第二种"),
        (_, _, true) => println!("匹配第三种"),
        _ => println!("匹配剩余"),
    }
}

//type three: 匹配struct
//匹配struct亦如此
#[test]
fn macth_struct() {
    struct wantmatch {
        name: Option<String>,
        age: i32,
    }
    let the_a = wantmatch {
        age: 32,
        name: None,
    };
    match the_a {
        //struct中的缺省是 ..
        wantmatch { age: 32, .. } => println!("匹配成功"),
        _ => println!("匹配失败"),
    }
}

//type four: 匹配字面量
#[test]
fn match_str() {
    let str_word = "我要飞翔";
    //在match中必须有一个不可辩驳的匹配
    match str_word {
        "你是" => println!("匹配不到"),
        "我要飞翔" => println!("匹配到了"),
        _ => println!("hhh"),
    }
}

/** ### 可辩驳与不可辨驳
顾名思义.可辩驳就是允许赋值失败的情况,不可辩驳就是不允许赋值失败的情况

这是不可辩驳: let a = ast;

这是可辩驳: let Some(s) = tsa;

可辨驳匹配缩窄了赋值的范围;而不可辩驳不做限制

基于此，match 匹配分支必须使用可反驳模式，除了最后一个分支需要使用能匹配任何剩余值的不可反驳模式。Rust 允许我们在只有一个匹配分支的 match 中使用不可反驳模式，不过这么做不是特别有用，并可以被更简单的 let 语句替代。
*/
#[test]
fn test_bianbo() {}
