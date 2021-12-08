use std::io;
use std::time::Instant;
use rand::Rng;

fn main() {
    println!("Please enter a number:");
    let mut input = String::new();
    let mut initial_list: Vec<i32> = Vec::new();
    let mut shuffled_list: Vec<i32> = Vec::new();
    io::stdin().read_line(&mut input).expect("error");
    let start = Instant::now();
    let range = input.trim().parse::<i32>().unwrap() + 1;
    for i in 1..range {
        initial_list.push(i);
    }
    println!("Initial list: {:?}", initial_list);
    let confirm_list = initial_list.clone();
    for _i in 0..initial_list.len() {
        let random_index = rand::thread_rng().gen_range(0..initial_list.len()).try_into().unwrap();
        let random_number = initial_list[random_index];
        initial_list.remove(random_index);
        shuffled_list.push(random_number);
    }
    println!("Shuffled list: {:?}", shuffled_list);
    let mut sorted_list = shuffled_list.clone();
    let mut limit = sorted_list.len() - 1;
    while confirm_list != sorted_list {
        for i in 0..limit {
            if sorted_list[i] > sorted_list[i + 1] {
                let hold = sorted_list.remove(i + 1);
                sorted_list.insert(i, hold);
            }
        }
        limit -= 1;
    }
    let duration = start.elapsed();
    println!("Sorted list: {:?}", sorted_list);
    println!("List sorted in {:?}", duration);
}