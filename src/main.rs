use std::io;
use rand::Rng;

fn main() {
    println!("Please enter a number:");
    let mut input = String::new();
    let mut hold: i32;
    let mut initial_list: Vec<i32> = Vec::new();
    let mut new_list: Vec<i32> = Vec::new();
    io::stdin().read_line(&mut input);
    let range = input.trim().parse::<i32>().unwrap() + 1;
    for i in 1..range {
        initial_list.push(i);
    }
    println!("Initial list: {:?}", initial_list);
    for i in 0..initial_list.len() {
        let mut a = rand::thread_rng().gen_range(0..initial_list.len()).try_into().unwrap();
        let mut b = initial_list[a];
        initial_list.remove(a);
        new_list.push(b);
    }
    println!("Shuffled list: {:?}", new_list);

}