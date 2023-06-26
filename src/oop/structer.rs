/**
定义结构体,结构体只存储属性,而对应的方法需要通过impl来获得
 */

//1.基本的struct
#[derive(Default)] //为其实现default-trait ;可在赋值的时候实现默认赋值
#[derive(Debug)] //实现Debug-trait,可以在控制台输出
struct SutOne {
    name: String,
    age: i32,
    address: String,
}

//2.元组struct--用途:为元组起名,并且使得可以批量复制
struct SutTuple(i32, i32); //注意元组struct要以;结尾

//3.空struct--相当于空元组struct  --  用途:只想实现trait但是不想存储值时使用,注意也要;结尾
struct StuNull;

#[test]
pub fn use_struct() {
    let one = SutOne {
        name: String::from("jiojio"),
        ..Default::default()
    };
    println!("{:?}", one);
    //同时支持js的赋值简写,由此可见struct就是js的obj
    let name = String::from("dio");
    let two = SutOne {
        name,
        ..Default::default()
    };
    //但是和js的解构赋值不同,rust是"先赋值先占用",而js是"后赋值后占用"
    let three = SutOne {
        name: "dioda".to_string(),
        ..two
    };

    let four = SutTuple(0, 12);
}
