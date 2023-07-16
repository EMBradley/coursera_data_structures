use std::{
    cmp::{min, Ordering},
    fmt, io,
    ops::{Add, Mul, Sub},
};

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut buffer = String::new();

    let n = {
        stdin.read_line(&mut buffer)?;
        buffer.trim().parse().expect("failed to read n")
    };
    let mut points: Vec<_> = (0..n)
        .map(|_| {
            buffer.clear();
            stdin.read_line(&mut buffer).expect("failed to read point");
            let coordinates: Vec<f64> = buffer
                .split(' ')
                .map(|s| s.trim().parse().expect("failed to parse point"))
                .collect();
            Point::from(&coordinates)
        })
        .collect();

    let d = Point::closest_points(&mut points);
    println!("{d:.4}");
    Ok(())
}

#[derive(Clone, Copy, PartialEq)]
struct Real(f64);

impl Real {
    fn new<T: Copy + Into<f64>>(x: T) -> Self {
        let x: f64 = x.into();
        assert!(!x.is_nan());
        Self(x)
    }
    fn sqrt(self) -> Self {
        let Self(x) = self;
        Self(x.sqrt())
    }
    fn abs(self) -> Self {
        let Self(x) = self;
        Self(x.abs())
    }
}
impl PartialOrd for Real {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let Self(x) = self;
        let Self(y) = other;
        x.partial_cmp(y)
    }
}
impl Ord for Real {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other)
            .expect("coordinates must contain real numbers")
    }
}
impl Eq for Real {}
impl Sub for Real {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        let Self(x) = self;
        let Self(y) = other;
        Self(x - y)
    }
}
impl Mul for Real {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        let Self(x) = self;
        let Self(y) = other;
        Self(x * y)
    }
}
impl Add for Real {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        let Self(x) = self;
        let Self(y) = other;
        Self(x + y)
    }
}
impl fmt::Display for Real {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self(x) = self;
        write!(f, "{x}")
    }
}

#[derive(Clone, Copy)]
struct Point {
    x: Real,
    y: Real,
}

impl Point {
    fn new<T, U>(x: T, y: U) -> Self
    where
        T: Copy + Into<f64>,
        U: Copy + Into<f64>,
    {
        Self {
            x: Real::new(x),
            y: Real::new(y),
        }
    }
    fn from<T: Copy + Into<f64>>(coordinates: &[T]) -> Self {
        assert!(coordinates.len() == 2);
        let x = coordinates[0];
        let y = coordinates[1];
        Self::new(x, y)
    }
    fn distance(self, other: Self) -> Real {
        let delta_x = self.x - other.x;
        let delta_y = self.y - other.y;
        ((delta_x * delta_x) + (delta_y * delta_y)).sqrt()
    }
    fn closest_points(points: &mut [Self]) -> Real {
        fn closest_points_impl(points: &[Point]) -> Real {
            assert!(points.len() >= 2);
            if points.len() == 2 {
                let p = points[0];
                let q = points[1];
                return p.distance(q);
            }
            if points.len() == 3 {
                return points
                    .iter()
                    .enumerate()
                    .filter_map(|(i, p)| {
                        points
                            .iter()
                            .skip(i + 1)
                            .copied()
                            .map(|q| p.distance(q))
                            .min()
                    })
                    .min()
                    .unwrap();
            }

            let (left_side, right_side) = points.split_at(points.len() / 2);
            let d_left = closest_points_impl(left_side);
            let d_right = closest_points_impl(right_side);
            let d_side = min(d_left, d_right);

            let mid_line = points[points.len() / 2].x;
            let mut middle_strip: Vec<_> = points
                .iter()
                .copied()
                .filter(|p| (p.x - mid_line).abs() <= d_side)
                .collect();
            middle_strip.sort_unstable_by_key(|p| p.y);
            let d_mid = middle_strip
                .iter()
                .enumerate()
                .filter_map(|(i, p)| {
                    middle_strip
                        .iter()
                        .skip(i + 1)
                        .take(7)
                        .copied()
                        .map(|q| p.distance(q))
                        .min()
                })
                .min();

            match d_mid {
                Some(d_mid) => min(d_side, d_mid),
                None => d_side,
            }
        }

        points.sort_unstable_by_key(|p| p.x);
        closest_points_impl(points)
    }
}

#[cfg(test)]
mod points_tests {
    use rand::prelude::*;
    use std::{array, f64::consts::SQRT_2};

    use super::*;

    fn naive_solution(points: &[Point]) -> Real {
        points
            .iter()
            .enumerate()
            .filter_map(|(i, p)| {
                points
                    .iter()
                    .skip(i + 1)
                    .copied()
                    .map(|q| p.distance(q))
                    .min()
            })
            .min()
            .unwrap()
    }

    #[test]
    fn example_1() {
        let mut points = [Point::from(&[0, 0]), Point::from(&[3, 4])];
        let Real(d) = Point::closest_points(&mut points);
        assert!((d - 5.0).abs() <= 0.001)
    }
    #[test]
    fn example_2() {
        let mut points = [
            Point::from(&[4, 4]),
            Point::from(&[-2, -2]),
            Point::from(&[-3, -4]),
            Point::from(&[-1, 3]),
            Point::from(&[2, 3]),
            Point::from(&[-4, 0]),
            Point::from(&[1, 1]),
            Point::from(&[-1, -1]),
            Point::from(&[3, -1]),
            Point::from(&[-4, 2]),
            Point::from(&[-2, 4]),
        ];
        let Real(d) = Point::closest_points(&mut points);
        assert!((d - SQRT_2).abs() <= 0.001)
    }
    #[test]
    fn stress_test() {
        let mut rng = thread_rng();

        for _ in 0..512 {
            let mut points: [Point; 128] = array::from_fn(|_| {
                let x: f64 = rng.gen();
                let y: f64 = rng.gen();
                Point::new(x, y)
            });

            let Real(naive_answer) = naive_solution(&points);
            let Real(smart_answer) = Point::closest_points(&mut points);

            assert!((naive_answer - smart_answer).abs() <= 0.001);
        }
    }
}
