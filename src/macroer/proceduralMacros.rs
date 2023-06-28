/** ## 过程宏
过程宏（Procedural macros）允许你操作给定 Rust 代码的抽象语法树（abstract syntax tree, AST）。过程宏是从一个（或者两个）TokenStream到另一个TokenStream的函数，用输出的结果来替换宏调用。
过程宏就是根据代码的AST做匹配,然后生成代码
 */
fn prod() {}
/**### derive 宏
+ derive 只能用于结构体和枚举
+ 作用是为属性生成代码
+ 使用是在struct或enum上声明 #[derive(xxx)]
*/
fn deri() {}

/** ### 类属性宏
+ 它们允许你创建新的属性
+ derive 只能用于结构体和枚举；属性还可以用于其它的项，比如函数
譬如rocket框架
```rust
#[route(GET, "/")]
fn index() {}
```
就是用了类属性宏
 */
fn lsxh() {}

/** ### 类函数宏
+ 可以接受未知数量的参数
例如
```rust
let sql = sql!(SELECT * FROM posts WHERE id=1);
```
就是用了类函数宏
 */
fn lhsh() {}
