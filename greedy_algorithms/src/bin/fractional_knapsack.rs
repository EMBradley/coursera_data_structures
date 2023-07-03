/*
Maximizing the Value of the Loot Problem ---
    Find the maximal value of items that fit into the backpack.
        Input: The capacity of a backpack W as well as the weights
        (w1, . . . , wn) and costs (c1, . . . , cn) of n different compounds.
        Output: The maximum total value of fractions of items that fit
        into the backpack of the given capacity: i.e., the maximum value
        of c1 · f1 + · · · + cn · fn such that w1 ·f1 +· · ·+wn ·fn ≤ W and
        0 ≤ fi ≤ 1 for all i (fi is the fraction of the i-th item taken to the backpack).
*/
use std::io;

fn main() {
    let mut first_line = String::new();
    io::stdin()
        .read_line(&mut first_line)
        .expect("failed to read line");
    let mut first_line = first_line
        .trim()
        .split(' ')
        .map(|x| -> u32 { x.trim().parse().expect("failed to parse input") });
    let number_of_items = first_line.next().unwrap();
    let knapsack_capacity = first_line.next().unwrap();
    let items: Vec<(u32, u32)> = (0..number_of_items)
        .map(|_| {
            let mut line = String::new();
            io::stdin()
                .read_line(&mut line)
                .expect("failed to read line");
            let mut line = line
                .trim()
                .split(' ')
                .map(|x| -> u32 { x.trim().parse().expect("failed to parse input") });
            let c = line.next().unwrap();
            let w = line.next().unwrap();
            (c, w)
        })
        .collect();
    println!("{:.4}", fractional_knapsack(knapsack_capacity, items));
}

fn fractional_knapsack(knapsack_capacity: u32, items: Vec<(u32, u32)>) -> f64 {
    let mut items = items;
    items.sort_by(|(cost_a, weight_a), (cost_b, weight_b)| {
        // Compares the cost/weight of a and b, using cross-multiplication to avoid division
        (cost_a * weight_b).cmp(&(cost_b * weight_a))
    });
    let mut best_haul = 0;
    let mut remaining_capacity = knapsack_capacity;
    while remaining_capacity > 0 {
        if let Some((cost, weight)) = items.pop() {
            if weight < remaining_capacity {
                remaining_capacity -= weight;
                best_haul += cost;
            } else {
                return f64::from(best_haul)
                    + f64::from(remaining_capacity) * f64::from(cost) / f64::from(weight);
            }
        } else {
            return f64::from(best_haul);
        }
    }
    return 0.0;
}

#[cfg(test)]
mod tests {
    use super::fractional_knapsack;

    #[test]
    fn example_1() {
        let capacity = 50;
        let items = vec![(60, 20), (100, 50), (120, 30)];
        let correct_answer = "180.0000";
        let computed_answer = format!("{:.4}", fractional_knapsack(capacity, items));
        assert_eq!(correct_answer, computed_answer);
    }
    #[test]
    fn example_2() {
        let capacity = 10;
        let items = vec![(500, 30)];
        let correct_answer = "166.6667";
        let computed_answer = format!("{:.4}", fractional_knapsack(capacity, items));
        assert_eq!(correct_answer, computed_answer);
    }
    #[test]
    fn dude_wheres_my_knapsack() {
        let capacity = 0;
        let items = vec![(60, 20), (100, 50), (120, 30)];
        let correct_answer = "0.0000";
        let computed_answer = format!("{:.4}", fractional_knapsack(capacity, items));
        assert_eq!(correct_answer, computed_answer);
    }
    #[test]
    fn no_items() {
        let capacity = 50;
        let items = vec![];
        let correct_answer = "0.0000";
        let computed_answer = format!("{:.4}", fractional_knapsack(capacity, items));
        assert_eq!(correct_answer, computed_answer);
    }
}
