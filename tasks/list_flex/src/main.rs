use std::collections::HashMap;


fn avg(nums: &Vec<i32>) -> Option<f64> {
    if nums.len() == 0 {
        return None;
    }
    let mut sum = 0;
    for n in nums {
        sum += n;
    }
    Some(sum as f64 / nums.len() as f64)
}

fn sort(nums: &Vec<i32>) -> Vec<i32>{
    let mut sorted_nums = nums.clone();
    if sorted_nums.len() == 0 {
        return sorted_nums;
    }
    let mut is_finish: bool = false;
    while !is_finish {
        is_finish = true;
        for i in 0..(sorted_nums.len() - 1) {
            if sorted_nums[i] > sorted_nums[i + 1] {
                let buff = sorted_nums[i + 1];
                sorted_nums[i + 1] = sorted_nums[i];
                sorted_nums[i] = buff;
                is_finish = false;
            }
        }
    }
    sorted_nums
}

fn med(nums: &Vec<i32>) -> Option<i32> {
    if nums.len() == 0 {
        return None;
    }
    let sorted_nums = sort(nums);
    let mid: i32;
    if nums.len() % 2 == 1 {
        mid = sorted_nums[(sorted_nums.len() - 1) / 2];
    } else {
        mid = (sorted_nums[sorted_nums.len() / 2 - 1] + sorted_nums[sorted_nums.len() / 2]) / 2;
    }
    Some(mid)
}

fn moda(nums: &Vec<i32>) -> Option<i32> {
    if nums.len() == 0 {
        return None;
    }
    let mut table = HashMap::new();
    for n in nums {
        let count = table.entry(n).or_insert(0);
        *count += 1;
    }
    let mut result = 0;
    let mut count = 0;
    for (key, data) in &table {
        if *data > count {
            count = *data;
            result = **key;
        }
    }
    Some(result)
}

fn main() {
    // let nums = vec![];
    let nums = vec![12, -1, -1, -1, 3, 9, -4, -1, 0, 1, 9, 100];
    match avg(&nums) {
        None => println!("Мало чисел((("),
        Some(avg) => println!("avg = {avg}"),
    };
    let sorted_nums = sort(&nums);
    println!("sorted vector = {:?}", sorted_nums);
    match med(&nums) {
        None => println!("Мало чисел((("),
        Some(med) => println!("med = {med}"),
    };
    match moda(&nums) {
        None => println!("Мало чисел((("),
        Some(moda) => println!("moda = {moda}"),
    };
}
