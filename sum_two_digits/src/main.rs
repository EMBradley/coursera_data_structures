use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read line");
    let mut nums = Vec::new();
    for string in input.trim().split(' ') {
        let num: u32 = string.parse().expect("failed to parse string");
        nums.push(num);
    }
    let sum: u32 = nums.iter().sum();
    println!("{}", sum)
}
