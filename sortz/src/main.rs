use rand::prelude::*;
struct Solution {}

impl Solution {
    pub fn quick_sort(&self, input: Vec<i32>) -> Vec<i32> {
        input
    }
}

fn generate_random_vector(c: i32) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    let mut r: Vec<i32> = (1..c).collect();
    r.shuffle(&mut rng);
    r
}

fn main() {
    let solution = Solution {};
    let ans = solution.quick_sort(generate_random_vector(10));
    println!("output: {:?}", ans);
}
