use std::env;
mod variable;
fn main() {
    //读取cmd --> 将读取的迭代器数据转为集合vec
    let cmd: Vec<String> = env::args().collect();
    println!("cmd is {:?}", cmd);


}
