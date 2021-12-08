mod part1;
mod part2;
mod utils;

fn main() {
    println!("Part 1: {}", part1::solution(utils::get_input("input")));
    println!("Part 2: {}", part2::solution(utils::get_input("input")));
}
