use rand::distr::Bernoulli;
use rand::prelude::*;
use rand::distr;
/* thoudsands:: Separable 是一个 trait,它提供了 separate_with_underscores() 方法,可以把数字格式化为带下划线的形式,提高可读性。*/
use thousands::Separable;
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
pub fn test_bernoulli(){
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
fn sub_part_bernoulli_fn_p(p: f64){
  match Bernoulli::new(p){
    Ok(b) => {
      println!("new({}) 概率为{:.3}",p, b.p());
    }
    Err(e) =>{
      println!("new({}) 失败，错误信息为{}", p, e);
    }
  }
}
pub fn test_bernoulli_fn_p(){
  // 测试 Bernoulli::p() 方法,它返回成功的概率 p。
  // 正数
  // 小数 (0..1)
  sub_part_bernoulli_fn_p(0.3);
  // 正数 [1,1]
  sub_part_bernoulli_fn_p(1.0);
  // 正数 (1..n)
  sub_part_bernoulli_fn_p(2.3);
  // 0
  sub_part_bernoulli_fn_p(0.0);
  // 负数
  // (-1,0)
  sub_part_bernoulli_fn_p(-0.3);
  // [-1,-1]
  sub_part_bernoulli_fn_p(-1.0);
  // (-∞, -1)
  sub_part_bernoulli_fn_p(-1.3);
}

#[
  doc = r#"
   测试 Bernoulli::from_ratio() 方法,它接受两个整数参数,分别表示成功和失败的次数,并返回一个 Bernoulli 分布实例。
   "#
]
#[allow(unused)]
pub fn test_bernoulli_fn_from_ratio(){
  // 测试 Bernoulli::from_ratio() 方法,它接受两个整数参数,分别表示成功和失败的次数,并返回一个 Bernoulli 分布实例。
  let temp = Bernoulli::from_ratio(3, 7).unwrap();
  let value = temp.p();
  println!( "值为{:.3} ",value);
}

#[
  doc = r#"
   测试 Bernoulli::sample() 方法,它从 Bernoulli 分布中采样,返回一个 bool 值,表示成功或失败。
   "#
]
#[allow(unused)]
pub fn test_bernoulli_sample(){
  use rand::distr::Distribution;
  let tmp = Bernoulli::new(0.1).unwrap();
  let mut rng = rand::rng();
  let result = tmp.sample(&mut rng);
  println!(" 结果为： {:?}",result)
}