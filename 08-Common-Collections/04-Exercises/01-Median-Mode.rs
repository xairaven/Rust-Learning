/*
Given a list of integers, use a vector and return the median
(when sorted, the value in the middle position)
and mode (the value that occurs most often;
a hash map will be helpful here) of the list.
 */

use std::collections::HashMap;

fn main() {
    let v: Vec<u32> = vec![1, 5, 7, 2, 3, 7, 7, 9, 2, 4, 4, 11];
    // Sorted: 1 2 2 3 4 4 5 7 7 9 11, Len: 11

    let median = median(&v);
    println!("Median: {median}");

    let mode = mode(&v);
    println!("Mode: {mode}");
}

fn median(vector: &Vec<u32>) -> u32 {
    let mut new_vector = vector.to_vec();
    new_vector.sort();

    let len = new_vector.len();
    if len == 0 {
        return 0;
    }

    if len % 2 == 1 {
        let result_index = len / 2 + 1;
        new_vector[result_index - 1]
    } else {
        let result_index = len / 2;
        new_vector[result_index - 1]
    }
}

fn mode(vector: &Vec<u32>) -> u32 {
    if vector.len() == 0 {
        return 0;
    }

    let mut occurrences: HashMap<u32, i32> = HashMap::new();

    for num in vector {
        let count = occurrences.entry(*num).or_insert(0);
        *count += 1;
    }

    let mut max_entries = 0;
    let mut mode = vector[0];
    for (number, number_of_entries) in &occurrences {
        if *number_of_entries >= max_entries {
            mode = *number;
            max_entries = *number_of_entries;
        }
    }

    mode
}