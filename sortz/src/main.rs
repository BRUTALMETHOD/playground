use rand::prelude::*;
use sortz::solution;
use std::time::SystemTime;

fn generate_random_vector(c: i32) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    let mut r: Vec<i32> = (1..c).collect();
    r.shuffle(&mut rng);
    r
}

fn main() {
    let solution = solution::Solution {};
    // #for i in (0..30).collect::<Vec<i32>>() {
    //     let ans = solution.quick_sort(generate_random_vector(i));
    //     println!("output: {:?}", ans);
    // }
    // let ans = solution.quick_sort(generate_random_vector(10));
    // println!("output: {:?}", ans);
    let start = SystemTime::now();
    let _ans = solution.merge_sort(generate_random_vector(100));
    let since = start.elapsed().expect("Time rollbacked").as_micros();
    println!("output: {:?}", _ans);
    println!("Completed in {} micro sec", since);
}
