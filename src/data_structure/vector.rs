/** ## Vector-可变数组
用于存放一堆相同数据类型的数据结构
在rust中，数组（array）和向量（vector）都是用于存储一组值的数据结构，但是它们有一些区别。

* **大小不同**：数组的大小是固定的，一旦定义就不能改变，而向量的大小是可变的，可以动态添加或删除元素。

* 内存分配不同：数组在定义时就会分配一段连续的内存空间，而向量则会在需要时动态分配内存空间。

* 访问方式不同：数组的元素可以通过下标来访问，例如`array[0]`，而向量的元素可以通过迭代器来访问，例如`vector.iter().next()`。

* 类型不同：数组的类型是`[T; N]`，表示包含N个类型为T的元素的数组，而向量的类型是Vec<T>，表示包含任意数量类型为T的元素的向量。

总的来说，数组适用于存储固定数量的元素，而向量适用于存储可变数量的元素。在需要动态添加或删除元素的情况下，向量更加方便和灵活。

rust的vector更像是js中的array: 作为struct的一部分,可以自由伸缩长度
 */
#[test]
pub fn use_vector() {
    //类型签名 Vec<T>

    //构造创建,但是需要声明类型
    let mut vec_one: Vec<i32> = Vec::new();
    //宏创建
    let mut vec_two = vec![1, 2];
    //数组式声明
    let mut vec_three = vec![2; 10];
    vec_one.push(18);
    vec_two.pop();
    //如果 map的item是 &i32 ; 那么设置参数为&item就可以省去一步
    let vec_four: Vec<i32> = vec_three.iter().map(|&item| item + 32).collect();
    println!("{:?}", vec_four);

    //获取值 1.slice获取,缺点是有可能会有越界panic,所以少用
    println!("第二个值{:?}", &vec_four[1]);

    //获取值 2.get()获取: get()->Optio<value> ;不会造成越界的panic问题,越界了就会返回None;相当于加强版的at() !!
    let &bsd = vec_four.get(2).unwrap();
    println!("第三个值{:?}", bsd);

    //for遍历
    for &item in &vec_four {
        println!("item : {:?}", item);
    }

    //由于vector只能存储同一类型的数据,如何用vector存储有限类型的数据呢?答案是用enum
    enum some_type {
        typp1(i32),
        typp2(String),
        typp3(i64),
    } //但是u1s1,一点都不优雅
    let mut mt = vec![
        some_type::typp1(12),
        some_type::typp2(String::from("你好啊")),
        some_type::typp3(121121),
    ];
    //用enum的好处是便于做模式匹配
    let res: Vec<i32> = mt
        .iter()
        .map(|item| {
            return match item {
                some_type::typp1(_) => 12,
                some_type::typp2(_) => 13,
                some_type::typp3(_) => 14,
            };
        })
        .collect();
    //遍历后结果是:
    println!("遍历后结果是:{:?}", res);

    //另一种方式就是用trait的多态能力进行存储
}
