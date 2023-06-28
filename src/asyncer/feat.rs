use std::pin::Pin;

use futures::{executor::block_on, FutureExt};

/**
+ Future状态机返回两种状态 :
    Pending 和 Ready ,由于可以使用Result<>表示then和catch,所以只需要用两种状态表示异步的完成与否
+
 */
async fn craete_other_func() {
    println!("异步驱动二")
}
async fn create_async_func() {
    //async 代码块会实现一个匿名的 Future trait object ，包裹一个 Generator。也就是一个实现了 Future 的
    //Generator。Generator实际上是一个状态机，配合.await当每次async 代码块中任何返回 Poll::Pending则即调
    //用generator yeild，让出执行权，一旦恢复执行，generator resume 继续执行剩余流程，当所有代码执行完，
    //就是状态机进入Complete，返回Poll::Ready，代表Future执行完毕。

    println!("异步驱动");
    //最外层的执行器执行并不能驱动里层的异步函数  , 需要声明 .await 才能驱动内层异步函数执行
    //所以rust的异步和js的异步一个最大的区别就是rust的异步是惰性的,必须用执行器驱动才能执行,(意思就是return一个Future要执行器驱动才能执行)而js的异步是非惰性的,只要new 一个 Promise就会执行
    //因此 js的await和rust的await原理是不一致的:  rust的await表示上层执行器先执行await的内容  |  js的await表示将代码延至await后处  ==> 但是rust是必须await才能执行(没有await就没有执行器驱动) js不用await也会自动去执行

    //实际上每一个.await本身就像一个执行器，在循环中查询Future的状态。如果返回Pending，则 yield，否则退出循环，结束当前Future。
    craete_other_func().await;
    let async_block = async { println!("async 代码块和async函数的效果一致") };
    async_block.await;

    //所以在async块里面写的代码逻辑上是同步的,但是实际驱动的是不同步的

    //除了使用 .await 执行,第二种办法是使用轮询(poll)来执行代码
    //轮询的原理就是制造一个loop然后主动驱动异步执行,直到获取结果,而驱动的片数是根据时间片进行划分?

    //使用 poll()宏来驱动异步执行
    //为了轮询 future，future 首先要用特殊类型 Pin<T> 来固定。
    //Pin<T>类型的数据表示在内存地址上不会移动的数据,否则轮询一圈后数据在内存上移动了,就抓不到进行轮询了
    let box_pin = Box::pin(get_book()); //使用Box::pin()能手动创建Pin<Box<T>>类型的数据
    let mut p = 11;
    let origin_pin = Pin::new(&mut p); //或是Pin::new()创建简易类型的pin
    let a = futures::poll!(box_pin);
}

//下面介绍future的方法
async fn get_book() -> i32 {
    println!("获得书本");
    return 23;
}
async fn get_music() -> i32 {
    println!("获取音乐");
    return 32;
}
//
async fn future_method() -> (i32, i32) {
    //select --like--> goland::select : 像是go的select语句,根据异步进行模式匹配,由于需要不断轮询获取到结果,所以一般要搭配runtime使用。
    loop {
        futures::select! {
            //使用fuse()函数将Future包装成新的Future(相当于复制一个future),并且不允许再调用poll方法
            res=get_book().fuse()=>{
            println!("先执行完获取书本");
            break;
        },
            res=get_music().fuse() =>{
            println!("先执行完获取音");
            break;
            }
        }
    }

    //join  --like--> promise.all  : 返回的数据是存储各个数据的元组
    futures::join!(get_book(), get_music())
}

#[test]
fn run_async() {
    //Future需要在一个执行器( executor )上运行，Future提供了几种不同的执行器,比如block_on 就是一个可以阻塞当前线程的执行器<那异步有啥用>
    // block_on(create_async_func());
    block_on(future_method());
}
