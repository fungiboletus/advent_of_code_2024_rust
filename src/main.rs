mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;

fn execute_day<F, G>(day: &str, data: &str, part_1: F, part_2: G)
where
    F: Fn(&str) -> i64,
    G: Fn(&str) -> i64,
{
    let now = std::time::Instant::now();
    println!("Day {}, part 1: {}", day, part_1(data));
    println!("Day {}, part 2: {}", day, part_2(data));
    println!("Time day {}: {:?}", day, now.elapsed());
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

use paste::paste;

macro_rules! execute_days {
    ($($day:literal),*) => {
        $(
            paste! {
                execute_day!(
                    $day,
                    [<day_ $day>]::[<day_ $day _part_1>],
                    [<day_ $day>]::[<day_ $day _part_2>]
                );
            }
        )*
    };
}

fn main() {
    execute_days!("01", "02", "03", "04", "05", "06", "07", "08", "09");
}
