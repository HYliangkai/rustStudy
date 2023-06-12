/** ## 枚举
rust中的枚举是一种很强大的数据结构,不同于其他语言rust的枚举可以附加函数,而且还可以为枚举做模式匹配(match),这是最特别的两个点
 */

//step 1 : 定义枚举
enum IPNetWork {
    V4,
    V6,
    V8,
}
//step2 : 实现附加函数
impl IPNetWork {
    pub fn found_ip(ip_str: &str) -> Option<IPNetWork> {
        if ip_str == "v4" {
            return Some(IPNetWork::V4);
        } else if ip_str == "v6" {
            return Some(IPNetWork::V6);
        } else if ip_str == "v8" {
            return Some(IPNetWork::V8);
        } else {
            return None;
        }
    }
}

//step 3 : 使用
#[test]
pub fn use_enum() {
    let ipo = IPNetWork::found_ip("v4");
}
