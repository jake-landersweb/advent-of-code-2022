use std::collections::HashMap;

pub fn part2() {
    let mut predictions: Vec<u32> = vec![];

    let lines: Vec<&str> =
        include_str!("/Users/jakelanders/code/advent-of-code-2022/src/lib/day3.txt")
            .lines()
            .collect();

    for i in (0..lines.len()).step_by(3) {
        let mut map: HashMap<char, u32> = HashMap::from([]);
        let l1 = lines[i + 0];
        let l2 = lines[i + 1];
        let l3 = lines[i + 2];
        for i in l1.chars() {
            if !map.contains_key(&i) {
                map.insert(i, 1);
            }
        }
        for i in l2.chars() {
            if map.contains_key(&i) {
                *map.get_mut(&i).unwrap() = 2;
            }
        }
        for i in l3.chars() {
            if map.contains_key(&i) && map.get(&i).unwrap() > &1 {
                if i.is_ascii_uppercase() {
                    predictions.push(i as u32 - 38);
                    break;
                } else {
                    predictions.push(i as u32 - 96);
                    break;
                }
            }
        }
    }

    let sum: u32 = predictions.iter().sum();

    println!("The sum of the priorities is: {:?}", sum);
}

pub fn part1() {
    let priorities: Vec<u32> =
        include_str!("/Users/jakelanders/code/advent-of-code-2022/src/lib/day3.txt")
            .lines()
            .map(|x| {
                let mut map: HashMap<char, u8> = HashMap::from([]);
                let mut duplicate = '_';
                for i in 0..x.len() {
                    let c = x.chars().nth(i).unwrap();
                    if i >= x.len() / 2 {
                        if map.contains_key(&c) {
                            duplicate = c;
                        }
                    } else {
                        if !map.contains_key(&c) {
                            map.insert(c, 0);
                        }
                    }
                }
                if duplicate.is_ascii_uppercase() {
                    return duplicate as u32 - 38;
                } else {
                    return duplicate as u32 - 96;
                }
            })
            .collect();
    let sum: u32 = priorities.iter().sum();

    println!("The sum of the priorities is: {:?}", sum)
}
