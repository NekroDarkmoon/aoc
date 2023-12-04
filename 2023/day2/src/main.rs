fn get_correct_games(file_name: &str) -> (i64, i64) {
    let content = std::fs::read_to_string(file_name).expect("Failed to read file");
    let mut sum: i64 = 0;
    let mut power_sum: i64 = 0;

    for line in content.lines() {
        let id = line
            .split_once(|c| c == ':')
            .unwrap()
            .0
            .split_whitespace()
            .last()
            .unwrap()
            .parse::<i64>()
            .unwrap();

        // Move line to start after the colon
        let line = line.split_once(|c| c == ':').unwrap().1.trim_start();

        let sets = line
            .split(';')
            .map(|set| {
                set.split(',')
                    .map(|value| {
                        let value = value.trim();
                        let color = value.split_whitespace().last().unwrap();
                        let quantity = value
                            .split_whitespace()
                            .next()
                            .unwrap()
                            .parse::<i64>()
                            .unwrap();

                        (color, quantity)
                    })
                    .collect()
            })
            .collect::<Vec<Vec<(&str, i64)>>>();

        // Go through and check if each set is valid and get power set of minimums
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        let mut valid = true;

        for set in sets {
            for (color, quantity) in set {
                if color == "red" {
                    red = red.max(quantity);
                } else if color == "green" {
                    green = green.max(quantity);
                } else if color == "blue" {
                    blue = blue.max(quantity);
                }
            }
        }

        if red > 12 || green > 13 || blue > 14 {
            valid = false;
        }

        if valid {
            sum += id;
        }

        power_sum += red * green * blue;
    }

    return (sum, power_sum);
}

fn main() {
    let file_name: &str = "src/input2.txt";

    let (part1, part2) = get_correct_games(file_name);
    println!("Part1: {}", part1);
    println!("Part2: {}", part2);
}
