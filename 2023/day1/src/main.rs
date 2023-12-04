fn get_calibration_values(filename: &str) -> i64 {
    let content = std::fs::read_to_string(filename).unwrap();

    let mut sum = 0;

    for line in content.lines() {
        let digits: Vec<i64> = line
            .to_string()
            .chars()
            .filter_map(|c| c.to_digit(10).map(|c| c as i64))
            .collect::<Vec<i64>>();

        sum += digits[0] * 10 + digits[digits.len() - 1];
    }

    return sum;
}

fn get_calibration_values2(filename: &str) -> i64 {
    let content = std::fs::read_to_string(filename).unwrap();

    let mut sum = 0;
    let numbers = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    for mut line in content.lines() {
        let digits: Vec<i64> = line
            .to_string()
            .chars()
            .filter_map(|c| c.to_digit(10).map(|c| c as i64))
            .collect::<Vec<i64>>();

        // Move starting of line to the first number or instance of a number word
        while !numbers.iter().any(|num| line.starts_with(num))
            && !line.chars().next().unwrap().is_numeric()
        {
            line = &line[1..];
        }

        // Move ending of line to the last number or instance of a number word
        while !numbers.iter().any(|num| line.ends_with(num))
            && !line.chars().last().unwrap().is_numeric()
        {
            line = &line[..line.len() - 1];
        }

        let mut first: i64 = 0;
        let mut last: i64 = 0;

        for (pos, num) in numbers.iter().enumerate() {
            if line.starts_with(num) {
                first = (pos as i64) + 1;
            }

            if line.ends_with(num) {
                last = (pos as i64) + 1;
            }
        }

        if line.chars().next().unwrap().is_numeric() {
            first = digits[0];
        }

        if line.chars().last().unwrap().is_numeric() {
            last = digits[digits.len() - 1];
        }

        sum += first * 10 + last;
    }

    return sum;
}

fn main() {
    let file_name: &str = "src/input.txt";

    let part1: i64 = get_calibration_values(file_name);
    println!("Part1: {}", part1);

    let part2: i64 = get_calibration_values2(file_name);
    println!("Part2: {}", part2);
}
