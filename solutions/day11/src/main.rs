mod part1;
mod part2;
mod utils;

fn main() {
    let grid = utils::get_input("input");
    println!("Part 1: {}", part1::solution(&grid));
    println!("Part 2: {}", part2::solution(&grid));
}
