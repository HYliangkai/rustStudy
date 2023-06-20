/** ## 数组
用于存放一堆相同数据类型的数据结构
rust中的数组并不怎么常用,因为相比之下vector可能更好用
 */
#[test]
pub fn use_array() {
    //数组类型的表现形式 [类型;长度]
    let a = [1, 2, 3, 4, 33];
    //访问使用 a[index] 访问即可
    println!("{:?}", a[1]);
}
