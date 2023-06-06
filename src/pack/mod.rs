pub mod find_me;
pub mod grandfather {
    pub fn grandfather_say() {
        println!("我是你爷爷");
    }
    pub mod father {
        pub fn father_say() {
            println!("我是你爸爸");
        }
        pub mod son {
            pub fn son_say() {
                println!("我是你儿子");
            }
        }
        pub mod sonB {
            pub mod sonC {
                pub fn sonC_say() {
                    println!("我是你儿子的儿子的儿子");
                }
            }
            pub fn call_son() {
                //模块调用方式 A 相对路径调用

                //同级模块调用 : 类似于 ./sonC
                self::sonC::sonC_say();
                //同顶级模块调用 : 类似于 ../son
                super::son::son_say();

                //模块调用方式 B 绝对路径调用 : 以
                crate::pack::grandfather::grandfather_say();
            }
        }
    }
}
