const INPUT: &str = include_str!("../day22.txt");

const NUM_CARDS_PT1: usize = 10007;

fn main() {
    part1();
}

fn part1() {
    println!(
        "Day 22-1: Card 2019 is at index {}",
        calc_part1().expect("Not found")
    )
}

type CardNo = usize;
fn calc_part1() -> Option<usize> {
    let mut cards: Vec<CardNo> = Vec::new();
    for i in 0..NUM_CARDS_PT1 {
        cards.push(i);
    }

    ShuffleStrategy::parse_strategies(INPUT)
        .iter()
        .for_each(|s| s.apply(&mut cards));

    for (i, value) in cards.iter().enumerate() {
        let card_no: CardNo = *value;
        if card_no == 2019 {
            return Some(i);
        }
    }
    None
}

#[derive(Debug)]
enum ShuffleStrategy {
    DealIntoNewStack,
    Cut { n: i32 },
    DealWithIncrement { n: usize },
}

impl ShuffleStrategy {
    fn parse_strategies(input: &str) -> Vec<ShuffleStrategy> {
        let mut strats = Vec::new();
        for line in input.trim().lines() {
            if line == "deal into new stack" {
                strats.push(ShuffleStrategy::DealIntoNewStack);
            } else if line.starts_with("deal with increment ") {
                let n = &line[20..].parse::<i32>().unwrap();
                strats.push(ShuffleStrategy::DealWithIncrement { n: *n as usize })
            } else {
                let n = &line[4..].parse::<i32>().unwrap();
                strats.push(ShuffleStrategy::Cut { n: *n })
            }
        }
        strats
    }

    fn apply(&self, cards: &mut Vec<usize>) {
        match self {
            ShuffleStrategy::DealIntoNewStack => {
                cards.reverse();
            }
            ShuffleStrategy::Cut { n } => {
                if *n >= 0 {
                    for _i in 0..*n {
                        let removed: CardNo = cards.remove(0);
                        cards.push(removed);
                    }
                } else {
                    for _i in 0..(*n * -1) {
                        let removed: CardNo = cards.pop().unwrap();
                        cards.insert(0, removed)
                    }
                }
            }
            ShuffleStrategy::DealWithIncrement { n } => {
                let mut arr: [usize; NUM_CARDS_PT1] = [NUM_CARDS_PT1 + 1; NUM_CARDS_PT1];

                let mut i = 0;
                for card in cards.iter() {
                    arr[i] = *card;
                    i = (i + *n) % NUM_CARDS_PT1;
                }

                cards.clear();
                arr.iter().for_each(|&c| cards.push(c));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(calc_part1(), Some(2514))
    }
}
