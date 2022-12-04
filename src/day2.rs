use std::collections::HashMap;

pub fn part1() {
    let score_map = HashMap::from([("A", 1), ("B", 2), ("C", 3)]);
    let score_map2 = HashMap::from([("X", 1), ("Y", 2), ("Z", 3)]);

    let scores: Vec<i32> =
        include_str!("/Users/jakelanders/code/advent-of-code-2022/src/lib/day2.txt")
            .lines()
            .map(|x| {
                let play1: &str = &x.chars().next().unwrap().to_string();
                let play2: &str = &x.chars().last().unwrap().to_string();

                let mut score: i32 = 0;

                // add second score amount on each play
                score += score_map2.get(play2).unwrap();

                // check that player2 wins
                if (play1 == "A" && play2 == "Y")
                    || (play1 == "B" && play2 == "Z")
                    || (play1 == "C" && play2 == "X")
                {
                    score += 6;
                } else if score_map.get(play1).unwrap() == score_map2.get(play2).unwrap() {
                    score += 3;
                }

                return score;
            })
            .collect();

    let sum: i32 = scores.iter().sum();

    println!("If everything goes to plan, the total score is: {:?}", sum);
}

pub fn part2() {
    let score_map = HashMap::from([("A", 1), ("B", 2), ("C", 3)]);
    let lose_map = HashMap::from([("A", 3), ("B", 1), ("C", 2)]);
    let win_map = HashMap::from([("A", 2), ("B", 3), ("C", 1)]);

    let scores: Vec<i32> =
        include_str!("/Users/jakelanders/code/advent-of-code-2022/src/lib/day2.txt")
            .lines()
            .map(|x| {
                let play1: &str = &x.chars().next().unwrap().to_string();
                let play2: &str = &x.chars().last().unwrap().to_string();

                let mut score: i32 = 0;

                // check what we need to do
                if play2 == "X" {
                    // lose
                    score += lose_map.get(play1).unwrap();
                } else if play2 == "Y" {
                    // draw
                    score += score_map.get(play1).unwrap();
                    score += 3;
                } else {
                    // win
                    score += win_map.get(play1).unwrap();
                    score += 6;
                }

                return score;
            })
            .collect();

    let sum: i32 = scores.iter().sum();

    println!("If everything goes to plan, the total score is: {:?}", sum);
}
