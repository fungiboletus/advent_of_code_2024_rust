use paste::paste;
use std::fmt::Display;

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;
mod day_10;
mod day_11;
mod day_12;
mod day_13;
mod day_14;
mod day_15;
mod day_16;
mod day_17;
mod day_18;
mod day_19;
mod day_20;
mod day_21;
mod day_22;
mod day_23;
mod day_24;
mod day_25;

fn execute_day<F, G, D1, D2>(day: &str, data: &str, part_1: F, part_2: G)
where
    F: Fn(&str) -> D1,
    G: Fn(&str) -> D2,
    D1: Display,
    D2: Display,
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
    execute_days!("01");
    execute_days!("02", "03", "04", "05", "06", "07", "08");
    execute_days!("09", "10", "11", "12", "13", "14", "15");
    execute_days!("16", "17", "18", "19", "20", "21", "22");
    execute_days!("23", "24", "25");
}
