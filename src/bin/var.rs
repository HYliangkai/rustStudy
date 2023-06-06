//两种方式使用 bin 文件夹以外的代码

//A. 使用 包名::模块 方式
use study::pack;
//A.B 一旦使用了A 方式,相当于把 study/pack 这个包引入本地,这时候就可以使用crate表示引入包(study)
use crate::pack::grandfather;
fn main() {
    grandfather::grandfather_say();
}
