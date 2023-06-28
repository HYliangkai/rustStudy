/** ## 宏
rust的宏相当于在编译期根据根据宏的规则进行ast装换,相当于说内置了元编程的能力,并且这个能力和一般代码是相互分开的,会使用效果很强大,能减少很多的编码时间
### 宏的分类
1. 从生效的位置
+ 自定义 #[derive] 宏在结构体和枚举上指定通过 derive 属性添加的代码
+ 类属性（Attribute-like）宏定义可用于任意项的自定义属性
+ 类函数宏看起来像函数不过作用于作为参数传递的 token

2.从语法层面
+ 过程宏
+ 声明式宏
*/
pub mod declarativeMacros;
pub mod proceduralMacros;
