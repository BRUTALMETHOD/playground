use rand::prelude::*;
struct Solution {}

impl Solution {
    fn perform_quick_sort(input: &mut Vec<i32>, low: i32, high: i32) -> () {
        // where low = lower bound, and high = upper bound
        // transverse through lower to upper bound,
        if low < high {
            // get the partitioning index
            let pi = Self::perform_quick_sort_partition(input, low, high);
            // sort element before and after the partition
            Self::perform_quick_sort(input, low, pi - 1);
            Self::perform_quick_sort(input, pi + 1, high);
        }
    }
    fn perform_swap(input: &mut Vec<i32>, i: i32, j: i32) {
        // swap input[i] against input[j]
        input.swap(i as usize, j as usize);
    }
    fn perform_quick_sort_partition(input: &mut Vec<i32>, low: i32, high: i32) -> i32 {
        //Choosing the pivot
        let pivot = input[high as usize];
        // all swaps will be to the left of the pivot
        // if there is no increment in i, it means that
        // all numbers are greater than pivot
        // i will start at '0'
        let mut i = low;
        // dont include pivot as swap
        for j in low..high {
            if input[j as usize] < pivot {
                Self::perform_swap(input, i, j);
                i = i + 1;
            }
        }
        Self::perform_swap(input, i, high);
        i
    }
    // entry to the loop
    pub fn quick_sort(&self, mut input: Vec<i32>) -> Vec<i32> {
        if input.len() == 0 {
            return vec![];
        }
        let input_length = input.len() - 1;
        Self::perform_quick_sort(&mut input, 0, input_length as i32);
        input.to_vec()
    }
    fn perform_merge_sort(input: Vec<i32>) -> Vec<i32> {
        // split into 2

        let m = input.len() / 2;
        println!("{:?}", m);
        let mut return_array = Vec::new();
        if m > 1 {
            let first_half: Vec<i32> = input[0..m].into();
            let second_half: Vec<i32> = input[m + 1..input.len()].into();
            let sorted_first = Self::perform_merge_sort(first_half);
            let sorted_second = Self::perform_merge_sort(second_half);
            let mut i = 0;
            let mut j = 0;
            while i < sorted_first.len() && j < sorted_second.len() {
                if sorted_first[i] < sorted_first[j] {
                    return_array.push(sorted_first[i]);
                    i = i + 1;
                } else {
                    return_array.push(sorted_second[j]);
                    j = j + 1;
                }
            }
        }
        return_array
    }
    pub fn merge_sort(&self, mut input: Vec<i32>) -> Vec<i32> {
        // functions here
        // merge sort works by spliting 0 to m , m+1 to r
        //
        Self::perform_merge_sort(input).to_vec()
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
    // #for i in (0..30).collect::<Vec<i32>>() {
    //     let ans = solution.quick_sort(generate_random_vector(i));
    //     println!("output: {:?}", ans);
    // }
    // let ans = solution.quick_sort(generate_random_vector(10));
    // println!("output: {:?}", ans);
    let ans = solution.merge_sort(generate_random_vector(10));
    println!("output: {:?}", ans)
}
