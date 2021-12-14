mod part1;
mod part2;
mod utils;

fn main() {
    let data = utils::get_input("input");
    println!("Part 1: {}", part1::solution(&data));
    println!("Part 1: {}", part2::solution(&data));
}
