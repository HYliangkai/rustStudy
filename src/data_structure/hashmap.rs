use std::collections::HashMap;

/** ## hashmap
这东西说白了就是高级版的object,不能用点语法注定用起来就是费劲
 */
#[test]
pub fn use_hashmap() {
    //init
    //1.还是new
    let mut hm1: HashMap<&str, &str> = HashMap::new();
    //2.从一组Array<(key,value)>中快速转化
    let mut _hm2 = HashMap::from([(12, 13)]);

    //judge
    //通过entry(key)函数获取Entry-struct
    //在Rust中，Entry是HashMap和BTreeMap中的一个枚举类型，用于表示插入或获取元素时的结果。Entry有三个可能的值：
    //Vacant：表示插入操作成功，没有找到指定的键，成功插入了一个新的键值对。
    //Occupied：表示获取操作成功，找到了指定的键，并返回了对应的值。
    //VacantEntry：表示获取操作失败，没有找到指定的键。

    //add
    //方法1:覆盖插入
    hm1.insert("性别", "男");
    //方法2:当值不存在时插入
    hm1.entry("性别").or_insert("人妖");

    //query
    //方法1 get() : 从hm取得的值自带Option , 并且从hm中用get()方法取值不会失去值在hm中的所有权
    let sex = hm1.get("性别").unwrap();
    println!("{:?}", sex);

    //update
    //获取hm中的数据默认是引用数据,所以可以进行直接更改源数据
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);

    //update 由于
}

// std中除了hashmap(基于哈希表,无序),还有treemap(基于红黑树,有序)实现
