fn get_correct_games(file_name: &str) -> i64 {
    let content = std::fs::read_to_string(file_name).expect("Failed to read file");
    let mut sum: i64 = 0;

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

        // Go through and check if each set is valid
        let mut valid = true;
        for set in sets {
            for (color, quantity) in set {
                if color == "red" && quantity > 12 {
                    valid = false;
                    break;
                } else if color == "green" && quantity > 13 {
                    valid = false;
                    break;
                } else if color == "blue" && quantity > 14 {
                    valid = false;
                    break;
                }
            }
        }

        if valid {
            println!("{} is valid", id);
            sum += id;
        }
    }

    return sum;
}

fn main() {
    let file_name: &str = "src/input2.txt";

    let part1 = get_correct_games(file_name);
    println!("Part1: {}", part1);
}
