use std::io;

fn main() {
    let mut total_distance = String::new();
    io::stdin()
        .read_line(&mut total_distance)
        .expect("failed to read distance");
    let total_distance = total_distance
        .trim()
        .parse()
        .expect("failed to parse distance");
    let mut full_tank_miles = String::new();
    io::stdin()
        .read_line(&mut full_tank_miles)
        .expect("failed to read mileage");
    let full_tank_miles = full_tank_miles
        .trim()
        .parse()
        .expect("failed to parse mileage");
    let mut _number_of_gas_stations = String::new();
    io::stdin()
        .read_line(&mut _number_of_gas_stations)
        .expect("failed to read number of gas stations");
    let mut gas_station_distances = String::new();
    io::stdin()
        .read_line(&mut gas_station_distances)
        .expect("failed to read gas station distances");
    let gas_station_distances: Box<[u16]> = gas_station_distances
        .trim()
        .split(' ')
        .map(|d| -> u16 { d.parse().expect("failed to parse gas station distances") })
        .collect();
    if let Some(answer) = car_fueling(total_distance, full_tank_miles, &gas_station_distances) {
        println!("{}", answer);
    } else {
        println!("-1");
    }
}

fn car_fueling(
    total_distance: u16,
    full_tank_miles: u16,
    gas_station_distances: &[u16],
) -> Option<u16> {
    let mut stops = 0;
    let mut distance_so_far = 0;
    let mut miles_before_empty = full_tank_miles;
    let distances = [gas_station_distances, &[total_distance]].concat();

    for stop_distance in distances {
        let distance_to_next_stop = stop_distance - distance_so_far;
        if distance_to_next_stop > full_tank_miles {
            return None;
        }
        if distance_to_next_stop > miles_before_empty {
            stops += 1;
            miles_before_empty = full_tank_miles;
        }
        miles_before_empty -= distance_to_next_stop;
        distance_so_far += distance_to_next_stop;
    }
    return Some(stops);
}

#[cfg(test)]
mod tests {
    use super::car_fueling;

    #[test]
    fn example_1() {
        let total_distance = 950;
        let full_tank_miles = 400;
        let gas_station_distances = [200, 375, 550, 750];
        assert_eq!(
            car_fueling(total_distance, full_tank_miles, &gas_station_distances),
            Some(2)
        )
    }
    #[test]
    fn example_2() {
        let total_distance = 10;
        let full_tank_miles = 3;
        let gas_station_distances = [1, 2, 5, 9];
        assert_eq!(
            car_fueling(total_distance, full_tank_miles, &gas_station_distances),
            None
        )
    }
    #[test]
    fn example_3() {
        let total_distance = 200;
        let full_tank_miles = 250;
        let gas_station_distances = [100, 150];
        assert_eq!(
            car_fueling(total_distance, full_tank_miles, &gas_station_distances),
            Some(0)
        )
    }
    #[test]
    fn example_4() {
        let total_distance = 6;
        let full_tank_miles = 3;
        let gas_station_distances = [2, 3, 4];
        assert_eq!(
            car_fueling(total_distance, full_tank_miles, &gas_station_distances),
            Some(1)
        )
    }
    #[test]
    fn example_5() {
        let total_distance = 7;
        let full_tank_miles = 3;
        let gas_station_distances = [1, 2, 3, 4, 5, 6];
        assert_eq!(
            car_fueling(total_distance, full_tank_miles, &gas_station_distances),
            Some(2)
        )
    }
    #[test]
    fn example_6() {
        let total_distance = 7;
        let full_tank_miles = 3;
        let gas_station_distances = [1, 2, 6];
        assert_eq!(
            car_fueling(total_distance, full_tank_miles, &gas_station_distances),
            None
        )
    }
    #[test]
    fn example_7() {
        let total_distance = 12;
        let full_tank_miles = 3;
        let gas_station_distances = [1, 2, 4, 5, 6, 7, 8, 10, 11];
        assert_eq!(
            car_fueling(total_distance, full_tank_miles, &gas_station_distances),
            Some(4)
        )
    }
    #[test]
    fn example_8() {
        let total_distance = 12;
        let full_tank_miles = 3;
        let gas_station_distances = [1, 2, 4, 5, 8, 9, 10, 11];
        assert_eq!(
            car_fueling(total_distance, full_tank_miles, &gas_station_distances),
            Some(4)
        )
    }
}
