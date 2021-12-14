mod part1;
mod part2;
mod utils;

fn main() {
    let graph = utils::get_input("input");

    println!("Part 1: {}", part1::solution(&graph));
    println!("Part 2: {}", part2::solution(&graph));
}
