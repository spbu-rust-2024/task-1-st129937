use std::io;
use itertools::join;

fn main() {
    let mut line = String::new();

    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");

    let mut arr: Vec<i32> = line
        .trim()
        .split_whitespace()
        .filter_map(|x| x.parse::<i32>().ok())
        .collect();

    insert_sort(&mut arr);

    print!("{}", join(&arr, " "));

}


fn insert_sort(nums: &mut Vec<i32>){
    let length = nums.len();
    for i in 1..length{
        let el = nums[i];
        let mut j = i;
        while (j > 0) && (nums[j - 1] > el) {
            nums[j] = nums[j - 1];
            j -= 1;
        }
        nums[j] = el;
    }
}

