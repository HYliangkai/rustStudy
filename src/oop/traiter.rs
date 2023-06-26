/** ## method
为struct实现方法
使用impl来为rust实现方法
根据函数中是否含有self来判断方法的类型
函数中有self的是实例方法/函数中没有self的是静态方法
至于实例方法和静态方法的区别就是一个有self--struct实例
*/

#[derive(Debug, Default)]
struct User {
    name: String,
    age: i32,
    sex: bool,
    address: Option<String>,
}

//可以存在多个impl,所以其实一个impl放self一个impl放非self
impl User {
    //impl中还可以存放const作为一个静态的变量;想要使用要使用 Struct::const  来使用
    const PI: f64 = 3.14159;
    //还可以声明static
    fn new(name: String, age: i32, sex: bool, address: Option<String>) -> User {
        return User {
            name,
            age: age * User::PI as i32,
            sex,
            address,
        };
    }
}
impl User {
    //如果是要修改struct,要使用 &mut self  /  如果只是取值就用&self
    //同时如果要使用到&mut , struct声明也要用到mut
    fn change_age(&mut self, age: i32) -> () {
        self.age = age * User::PI as i32;
        return ();
    }
    fn get_age(&self) -> i32 {
        return self.age;
    }
}

/** ## trait
使用trait代码能获取多肽+特征复用等效果;是rust实现工程化的必备良药
trait 告诉 Rust 编译器某个特定类型拥有可能与其他类型共享的功能。可以通过 trait 以一种抽象的方式定义共享的行为。可以使用 trait bounds 指定泛型是任何拥有特定行为的类型。
 */
pub trait Run {
    //默认泛型实现
    //在trait中定义一组共享的数据---可以有默认实现,或者是只有签名要使用的时候来实现

    //1.定义共享const
    const MAX_AGE: i32; //签名版
    const MIN_AGE: i32 = 0; //默认实现版

    //2.定义关联类型,在实现的时候进行赋值
    type RunType; //注意关联类型只需要在某个实现中定义一次接下来就不用定义了,相当于公共代码
                  //注意关联类型没有默认实现

    //3.定义共享函数,还是可self可不self
    fn print(&self); //只定义函数签名
    fn say(code: <Self as Run>::RunType) {
        //注意由于trait具有多态,所以使用self的是h
        println!("我是默认实现");
    } //默认实现 -- 没有[ ; ] ,因为不是签名
}

pub trait Foo {
    fn see(&self);
}
impl Foo for User {
    fn see(&self) {
        println!("foo 的 fee");
    }
}

//实现 trait : 1.使用宏: #[derive(TraitName)]
//2.使用impl - for
impl Run for User {
    const MAX_AGE: i32 = 100;
    //重载默认实现
    const MIN_AGE: i32 = 1;

    type RunType = i32;

    fn print(&self) {
        println!("hhhhh");
        //如何达到super的效果:运用多态 ,进行Self(注意不是self-->Self表示当前类型)的类型强转
        <Self as Run>::say(22);
    }
    //重载默认实现
    fn say(code: <Self as Run>::RunType) {
        //<Self as Run>  ===  <User as Run>
        //Self               ===   User
        println!("你好✌️");
    }
}

#[test]
pub fn use_method() {
    let mut user1 = User::new(
        String::from("jiodio"),
        18,
        true,
        Some(String::from("newyork")),
    );
    println!("{:?}", &user1.get_age());
    user1.change_age(19);
    println!("{:?}", &user1.get_age());
}

//Trait的多态表现  -- rust书上叫做 trait bound

//单限制
pub fn run_trait(stu: impl Run) {
    //直接调用实例函数
    stu.print();
    //调用静态函数直接实现类型转换就行了,里外的调用都是一样的
    <User as Run>::say(12); //<xx as yy> : 完全限定语法
}
//多限制
pub fn ru_foo_trait(stu: impl Run + Foo) {
    stu.see()
}

#[test]
pub fn at_run() {
    run_trait(User {
        ..Default::default()
    });
    ru_foo_trait(User {
        ..Default::default()
    })
}

#[test]
pub fn use_trait() {
    let new_user = User {
        name: "todo!()".to_string(),
        age: 18,
        sex: true,
        address: Some("china".to_string()),
    };
    new_user.print();
    //没有带self的函数在这里
    User::say(12);
}

/** ### 孤儿规则(orphan rule)
如果Astruct和Btrait都在本地crate外面,那么无法为Astruct实现Btrait  -  譬如无法为Vec<T> 实现Dispaly trait
换言之就是struct或trait必须得一个在本地才能为其实现trait
但是newtype 模式能解除孤儿模式的限制
newtype模式的实现是创建一个tuple struct将外部struct包裹进来成为内部数据,然后在实现上只要实现tuple struct,然后内部调用具体实现即可
---> 核心思想是使用tuple包一层 ,从而创建一个包含oldtype的newtype
 */

pub struct NewType(Vec<String>); //先定义一个包裹Vec的tuple struct
impl std::fmt::Display for NewType {
    //为NewType实现Display
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<<{}>>", self.0.join(", "))
    }
}
#[test]
fn use_newtypeI() {
    println!(
        "uadb={}",
        NewType(vec![String::from("1"), String::from("213")])
    )
}
