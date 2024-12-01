mod day_01;

fn execute_day<F, G>(day: &str, data: &str, part_1: F, part_2: G)
where
    F: Fn(&str) -> i64,
    G: Fn(&str) -> i64,
{
    let now = std::time::Instant::now();
    println!("Day {}, part 1: {}", day, part_1(data));
    println!("Day {}, part 2: {}", day, part_2(data));
    println!("Time: {:?}", now.elapsed());
}

macro_rules! execute_day {
    ($day:expr, $part_1:expr, $part_2:expr) => {
        execute_day(
            $day,
            include_str!(concat!("../inputs/day_", $day, ".txt")),
            $part_1,
            $part_2,
        );
    };
}

fn main() {
    execute_day!("01", day_01::day_1_part_1, day_01::day_1_part_2);
}
