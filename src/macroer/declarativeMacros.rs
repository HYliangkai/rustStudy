/** ## 声明式宏
声明式宏（Declarative macros）使得你能够写出类似 match 表达式的东西，来操作你所提供的 Rust 代码。它使用你提供的代码来生成用于替换宏调用的代码。
声明式宏相当于对代码做正则表达式的匹配,落入对应的规则后做函数操作
 */
//声明宏的语法 #[macro_export] + macro_rules!
#[macro_export] //#[macro_export] 标注说明，只要将定义了宏的 crate 引入作用域，宏就应当是可用的。如果没有该标注，这个宏就不能被引入作用域。
macro_rules! aac {
    () => {};
}
