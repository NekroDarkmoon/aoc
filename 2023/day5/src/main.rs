use std::thread;

fn get_maps(content: String) -> Vec<Vec<[i64; 3]>> {
    let sections: Vec<String> = content
        .to_string()
        .split("\r\n\r\n")
        .map(|s| s.to_string())
        .collect();

    let mut maps = vec![];

    for section in sections.into_iter().skip(1) {
        let mut map = vec![];
        for line in section.lines().skip(1) {
            let info = line
                .split_whitespace()
                .map(|s| s.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();

            map.push([info[0], info[1], info[2]]);
        }

        maps.push(map);
    }

    return maps;
}

fn get_seeds1(content: String) -> Vec<i64> {
    let seeds: Vec<i64> = content
        .lines()
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .split_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
        .collect();

    return seeds;
}

fn get_lowest_location1(filename: &str) -> i64 {
    let content = std::fs::read_to_string(filename).expect("Failed to read file");

    let maps: Vec<Vec<[i64; 3]>> = get_maps(content.clone());
    let mut lowest_location: i64 = -1;
    let seeds: Vec<i64> = get_seeds1(content);

    for mut seed in seeds {
        for map in maps.iter() {
            let mut next_step = seed;

            for &[dest, src, range] in map {
                if (src..(src + range)).contains(&seed) {
                    next_step = dest + (seed - src);
                }
            }

            seed = next_step;
        }

        if lowest_location == -1 {
            lowest_location = seed;
        } else {
            lowest_location = lowest_location.min(seed);
        }
    }

    return lowest_location;
}

fn get_seeds2(content: String) -> Vec<i64> {
    let mut seeds: Vec<i64> = vec![];

    let seed_data: Vec<i64> = get_seeds1(content);
    for chunk in seed_data.chunks(2) {
        let new_seeds: Vec<i64> = (chunk[0]..(chunk[0] + chunk[1])).collect();
        seeds.extend(new_seeds);
    }

    println!("Done Creating Seeds: {}", seeds.len());
    return seeds;
}

fn get_lowest_location2(filename: &str) -> i64 {
    let content = std::fs::read_to_string(filename).expect("Failed to read file");

    let maps: Vec<Vec<[i64; 3]>> = get_maps(content.clone());
    let mut lowest_location: i64 = -1;

    let seed_data: Vec<i64> = get_seeds1(content);
    let mut threads = vec![];
    for chunk in seed_data.chunks(2) {
        let new_seeds: Vec<i64> = (chunk[0]..(chunk[0] + chunk[1])).collect();
        println!("Starting thread with {} seeds. ", new_seeds.len());

        // Create threads to run the seeds in parallel giving them a copy of the maps
        let maps_copy = maps.clone();
        let thread = thread::spawn(move || thread_runner(new_seeds, maps_copy));
        threads.push(thread);
    }

    // Get results from threads in a single vector
    let mut possible_cadidates = vec![];
    for thread in threads {
        possible_cadidates.push(thread.join().unwrap());
    }

    // Find the lowest location
    lowest_location = possible_cadidates.iter().min().unwrap().clone();

    return lowest_location;
}

fn thread_runner(seeds: Vec<i64>, maps: Vec<Vec<[i64; 3]>>) -> i64 {
    let mut possible_cadidates = vec![];

    for mut seed in seeds {
        for map in maps.iter() {
            let mut next_step = seed;

            for &[dest, src, range] in map {
                if (src..(src + range)).contains(&seed) {
                    next_step = dest + (seed - src);
                }
            }

            seed = next_step;
        }

        possible_cadidates.push(seed);
    }

    let lowest_location = possible_cadidates.iter().min().unwrap().clone();
    println!("Thread Done: {}", lowest_location);
    return lowest_location;
}

fn main() {
    let filename = "./src/input5.txt";
    let part1 = get_lowest_location1(filename);
    println!("Part 1: {}", part1);

    let part2 = get_lowest_location2(filename);
    println!("Part 2: {}", part2);
}
