use rand::distr::Bernoulli;
use rand::prelude::*;
use rand::distr;
/* thoudsands:: Separable 是一个 trait,它提供了 separate_with_underscores() 方法,可以把数字格式化为带下划线的形式,提高可读性。*/
use thousands::Separable;


#[
    doc = r#"
    # Crate rand
    ## mod distr
    - struct Alphabetic Sample a u8, uniformly distributed over letters: a-z and A-Z
    - struct Alphanumeric Sample a u8, uniformly distributed over letters and numbers: a-z, A-Z and 0-9
    - struct Bernoulli Sample a bool, with a given probability of being true
    - struct Iter An iterator over a Distribution, producing an infinite stream of samples
    - struct Map A Distribution which maps sampled values to type S
    - struct Open01 A distribution to sample floating point numbers uniformly in the open interval (0, 1), i.e. not including either endpoint.
    - struct OpenClosed01 A distribution to sample floating point numbers uniformly in the half-open interval (0, 1], i.e. including 1 but not 0.
    - struct StandardUniform The Standard Uniform distribution
    - struct Uniform Sample values uniformly between two bounds.
    - enum BernoulliError Error type returned from Bernoulli::new().
    ### mod slice
    - struct Choose A distribution to uniformly sample elements of a slice.
    - struct Empty Error:empty slice
    ### mod uniform
    - struct Uniform Sample values uniformly between two bounds.
    - struct UniformChar The back-end implementing UniformSampler for char.
    - struct UniformDuration The back-end implementing UniformSampler for Duration.
    - struct UniformFloat The back-end implementing UniformSampler for floating-point types.
    - struct UniformInt The back-end implementing UniformSampler for integer types.
    - struct UniformUsize 32  -bit or 64-bit The back-end implementing UniformSampler for usize.
    - error Error Error type returned from Uniform::new and new_inclusive
    ### mod weighted (alloc) alloc 代表这个类型会使用 堆内存。
    - struct WeightedIndex A distribution using weighted sampling of discrete items.
    - enum Error Invalid weight errors
    ## mod prelude
    - 用于引用常用的 trait 和类型,简化代码书写。
    ## mod rngs
    - struct ChaCha8Rngchacha A cryptographically secure random number generator that uses the ChaCha stream cipher.
    - struct ChaCha12Rngchacha A cryptographically secure random number generator that uses the ChaCha stream cipher.
    - struct ChaCha20Rngchacha A cryptographically secure random number generator that uses the ChaCha stream cipher.
    - struct SmallRng A small-state, fast, non-crypto, non-portable PRNG
    - struct StdRngstd_rng A strong, fast (amortized), non-portable RNG
    - struct SysErrorsys_rng A small and no_std compatible error type
    - struct SysRngsys_rng A TryRng interface over the system’s preferred random number source
    8. ThreadRngthread_rng A reference to the thread-local generator 
    9. Xoshiro128PlusPlus A xoshiro128++ random number generator. 
    10.Xoshiro256PlusPlus A xoshiro256++ random number generator.
    ## mod seq
    - struct IndexedSamples An iterator over multiple slice elements.
    - struct IndexedRandom for sampling slices and other indexable lists.
    - struct IndexedMutRandom for sampling slices and other mutably indexable lists.
    - struct SliceRandom for mutating slices.
    - struct IteratorRandom for sampling iterators.
    - struct index::sample low-level API to choose multiple indices from 0..length
    ### mod index
    IndexVecalloc
    - enum IndexVec (alloc) A vector of indices.
    - enum IndexVecIntoIteralloc (alloc) Return type of IndexVec::into_iter.
    - enum IndexVecIteralloc (alloc) Return type of IndexVec::iter.
    - function sample (alloc) Randomly sample exactly amount distinct indices from 0..length, and return them in random order (fully shuffled).
    - function sample_array Randomly sample exactly N distinct indices from 0..len, and return them in random order (fully shuffled).
    - function sample_weighted (alloc and std) Randomly sample amount distinct indices from 0..length
    "#
]

#[ doc = r#"
  - struct Alphabetic Sample a u8, uniformly distributed over letters: a-z and A-Z
"#
]
#[allow(unused)]
pub fn test_distr_alphabetic(){
  use rand::distr::{Alphabetic, SampleString};
  use rand::RngExt;

  let mut rng = rand::rng();
  let chars: String = (0..7).map(|_| rng.sample(Alphabetic) as char).collect();
  println!("Random chars: {}", chars);

  let string = Alphabetic.sample_string(&mut rand::rng(), 16);
  println!("Random string: {}", string);
}


#[ doc = r#"
  - struct Alphanumeric Sample a u8, uniformly distributed over letters and numbers: a-z, A-Z and 0-9
"#
]
#[allow(unused)]
pub fn test_alphanumeric(){
  use rand::RngExt;
  use rand::distr::{Alphanumeric, SampleString};
  let mut rng = rand::rng();
  let chars: String = (0..10).map(|_| rng.sample(Alphanumeric) as char).collect();
  println!("Random chars: {}", chars);
  
  let string = Alphanumeric.sample_string(&mut rand::rng(), 16);
  println!("Random string: {}", string);
}

#[allow(unused)]
pub fn test_distr_bernoulli(){
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

#[
  doc = r#"
   测试 Bernoulli::sample_iter() 方法,它返回一个迭代器,可以从 Bernoulli 分布中连续采样。
   "#
]
pub fn test_bernoulli_sample_iter(){
  use rand::distr::Distribution;
  let tmp = Bernoulli::new(0.1).unwrap();
  let mut rng = rand::rng();
  let result: Vec<bool> = tmp.sample_iter(&mut rng).take(10).collect();
  println!(" 结果为： {:?}",result)
}

