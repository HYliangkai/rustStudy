/** ## 生命周期
+ 生命周期是对引用数据的注解,是为了帮助rust编译器做更完善的编译
### 什么情况下需要做生命周期注解
1. 函数签名中入参中参数使用2个以上的引用,出参中使用一个以上的引用
譬如
```rust
fn need(pa:&i32,pb:&i32)->&str
```
2.定义结构体时属性是指针值时
```rust
struct Need{
  name:&str,
  age:i32
}
```
 */
#[test]
pub fn use_life_time() {}
