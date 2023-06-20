/** ## .. 语法
在rust中，..语法通常用于表示范围或迭代器。具体来说，有以下几种用法：
1. 范围语法：start..end表示从start到end-1的范围，包括start但不包括end。例如：

```rust
let range = 1..4;
for i in range {
    println!("{}", i);
}
```
这里定义了一个范围1..4，表示从1到3的范围。然后使用for循环遍历该范围，并打印每个元素。

2. 迭代器语法：..表示一个空的范围，可以用于创建一个空的迭代器。例如：

```rust
let empty_range = ..;
let empty_vec: Vec<i32> = Vec::new();
let empty_iter = empty_vec.iter();
```

这里分别定义了一个空的范围、一个空的向量和一个空的向量迭代器。

3. 结构体语法：..表示结构体中未显式指定的字段，可以用于初始化结构体中的默认值。例如：

```rust
struct Point {
    x: i32,
    y: i32,
    z: i32,
}
let point = Point { x: 1, ..Default::default() };
```


这里定义了一个结构体Point，包含三个整数类型的字段x、y和z。然后使用..Default::default()语法来初始化结构体中未显式指定的字段，即将y和z的值初始化为默认值0。


4. ..语法还可以用于模式匹配中的通配符，表示匹配任意值。例如：

```rust
let tuple = (1, 2, 3);
match tuple {
    (1, ..) => println!("Matched 1"),
    (_, _, _) => println!("Matched something else"),
}
```
这里定义了一个元组tuple，包含三个整数类型的值。然后使用match语法对元组进行模式匹配，使用..通配符表示匹配任意值。如果元组的第一个值为1，则打印Matched 1，否则打印Matched something else
 */
pub fn use_dot() {}
