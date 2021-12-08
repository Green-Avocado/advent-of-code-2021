mod part1;
mod part2;
mod utils;

fn main() {
    let nums = utils::get_input("input");

    println!("Part 1: {}", part1::solution(&nums));
    println!("Part 2: {}", part2::solution(&nums));
}
