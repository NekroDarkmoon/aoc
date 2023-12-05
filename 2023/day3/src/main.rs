use std::collections::HashMap;

fn parse_grid(input: &str) -> HashMap<(i64, i64), char> {
    let mut grid = HashMap::new();

    for (y, line) in input.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            grid.insert((x as i64, y as i64), char);
        }
    }

    return grid;
}

fn adjacent_sum1(filename: &str) -> i64 {
    let content = std::fs::read_to_string(filename).expect("Failed to read file");
    let mut sum = 0;

    let grid = parse_grid(&content);

    let symbols = grid
        .iter()
        .filter(|(_, char)| !char.is_digit(10) && **char != '.')
        .map(|(pos, _)| pos.clone())
        .collect::<Vec<(i64, i64)>>();

    let digits = grid
        .iter()
        .filter(|(_, char)| char.is_ascii_digit())
        .map(|(pos, _)| pos.clone())
        .collect::<Vec<(i64, i64)>>();

    let mut adjacent_numbers = vec![];

    for pos in digits.iter() {
        let mut points = vec![];
        let mut x = pos.clone();

        let mut char = grid.get(&x);
        while let Some(data) = char {
            if data.is_digit(10) {
                points.push(x.clone());
                x = (x.0 + 1, x.1);
                char = grid.get(&x);
            } else {
                break;
            }
        }

        if !digits.contains(&(pos.0 - 1, pos.1)) {
            for symbol in symbols.iter() {
                // Check adjacent and diagonal points to see if they are in the points vector
                let check_points = vec![
                    (symbol.0, symbol.1 - 1),
                    (symbol.0, symbol.1 + 1),
                    (symbol.0 - 1, symbol.1),
                    (symbol.0 + 1, symbol.1),
                    (symbol.0 - 1, symbol.1 - 1),
                    (symbol.0 + 1, symbol.1 - 1),
                    (symbol.0 - 1, symbol.1 + 1),
                    (symbol.0 + 1, symbol.1 + 1),
                ];

                for check_point in check_points.iter() {
                    if points.contains(check_point) {
                        adjacent_numbers.push(pos.clone());
                    }
                }
            }
        }
    }

    adjacent_numbers.sort();
    adjacent_numbers.dedup();

    for mut pos in adjacent_numbers.into_iter() {
        let mut numbers = vec![];
        let mut value = grid.get(&pos);

        while let Some(data) = value {
            if data.is_digit(10) {
                numbers.push(data.clone());
                pos = (pos.0 + 1, pos.1);
                value = grid.get(&pos);
            } else {
                break;
            }
        }

        // Convert the vector of chars to a string and then parse it to an i64
        sum += numbers.iter().collect::<String>().parse::<i64>().unwrap();
    }

    return sum;
}

fn adjacent_sum2(filename: &str) -> i64 {
    let content = std::fs::read_to_string(filename).expect("Failed to read file");
    let mut sum = 0;

    let grid = parse_grid(&content);

    let symbols = grid
        .iter()
        .filter(|(_, char)| **char == '*')
        .map(|(pos, _)| pos.clone())
        .collect::<Vec<(i64, i64)>>();

    let digits = grid
        .iter()
        .filter(|(_, char)| char.is_ascii_digit())
        .map(|(pos, _)| pos.clone())
        .collect::<Vec<(i64, i64)>>();

    let mut adjacent_numbers: HashMap<(i64, i64), Vec<(i64, i64)>> = HashMap::new();

    for pos in digits.iter() {
        let mut points = vec![];
        let mut x = pos.clone();

        let mut char = grid.get(&x);
        while let Some(data) = char {
            if data.is_digit(10) {
                points.push(x.clone());
                x = (x.0 + 1, x.1);
                char = grid.get(&x);
            } else {
                break;
            }
        }

        if !digits.contains(&(pos.0 - 1, pos.1)) {
            for symbol in symbols.iter() {
                // Check adjacent and diagonal points to see if they are in the points vector
                let check_points = vec![
                    (symbol.0, symbol.1 - 1),
                    (symbol.0, symbol.1 + 1),
                    (symbol.0 - 1, symbol.1),
                    (symbol.0 + 1, symbol.1),
                    (symbol.0 - 1, symbol.1 - 1),
                    (symbol.0 + 1, symbol.1 - 1),
                    (symbol.0 - 1, symbol.1 + 1),
                    (symbol.0 + 1, symbol.1 + 1),
                ];

                for check_point in check_points.iter() {
                    if points.contains(check_point) {
                        match adjacent_numbers.get_mut(symbol) {
                            Some(value) => value.push(pos.clone()),
                            None => {
                                adjacent_numbers.insert(symbol.clone(), vec![pos.clone()]);
                            }
                        }
                    }
                }
            }
        }
    }

    // Dedup the hashmap values
    for (_, value) in adjacent_numbers.iter_mut() {
        value.sort();
        value.dedup();
    }

    for (_, value) in adjacent_numbers.iter() {
        if value.len() < 2 {
            continue;
        }

        let mut prod = 1;
        for mut pos in value.clone() {
            let mut numbers = vec![];
            let mut value = grid.get(&pos);

            while let Some(data) = value {
                if data.is_digit(10) {
                    numbers.push(data.clone());
                    pos = (pos.0 + 1, pos.1);
                    value = grid.get(&pos);
                } else {
                    break;
                }
            }

            prod *= numbers.iter().collect::<String>().parse::<i64>().unwrap();
        }

        sum += prod;
    }

    return sum;
}

fn main() {
    let filename = "./src/input3.txt";

    let part1 = adjacent_sum1(filename);
    println!("Part 1: {}", part1);

    let part2 = adjacent_sum2(filename);
    println!("Part 2: {}", part2);
}
