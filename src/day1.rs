pub fn day1() {
    // read a file
    let mut sums: Vec<i32> =
        include_str!("/Users/jakelanders/code/advent-of-code-2022/src/lib/day1.txt")
            .split("\n\n")
            .map(|b| b.lines().map(|i| i.parse::<i32>().unwrap()).sum())
            .collect();

    sums.sort();
    sums.reverse();
    sums.truncate(3);
    let sum: i32 = sums.iter().sum();

    println!("The largest amount of calories found was: {:?}", sums[0]);
    println!(
        "The top 3 largest amounts are: {:?}, with a sum of: {:?}",
        sums, sum
    );
}
