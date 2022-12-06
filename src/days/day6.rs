pub fn run() {
    part1();
    part2();
}

fn part1() {
    let input = include_str!("/Users/jakelanders/code/advent-of-code-2022/src/lib/day6.txt");

    let mut current_scope: Vec<char> = vec![];
    let mut index = 0;

    for i in input.chars() {
        index += 1;
        current_scope.push(i);
        if current_scope.len() == 4 {
            if different(&current_scope) {
                break;
            }
            current_scope.remove(0);
        }
    }

    println!("Different = {:?}, scope: {:?}", index, current_scope);
}

fn different(input: &Vec<char>) -> bool {
    let mut seen: Vec<&char> = vec![];
    for i in input {
        if seen.contains(&i) {
            return false;
        }
        seen.push(i);
    }
    return true;
}

fn part2() {
    let input = include_str!("/Users/jakelanders/code/advent-of-code-2022/src/lib/day6.txt");

    let mut current_scope: Vec<char> = vec![];
    let mut index = 0;

    for i in input.chars() {
        index += 1;
        current_scope.push(i);
        if current_scope.len() == 14 {
            if different(&current_scope) {
                break;
            }
            current_scope.remove(0);
        }
    }

    println!("Different = {:?}, scope: {:?}", index, current_scope);
}
