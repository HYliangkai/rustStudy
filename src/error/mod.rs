/**
## 错误处理
Rust 有一个分等级的错误处理方案：
如果某些东西可能不存在，则使用 Option
如果出了问题并且可以合理地处理，则使用 Result
如果有什么东西出错了，而且不能合理地处理，线程就会 panic
如果发生了灾难性的事情，程序就会直接中止（abort）
--------
对应程序内部能解决的错误,采用Option和Result来进行处理
rust的错误处理是设计的优雅的地方,他不像其他语言使用error来抛出错误,使用null或者undefind来表示空的情况
正因为rust是一门很“安全”的语言,所以不允许null->即空指针的情况出现 ;
而rust采用Option和Result这两个enum来顶替null和error;由于rust中的enum允许使用函数,用enum来装载这两种情况再好不过
*/
#[test]
fn use_optipn() {
    //
}

#[test]
fn use_result() {}

#[test]
fn use_panic_and_abort() {}
