/**
## 错误处理
Rust 有一个分等级的错误处理方案：
如果某些东西可能不存在，则使用 Option
如果出了问题并且可以合理地处理，则使用 Result
如果有什么东西出错了，而且不能合理地处理，线程就会 panic
如果发生了灾难性的事情，程序就会直接中止（abort）
--------
对应程序内部能解决的错误,采用Option和Result来进行处理
*/
#[test]
fn use_optipn() {}

#[test]
fn use_result() {}
