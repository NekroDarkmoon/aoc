use std::io::Error;

fn part1(filename: &str) -> Result<i64, Error> {
    let content = std::fs::read_to_string(filename)?;
    let (times, distance) = content.split_once("\r\n").unwrap();

    let times = times
        .split_once(":")
        .unwrap()
        .1
        .trim()
        .split_whitespace()
        .map(|c| c.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let distance = distance
        .split_once(":")
        .unwrap()
        .1
        .trim()
        .split_whitespace()
        .map(|c| c.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let mut counts: Vec<i64> = vec![];
    for i in 0..times.len() {
        let mut count = 0;
        let t = times[i];
        let d = distance[i];

        for j in 0..t {
            let distance_traveled = j * (t - j);
            if distance_traveled > d {
                count += 1;
            }
        }

        counts.push(count);
    }

    let result = counts.iter().product::<i64>();

    return Ok(result);
}

fn part2(filename: &str) -> Result<i64, Error> {
    let content = std::fs::read_to_string(filename)?;
    let (times, distance) = content.split_once("\r\n").unwrap();

    let time = times
        .split_once(":")
        .unwrap()
        .1
        .trim()
        .split_whitespace()
        .fold("".to_string(), |acc, c| acc.to_owned() + c)
        .parse::<i64>()
        .unwrap();

    let distance = distance
        .split_once(":")
        .unwrap()
        .1
        .trim()
        .split_whitespace()
        .fold("".to_string(), |acc, c| acc.to_owned() + c)
        .parse::<i64>()
        .unwrap();

    let mut count = 0;
    for i in 0..time {
        let distance_traveled = i * (time - i);
        if distance_traveled > distance {
            count += 1;
        }
    }

    return Ok(count);
}

fn main() {
    let file_name = "src/input6.txt";

    let p1 = part1(file_name);
    println!("Part 1: {:?}", p1);

    let p2 = part2(file_name).unwrap();
    println!("Part 2: {:?}", p2);
}
