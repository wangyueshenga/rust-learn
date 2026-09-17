#[allow(unused_imports)]
use std::thread;
// use rayon::prelude::*;
pub fn cpu_work(n: u64) -> u64 {
    //let results: Vec<u64> = inputs.par_iter().map(|&n| cpu_work(n)).collect();
    
    (0..n).map( |i| i * i).sum()
}