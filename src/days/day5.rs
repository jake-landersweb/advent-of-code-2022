use std::process::exit;

pub fn part2() {
    let items: Vec<&str> =
        include_str!("/Users/jakelanders/code/advent-of-code-2022/src/lib/day5.txt")
            .split("\n\n")
            .map(|x| {
                println!("{:?}", x.matches("").count());
                return x;
            })
            .collect();

    // compose header
    let lines = items[0].lines().collect::<Vec<&str>>();
    let mut crates = getCrates(lines);

    // loop over moves
    let moves = items[1].lines().collect::<Vec<&str>>();

    for i in moves {
        let options: Vec<&str> = i.split(" ").collect();
        let amount = options[1].parse::<usize>().unwrap();
        let from = options[3].parse::<usize>().unwrap();
        let to = options[5].parse::<usize>().unwrap();

        let mut tmp: Vec<&str> = vec![];

        for _ in 0..amount {
            tmp.push(crates[from - 1].pop().unwrap());
        }

        tmp = tmp.into_iter().rev().collect();

        for i in tmp {
            crates[to - 1].push(i);
        }
    }

    // get last values
    let mut last: Vec<&str> = vec![];

    for i in crates {
        last.push(i.last().unwrap());
    }

    println!("{:?}", last);
}

pub fn part1() {
    let items: Vec<&str> =
        include_str!("/Users/jakelanders/code/advent-of-code-2022/src/lib/day5.txt")
            .split("\n\n")
            .map(|x| {
                println!("{:?}", x.matches("").count());
                return x;
            })
            .collect();

    // compose header
    let lines = items[0].lines().collect::<Vec<&str>>();
    let mut crates = getCrates(lines);

    // loop over moves
    let moves = items[1].lines().collect::<Vec<&str>>();

    for i in moves {
        let options: Vec<&str> = i.split(" ").collect();
        let amount = options[1].parse::<usize>().unwrap();
        let from = options[3].parse::<usize>().unwrap();
        let to = options[5].parse::<usize>().unwrap();

        for _ in 0..amount {
            let tmp = crates[from - 1].pop().unwrap();
            crates[to - 1].push(tmp);
        }
    }

    // get last values
    let mut last: Vec<&str> = vec![];

    for i in crates {
        last.push(i.last().unwrap());
    }

    println!("{:?}", last);
}

pub fn getCrates(mut input: Vec<&str>) -> Vec<Vec<&str>> {
    let mut crates: Vec<Vec<&str>> = std::iter::repeat(vec![]).take(9).collect::<Vec<_>>();
    input.pop();

    for i in input {
        let spl = i.split(" ");
        let mut space_count = 0;
        let mut letters: Vec<&str> = vec![];
        for i in spl {
            if i == "" {
                // space
                space_count += 1;
                if space_count == 4 {
                    letters.push(" ");
                    space_count = 0;
                }
            } else if i != "[" && i != "]" {
                // letter
                letters.push(i);
            }
        }
        // add in transposed way
        for i in 0..letters.len() {
            crates[i].push(letters[i]);
        }
    }
    crates = crates
        .into_iter()
        .map(|x| x.into_iter().rev().filter(|&item| item != " ").collect())
        .collect();

    return crates;
}
