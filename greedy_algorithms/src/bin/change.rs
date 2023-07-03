/*
Money Change Problem ---
    Compute the minimum number of coins needed
    to change the given value into coins with de-
    nominations 1, 5, and 10.
*/
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read input");
    let money = input.trim().parse().expect("failed to parse input");
    println!("{}", change(money));
}

fn change(money: u16) -> u16 {
    (money / 10) + ((money % 10) / 5) + (money % 5)
}

#[cfg(test)]
mod tests {
    use super::change;

    #[test]
    fn example_1() {
        assert_eq!(change(2), 2)
    }
    #[test]
    fn example_2() {
        assert_eq!(change(28), 6)
    }
}
