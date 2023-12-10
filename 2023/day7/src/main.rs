#[derive(Debug, Clone)]
struct Hand {
    cards: String,
    bid: i64,
    score: i64,
}

fn get_card_order_score1(card: &str) -> Vec<i64> {
    let card = card.to_string();
    let value = card
        .chars()
        .map(|c: char| {
            let val = match c {
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'J' => 11,
                'T' => 10,
                _ => c.to_string().parse::<i64>().unwrap(),
            };

            return val;
        })
        .collect::<Vec<i64>>();

    return value;
}

fn get_card_order_score2(card: &str) -> Vec<i64> {
    let card = card.to_string();
    let value = card
        .chars()
        .map(|c: char| {
            let val = match c {
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'J' => 0,
                'T' => 10,
                _ => c.to_string().parse::<i64>().unwrap(),
            };

            return val;
        })
        .collect::<Vec<i64>>();

    return value;
}

fn part1(filename: &str) -> i64 {
    let content = std::fs::read_to_string(filename).expect("Failed to read file");

    let hands = content
        .lines()
        .map(|line| {
            let (cards, bid) = line.split_once(" ").unwrap();
            let bid = bid.parse::<i64>().unwrap();

            let counts = cards
                .chars()
                .fold(std::collections::HashMap::new(), |mut acc, c| {
                    *acc.entry(c).or_insert(0) += 1;
                    acc
                });

            // Get highest count of cards
            let freq = counts.values().max().unwrap();
            let multi_twos = counts.values().filter(|&v| *v == 2).count();
            let rank = match freq {
                5 => 7,
                4 => 6,
                3 if multi_twos == 1 => 5,
                3 => 4,
                2 if multi_twos == 2 => 3,
                2 => 2,
                1 => 1,
                _ => 0,
            };

            return Hand {
                cards: cards.to_string(),
                bid,
                score: rank,
            };
        })
        .collect::<Vec<Hand>>();

    // Sort by score
    let mut sorted_hands = hands.clone();
    sorted_hands.sort_by(|a, b| {
        let order = a.score.cmp(&b.score);
        if order != std::cmp::Ordering::Equal {
            return order;
        }

        let a_score = get_card_order_score1(&a.cards);
        let b_score = get_card_order_score1(&b.cards);

        return a_score.cmp(&b_score);
    });

    let mut total = 0;
    for (rank, hand) in sorted_hands.iter().enumerate() {
        print!("{:?} ", hand);
        println!("{} * {}", hand.bid, rank + 1);
        total += hand.bid * (rank as i64 + 1);
    }

    return total;
}

fn part2(filename: &str) -> i64 {
    let content = std::fs::read_to_string(filename).expect("Failed to read file");

    let hands = content
        .lines()
        .map(|line| {
            let (cards, bid) = line.split_once(" ").unwrap();
            let bid = bid.parse::<i64>().unwrap();

            let mut wild_counts = 0;
            let counts = cards
                .chars()
                .fold(std::collections::HashMap::new(), |mut acc, c| {
                    if c == 'J' {
                        wild_counts += 1;
                        return acc;
                    }

                    *acc.entry(c).or_insert(0) += 1;
                    acc
                });

            // Get highest count of cards
            let freq = counts.values().max().unwrap_or(&0) + wild_counts;
            let multi_twos = counts.values().filter(|&v| *v == 2).count();
            let rank = match freq {
                5 => 7,
                4 => 6,
                3 if multi_twos == 1 => 5,
                3 => 4,
                2 if multi_twos == 2 => 3,
                2 => 2,
                1 => 1,
                _ => 0,
            };

            return Hand {
                cards: cards.to_string(),
                bid,
                score: rank,
            };
        })
        .collect::<Vec<Hand>>();

    // Sort by score
    let mut sorted_hands = hands.clone();
    sorted_hands.sort_by(|a, b| {
        let order = a.score.cmp(&b.score);
        if order != std::cmp::Ordering::Equal {
            return order;
        }

        let a_score = get_card_order_score2(&a.cards);
        let b_score = get_card_order_score2(&b.cards);

        return a_score.cmp(&b_score);
    });

    let mut total = 0;
    for (rank, hand) in sorted_hands.iter().enumerate() {
        print!("{:?} ", hand);
        println!("{} * {}", hand.bid, rank + 1);
        total += hand.bid * (rank as i64 + 1);
    }

    return total;
}

fn main() {
    let file_name = "src/input7.txt";

    let p1 = part1(file_name);
    println!("Part 1: {}", p1);

    let p2 = part2(file_name);
    println!("Part 2: {}", p2);
}
