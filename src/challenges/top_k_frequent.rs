// Leetcode 347
// Given an integer array nums and an integer k,
// return the k most frequent elements.
// You may return the answer in any order.

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

pub fn print_solve(){
    println!("top_k_frequent");
    let test_vec1 = vec![3, 1, 1, 1, 2, 2, 3];
    let test_vec2 = vec![1];
    let test_vec3 = vec![3, 3, 1, 1, 1, 1, 2, 4, 3, 2, 1, 5];
    println!("{:?}: {:?}", test_vec1, top_k_frequent_std_hashmap_heap(test_vec1.clone(), 2));
    println!("{:?}: {:?}", test_vec2, top_k_frequent_std_hashmap_heap(test_vec2.clone(), 1));
    println!("{:?}: {:?}", test_vec3, top_k_frequent_std_hashmap_heap(test_vec3.clone(), 2));

}

// The heap solution was prepared with the assistance of ChatGPT
// It results in a marginal improvement in memory usage.
// It does not result in a vector that is sorted numerically.
pub fn top_k_frequent_std_hashmap_heap(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let map = build_hash_map(nums);

    let mut heap = BinaryHeap::new();
    for (num, count) in map {
        heap.push(Frequency { count, num });
        if heap.len() > k as usize {
            heap.pop();
        }
    }

    heap.into_iter().map(|f| f.num).collect()
}

// The struct Frequency is used to implement Ord and PartialOrd
// This was provided by ChatGPT GPT-4
#[derive(Eq, PartialEq)]
struct Frequency {
    count: i32,
    num: i32,
}

impl Ord for Frequency {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .count
            .cmp(&self.count)
            .then_with(|| self.num.cmp(&other.num))
    }
}

impl PartialOrd for Frequency {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn top_k_frequent_std_hashmap(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let map = build_hash_map(nums);

    // intermediate vector to collect hashmap before sorting.
    // heap will be more memory efficient
    let sorted = intermediate_sorted_vector(map);

    // collect the first k elements of the sorted vector.
    let mut result = Vec::new();
    for i in 0..k as usize {
        result.push(sorted[i].0);
    }
    result
}

fn intermediate_sorted_vector(map: HashMap<i32, i32>) -> Vec<(i32, i32)> {
    // intermediate vector to collect hashmap before sorting.
    let mut v: Vec<(i32, i32)> = map.into_iter().collect();
    // sort the intermediate vector by the second element of the tuple.
    v.sort_by(|a, b| b.1.cmp(&a.1));
    v
}

fn build_hash_map(nums: Vec<i32>) -> HashMap<i32, i32> {
    let mut map = HashMap::new();
    for num in nums {
        //
        let count = map.entry(num).or_insert(0);
        *count += 1;
    }
    map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_top_k_frequent() {
        assert_eq!(
            top_k_frequent_std_hashmap(vec![1, 1, 1, 2, 2, 3], 2),
            vec![1, 2]
        );
        assert_eq!(top_k_frequent_std_hashmap(vec![1], 1), vec![1]);
    }

    #[test]
    fn test_top_k_frequent_heap() {
        assert_eq!(
            top_k_frequent_std_hashmap_heap(vec![1, 1, 1, 2, 2, 3], 2),
            vec![2,1]
        );
        assert_eq!(top_k_frequent_std_hashmap_heap(vec![1, 1, 2], 1), vec![1]);
    }
}
