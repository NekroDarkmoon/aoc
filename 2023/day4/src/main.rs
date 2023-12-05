fn get_winning_numbers(filename: &str) -> (i64, i64) {
    let content = std::fs::read_to_string(filename).expect("Failed to read file");

    let mut sum = 0;

    let mut cards: Vec<i64> = vec![1; content.lines().count()];

    for (card_num, line) in content.lines().enumerate() {
        let (_, card) = line.split_once(": ").unwrap();
        let (winning_numbers, card_numbers) = card.split_once(" | ").unwrap();

        let winning_numbers: Vec<i64> = winning_numbers
            .split_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .collect();

        let card_numbers: Vec<i64> = card_numbers
            .split_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .collect();

        let mut count: i64 = 0;
        let mut score: i64 = 0;

        for number in card_numbers {
            if winning_numbers.contains(&number) {
                if count == 0 {
                    score += 1;
                } else {
                    score *= 2;
                }

                count += 1;
            }
        }

        for pos in (card_num + 1)..(card_num + (count as usize) + 1) {
            cards[pos] += cards[card_num];
        }

        sum += score;
    }

    let total_cards = cards.iter().sum();

    return (sum, total_cards);
}

fn main() {
    let filename = "src/input4.txt";
    let (part1, part2) = get_winning_numbers(filename);
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
