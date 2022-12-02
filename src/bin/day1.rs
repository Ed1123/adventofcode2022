fn main() {
    let input = include_str!("../input/day1.txt");
    let mut max_calories_elve: Vec<u32> = input
        .split("\n\n")
        .map(|elve_foods| elve_foods.split("\n").flat_map(|x| x.parse::<u32>()).sum())
        .collect();

    max_calories_elve.sort_by(|a, b| b.cmp(a));

    println!("Part one: {:?}", max_calories_elve[0]);
    println!(
        "Part two: {:?}",
        max_calories_elve.iter().take(3).sum::<u32>()
    );
}
