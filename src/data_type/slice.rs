/** ##  切片
切片是所有权机制下诞生的数据类型.slice是一段连续的内存引用,相当于是对源数据的借用,所以不会产生所有权转移
  ### 语法 --> 偏移量遵从range语法
  1. &origin_data[x..y]: 从x切到y
  2. &origin_data[..]: 切所有数据
  3. &origin_data[..1]: 从0切到1
  4. &origin_data[1..]: 从1切到底
  ==》x和y如果不写就是默认值(0和-1)
*/
#[test]
fn use_slice() {
    let origin_data = String::from("切片儿");
    //对origin_data进行引用切片,即slice

    let data_slice = &origin_data[0..3]; //[借用]0到3byte的数据
    assert_eq!(data_slice, String::from("切")); //utf-8一个中文占3位

    //字面量 "any_word"的数据类型是 &str : 它是一个指向二进制程序特定位置的 slice , 源数据不是mut,所以只能使用不能修改
}
