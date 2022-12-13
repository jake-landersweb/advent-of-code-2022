pub fn run() {
    part2();
}

fn part1() {
    let grid = get_grid();

    let size = grid.len();

    let mut total_trees: usize = 0;

    for x in 0..size {
        for y in 0..size {
            if seek(&grid, x, y, size) {
                total_trees += 1;
            }
        }
    }

    println!("The number of trees that are visible is: {:?}", total_trees);
}

fn get_grid() -> Vec<Vec<u8>> {
    let grid: Vec<Vec<u8>> =
        include_str!("/Users/jakelanders/code/advent-of-code-2022/src/lib/day8.txt")
            .lines()
            .map(|x| x.chars().map(|y| y.to_digit(10).unwrap() as u8).collect())
            .collect();
    return grid;
}

fn seek(grid: &Vec<Vec<u8>>, x: usize, y: usize, size: usize) -> bool {
    // check edge
    if x == 0 || y == 0 || x == size - 1 || y == size - 1 {
        return true;
    }
    let tree_size = get_tree_size(grid, x, y);
    let mut block_count: usize = 0;
    // check up
    for i in 0..y {
        if get_tree_size(grid, x, i) >= tree_size {
            block_count += 1;
            break;
        }
    }
    // check down
    for i in y + 1..size {
        if get_tree_size(grid, x, i) >= tree_size {
            block_count += 1;
            break;
        }
    }
    // check left
    for i in 0..x {
        if get_tree_size(grid, i, y) >= tree_size {
            block_count += 1;
            break;
        }
    }
    // check right
    for i in x + 1..size {
        if get_tree_size(grid, i, y) >= tree_size {
            block_count += 1;
            break;
        }
    }

    return block_count < 4;
}

fn get_tree_size(grid: &Vec<Vec<u8>>, x: usize, y: usize) -> u8 {
    return grid[x][y];
}

fn part2() {
    let grid = get_grid();
    let size = grid.len();

    let mut best_scene = 0;

    for x in 0..size {
        for y in 0..size {
            let scene = calc_scene(&grid, x, y, size);
            if scene > best_scene {
                best_scene = scene;
            }
        }
    }

    println!("The tree with the best scene is: {:?}", best_scene);
}

fn calc_scene(grid: &Vec<Vec<u8>>, x: usize, y: usize, size: usize) -> usize {
    let tree_size = get_tree_size(grid, x, y);

    let mut up: usize = 0;
    let mut down: usize = 0;
    let mut left: usize = 0;
    let mut right: usize = 0;

    // check up
    for i in (0..y).rev() {
        up += 1;
        if get_tree_size(grid, x, i) >= tree_size {
            break;
        }
    }
    // check down
    for i in y + 1..size {
        down += 1;
        if get_tree_size(grid, x, i) >= tree_size {
            break;
        }
    }
    // check left
    for i in (0..x).rev() {
        left += 1;
        if get_tree_size(grid, i, y) >= tree_size {
            break;
        }
    }
    // check right
    for i in x + 1..size {
        right += 1;
        if get_tree_size(grid, i, y) >= tree_size {
            break;
        }
    }

    return up * down * left * right;
}
