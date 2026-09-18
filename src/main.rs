
// mod learn;
// mod learn_ownership;
// use learn_ownership::Node;
// /*
//     你在 main.rs 里直接 use learn_ownership::Node;,但没有先声明模块。必须先 mod,再 use:
//     顺序上 mod 声明放前面更清晰。
// */
// use std::time::Instant;
// mod process_user;
// use process_user::{User, process_user};
// mod nonetypeerr;
// use nonetypeerr::*;
// mod thread_do;
// #[allow(unused_imports)]
// use std::thread;
// use thread_do::cpu_work;

use rand::distr::Bernoulli;
use rand::prelude::*;
use rand::distr;
/* thoudsands:: Separable 是一个 trait,它提供了 separate_with_underscores() 方法,可以把数字格式化为带下划线的形式,提高可读性。*/
use thousands::Separable;
// use rand::distr::Distribution;
#[allow(unused)]
#[allow(unused_imports)]
use rand::rngs::*;
#[allow(unused)]
#[allow(unused_imports)]
use rand::seq::*;
#[doc = 
    r#"
    1. Bernoulli 分布是一个离散概率分布,它只有两个可能的结果:成功(通常表示为 1)和失败(通常表示为 0)。成功的概率为 p,失败的概率为 1-p。
    2. 在 Rust 中,可以使用 rand crate 提供的 distr::Bernoulli 来生成 Bernoulli 分布的随机样本。
    3. 使用 distr::Bernoulli::new(p) 创建一个 Bernoulli 分布实例,其中 p 是成功的概率。
    4. 然后可以使用 sample 方法从该分布中采样,返回一个 bool 值,表示成功或失败。
    5. 可以通过多次采样来估计成功的概率,并与理论值进行比较。
    "#
    ]
    #[allow(unused)]
fn test_bernoulli(){
  let mut count = 0;
  let mut count2 = 0;
  let mut count3 = 0;
  let bernoulli = distr::Bernoulli::new(0.3).unwrap();
  for _ in 0..1_000_000_000{
    count2 += 1;
    if count2 % 1_000_000 == 0{
      count3 += 1;
      println!("已完成 {} * 1_000_000 次实验", count3.separate_with_underscores());
      count2 = 0;
      
    }
    let value: bool = bernoulli.sample(&mut rand::rng());
    if value{
      count += 1;
    }
  }
  println!("\n在 1_000_000_000 次实验中,成功概率为 {} %", count as f64 / 1_000_000_000.0);
}

#[
  doc = r#"
   测试 Bernoulli::p() 方法,它返回成功的概率 p。

  "#
]
#[allow(unused)]
fn test_bernoulli_fn_p(){
  // 测试 Bernoulli::p() 方法,它返回成功的概率 p。
  let temp = Bernoulli::new(0.3).unwrap();
  let value = temp.p();
  println!( "值为{:.3} ",value);
}

#[allow(unused)]
fn test_bernoulli_fn_from_ratio(){
  // 测试 Bernoulli::from_ratio() 方法,它接受两个整数参数,分别表示成功和失败的次数,并返回一个 Bernoulli 分布实例。
  let temp = Bernoulli::from_ratio(3, 7).unwrap();
  let value = temp.p();
  println!( "值为{:.3} ",value);
}

fn main() {


  test_bernoulli_fn_from_ratio();
  // test_bernoulli();
  // test_bernoulli_fn_p();
  // let mut rng = rand::rng();
  // let num: Vec<u64> = (0..10).map(|_| rng.random_range(0..101)).collect();

  // for i in num {
  //   println!("输出rand::thread_rng().gen_range(1..101)的随机数: {}", i);
  // }

    /*
    let start = Instant::now();
    #[warn(unused_variables)]
    let _results: Vec<u64> = (0..10_000_000).map(|n| learn::fibonacci(n % 30)).collect();
    println!("Elapsed: {:.2?}", start.elapsed());

    let mut root = Node::new("root");
    let child = Node::new("child");
    root.add_child(child);

    let string = "12345"; // let string: str = "12345";   // ❌ 显式标注成 str 才错
    let _user: User = process_user(0xFFFF, string);

    let _user = find_user(999);

    match find_user(999) {
        Some(user) => println!("{}", user.name),
        None => println!("User not found"),
    }

    let _name = find_user(999)
        .map(|u| u.name)
        .unwrap_or_else(|| " Unknow".to_string());

    let start = std::time::Instant::now();
    let handless: Vec<_> = (0..4)
        .map(|_| thread::spawn(|| cpu_work(3_000_000)))
        .collect();
    let _results: Vec<u64> = handless.into_iter().map(|h| h.join().unwrap()).collect();
    println!("4 threads: {:.2?}", start.elapsed());*/
}

/* 错误代码原因的部分，详细内容可以查看deepseek。
    0xFFFF 在rust中不允许使用 值: 类型 这种方式传递参数值，
    可以使用的：
    0xFFFF
    0xFFFF_i64
    0xFFFF as i64

    ** 错误 6:"12345" 是 &str,不是 String
    ``` rust
    ``` let mut string = "12345";   // 类型:&str
    如果是 &str,&string 的类型是 &&str,虽然能 deref 成 &str,但不自然

    如果是 &str,&string 的类型是 &&str,虽然能 deref 成 &str,但不自然 是什么意思， 其次 "12345" 为什么 是 &str 而不是 str呢

    两个概念:自动解引用 与 str / &str
    问题一:&string 为什么是 &&str
    先看类型链
    rust
    let string: &str = "12345";   // string 的类型是 &str

    &string                        // 对 &str 取引用 → 得到 &&str
    &T 就是"对 T 取引用"。既然 string 本身已经是 &str,再取一次 &,类型自然变成 &&str。

    text
    string   : &str
    &string  : &&str
    那为什么还能传给 name: &str?
    因为 Rust 有一个机制叫 Deref 强制转换(Deref Coercion),更准确说是自动解引用。

    &&str 可以自动被转成 &str:

    text
    &&str  ──解一层──►  &str   ✅ 自动发生
    所以下面这段能编译:

    rust
    fn f(name: &str) {}

    let s: &str = "hi";
    f(&s);      // 传的是 &&str,自动解引用成 &str → 能编译
    f(s);       // 传的是 &str → 更直接
    为什么说"不自然"
    因为它能工作,但不是你想要的写法:

    f(&s) 多取了一次引用,编译器又要多解一次,虽然编译器能搞定,但语义上是"绕了一圈"

    读代码的人会疑惑:为什么要 &s?是不是 s 是 String?

    真正想表达的只是"把 s 传进去",直接 f(s) 更清晰

    关键区分:

    rust
    let a: String = String::from("hi");   // a 是 String
    let b: &str   = "hi";                 // b 是 &str

    f(&a);   // &String → deref 成 &str   ✅ 有意义,因为 a 不是引用
    f(&b);   // &&str   → deref 成 &str   ⚠️ 能编译但多余
    f(b);    // &str    → 直接匹配         ✅ 最自然
    &String 自动转 &str 是常见且推荐的(因为 String 本身不是引用);但 &&str 转 &str 是多余的一层,所以我说"不自然"。

    Deref 强制转换能解几层?
    Rust 会反复解引用直到类型匹配,所以理论上一串引用都能解:

    rust
    let s: &str = "hi";
    let r1 = &s;      // &&str
    let r2 = &&s;     // &&&str

    f(s);    // ✅
    f(r1);   // ✅ 自动解一层
    f(r2);   // ✅ 自动解两层(但要类型匹配得上)
    能编译,但没人会这么写。代码可读性很差。

    问题二:为什么 "12345" 是 &str 而不是 str
    根本原因:str 是"不定长类型"(unsized)
    str 在 Rust 里是没有固定大小的类型。它的长度只有在运行时才知道(取决于有多少个字符)。

    Rust 规定:所有变量在编译期必须有已知的大小(这样才能在栈上分配空间)。而 str 的大小是不定的,所以:

    rust
    let s: str = "12345";   // ❌ 编译错误
    // error: the size for values of type `str` cannot be known at compilation time
    那怎么表示字符串?
    用引用 + 长度。&str 是一个胖指针(fat pointer),包含两部分:

    text
    &str 在内存里:
    ┌─────────────┬──────────┐
    │  指针(8字节) │ 长度(8字节) │
    └─────────────┴──────────┘
        │
        ▼
    "12345" 存在程序的只读数据段
    指针:指向字符串第一个字节

    长度:告诉程序这个 str 有多长

    有了长度,str 的大小就"间接"确定了 —— 通过 &str 这个胖指针来描述。

    那为什么字面量偏偏是 &str?
    字符串字面量 "12345" 的数据存在程序的静态 / 只读数据段(编译期就确定):

    text
    .rodata 段:
    '1' '2' '3' '4' '5'
    ↑
    程序里所有 "12345" 都指向这里
    字面量本身是"一块不可变的字符串数据",类型是 str。但因为它大小不定,不能直接作为变量,所以 Rust 给你的是一个指向它的引用 &str。

    所以:

    rust
    let s = "12345";        // s: &str,指向只读数据段
    let t: &str = "12345";  // 显式写法,一样
    let owned = "12345".to_string();  // 拷贝到堆上,变成 String
    str 和 &str、String 三者关系
    类型	含义	大小	存放位置
    str	字符串数据本身(不定长)	编译期未知	任何地方
    &str	指向 str 的胖指针(指针 + 长度)	16 字节(64位系统)	栈
    String	拥有所有权的可增长字符串	24 字节(指针+长度+容量)	数据在堆上
    与 C++ 对照
    C++	Rust	说明
    const char*	&str	都是"指向字符串的引用"
    std::string	String	都拥有数据、可增长
    std::string_view	&str	都是"借用视图",不拥有
    C++ 没有"不定长类型"	str 就是不定长	Rust 用胖指针处理它
    回到你的代码
    rust
    let string = "12345";              // string: &str

    let user = process_user(0xFFFF, &string);   // 传 &&str → 自动 deref 成 &str ⚠️
    let user = process_user(0xFFFF, string);    // 传 &str  → 直接匹配     ✅ 推荐

    let string = String::from("12345");         // string: String
    let user = process_user(0xFFFF, &string);   // 传 &String → deref 成 &str ✅ 推荐
    记忆口诀:

    变量是 &str → 直接传,别加 &

    变量是 String → 加 &,让它借出去

    一句话总结
    &string 是 &&str:Rust 会自动解引用让它匹配 &str,能编译,但多了一层没意义的引用,写法不自然。

    "12345" 是 &str 不是 str:因为 str 是不定长类型,大小在编译期未知,不能直接作为变量存在;必须用 &str 这个"指针 + 长度"的胖指针来引用它。
*/

/* 错误代码及错误信息：
错误代码：
src/porcess_user.rs
pub fn process_user(user_id: i64, name: &str) -> User{
    User {
        id: user_id,
        name: name.to_uppercase(),
    }
}


#[derive(Deserialize)]

pub struct UserInput {
    id: i64,
    name: String,
}


src/main.rs
mod learn;
use learn_ownership::Node;
use std::time::Instant;
mod process_user;

fn main() {
    let start = Instant::now();
    #[warn(unused_variables)]
    let _results: Vec<u64> = (0..10_000_000).map(|n| learn::fibonacci(n % 30)).collect();
    println!("Elapsed: {:.2?}", start.elapsed());

    let mut root = Node::new("root");
    let child = Node::new("child");
    let result = root.add_child(child);
    println!("add_child() {:?}", result);

    let mut string = "12345";
    let user:User = porcess_user(0xFFFF: i64, &string);

}

错误信息：
error info
error: expected one of `)`, `,`, `.`, `?`, or an operator, found `:`
  --> src\main.rs:18:40
   |
18 |     let user:User = porcess_user(0xFFFF: i64, &string);
   |                                        ^ expected one of `)`, `,`, `.`, `?`, or an operator

error[E0432]: unresolved import `learn_ownership`
 --> src\main.rs:2:5
  |
2 | use learn_ownership::Node;
  |     ^^^^^^^^^^^^^^^ use of unresolved module or unlinked crate `learn_ownership`
  |
help: to make use of source file src\learn_ownership.rs, use `mod learn_ownership` in this file to declare the module
  |
1 + mod learn_ownership;
  |

error: cannot find derive macro `Deserialize` in this scope
 --> src\process_user.rs:9:10
  |
9 | #[derive(Deserialize)]
  |          ^^^^^^^^^^^

error[E0425]: cannot find type `User` in this scope
 --> src\process_user.rs:1:50
  |
1 | pub fn process_user(user_id: i64, name: &str) -> User{
  |                                                  ^^^^ not found in this scope
error[E0422]: cannot find struct, variant or union type `User` in this scope
 --> src\process_user.rs:2:5
  |
2 |     User {
  |     ^^^^ not found in this scope

error[E0425]: cannot find type `User` in this scope
  --> src\main.rs:18:14
   |
18 |     let user:User = porcess_user(0xFFFF: i64, &string);
   |              ^^^^ not found in this scope

Some errors have detailed explanations: E0422, E0425, E0432.
For more information about an error, try `rustc --explain E0422`.
error: could not compile `RUST` (bin "RUST") due to 6 previous errors
*/
