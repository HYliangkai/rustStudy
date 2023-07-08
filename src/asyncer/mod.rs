/**  ## 异步
rust只提供异步语法 async / await 的使用,具体的实现依赖社区runtime来实现
rust中能产生异步代的是Future特性,本质是个状态机---类似于js的Promise,也是一个状态机
不同于js的Promise是自动推动的(就是说一但声明就立即开始运作) / Future是惰性的,需要每次都来轮询状态才能推动
*/
pub mod feat;
