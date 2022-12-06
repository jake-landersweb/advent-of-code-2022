pub fn part2() {
    let coverage: Vec<i32> =
        include_str!("/Users/jakelanders/code/advent-of-code-2022/src/lib/day4.txt")
            .lines()
            .map(|x| {
                let pair: Vec<&str> = x.split(",").collect();
                let range1: Vec<u32> = pair[0]
                    .split("-")
                    .map(|i| i.parse::<u32>().unwrap())
                    .collect();
                let range2: Vec<u32> = pair[1]
                    .split("-")
                    .map(|i| i.parse::<u32>().unwrap())
                    .collect();

                // check for overlaps
                if range2[1] >= range1[1] && range2[0] <= range1[1] {
                    return 1;
                }

                if range1[1] >= range2[1] && range1[0] <= range2[1] {
                    return 1;
                }

                // base case of no
                return 0;
            })
            .collect();

    let sum: i32 = coverage.iter().sum();

    println!("The num of total coverages is: {:?}", sum);
}

pub fn part1() {
    let coverage: Vec<i32> =
        include_str!("/Users/jakelanders/code/advent-of-code-2022/src/lib/day4.txt")
            .lines()
            .map(|x| {
                let pair: Vec<&str> = x.split(",").collect();
                let range1: Vec<u32> = pair[0]
                    .split("-")
                    .map(|i| i.parse::<u32>().unwrap())
                    .collect();
                let range2: Vec<u32> = pair[1]
                    .split("-")
                    .map(|i| i.parse::<u32>().unwrap())
                    .collect();

                // check range1 covers range2
                if range1[0] <= range2[0] && range1[1] >= range2[1] {
                    return 1;
                }
                // check range2 covers range1
                if range2[0] <= range1[0] && range2[1] >= range1[1] {
                    return 1;
                }

                // base case of no
                return 0;
            })
            .collect();

    let sum: i32 = coverage.iter().sum();

    println!("The num of total coverages is: {:?}", sum);
}
