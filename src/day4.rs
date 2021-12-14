use std::{
    collections::HashSet,
    fmt::{Debug, Display},
};

pub fn part1(input: Vec<String>) -> u32 {
    let mut input_iter = input.iter();

    // the first line is the numbers
    let number_order: Vec<u32> = input_iter
        .next()
        .unwrap()
        .split(",")
        .map(|i| i.parse::<u32>().unwrap())
        .collect();
    input_iter.next();

    let mut bingo_cards: Vec<BingoCard> = vec![];
    let mut curr_input: Vec<String> = vec![];

    for line in input_iter {
        if line.is_empty() {
            let card = BingoCard::new(curr_input);
            bingo_cards.push(card);
            curr_input = vec![];
            continue;
        }

        curr_input.push(line.to_string());
    }

    for p in number_order.iter() {
        match bingo_cards.iter_mut().any(|card| {
            card.mark_selected(*p);
            card.is_winner()
        }) {
            true => {
                let winner_card = bingo_cards.iter().find(|card| card.is_winner()).unwrap();
                println!("We found a winner! Winning card: {}", winner_card);

                return winner_card.calculate_score(*p);
            }
            false => println!("Still no winner"),
        }
    }

    panic!("We should not have hit here")
}

pub fn part2(input: Vec<String>) -> u32 {
    let mut input_iter = input.iter();

    // the first line is the numbers
    let number_order: Vec<u32> = input_iter
        .next()
        .unwrap()
        .split(",")
        .map(|i| i.parse::<u32>().unwrap())
        .collect();
    input_iter.next();

    let mut bingo_cards: Vec<BingoCard> = vec![];
    let mut curr_input: Vec<String> = vec![];

    for line in input_iter {
        if line.is_empty() {
            let card = BingoCard::new(curr_input);
            bingo_cards.push(card);
            curr_input = vec![];
            continue;
        }

        curr_input.push(line.to_string());
    }

    for p in number_order.iter() {
        let before_count = bingo_cards.len();
        let non_winning_bingo_cards: Vec<BingoCard> = bingo_cards
            .iter_mut()
            .filter_map(|card| {
                card.mark_selected(*p);
                match card.is_winner() {
                    true => None,
                    false => Some(card.to_owned()),
                }
            })
            .collect();

        println!(
            "Before: {}, After: {}",
            before_count,
            non_winning_bingo_cards.len()
        );

        if bingo_cards.len() == 1 && non_winning_bingo_cards.is_empty() {
            println!("Last bingo: {}", bingo_cards[0]);
            return bingo_cards[0].calculate_score(*p);
        }

        bingo_cards = non_winning_bingo_cards;
    }

    panic!("We should not have hit here")
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct BingoCard {
    squares: Vec<Vec<(u32, bool)>>,
}

impl Display for BingoCard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = writeln!(f, "[");
        self.squares.iter().for_each(|row| {
            let _ = writeln!(f, "{:?}", row);
        });
        let _ = writeln!(f, "]");
        Ok(())
    }
}

impl BingoCard {
    fn new(input: Vec<String>) -> Self {
        let mut rows = vec![];

        input.iter().for_each(|line| {
            let mut squares: Vec<(u32, bool)> = vec![];
            line.split_whitespace().for_each(|bingo_number_str| {
                let bingo_number = bingo_number_str.parse::<u32>().unwrap();
                squares.push((bingo_number, false));
            });
            rows.push(squares);
        });

        Self { squares: rows }
    }

    /**
     * First we check for a horizontal win, and then a vertical win
     */
    fn is_winner(&self) -> bool {
        // first check rows for wins
        let row_win = self.squares.iter().any(|row| {
            // check to see if all of the squares in the row are true
            row.iter().all(|square| square.1)
            // if all have been selected, return true and stop looking
        });

        if row_win {
            return true;
        }

        // now check columns!

        for i in 0..self.squares.len() {
            let mut vertical_win = true;
            for j in 0..self.squares.get(i).unwrap().len() {
                let s = self.squares.get(j).unwrap().get(i).unwrap();
                if !s.1 {
                    vertical_win = false
                }
            }

            if vertical_win {
                return true;
            }
        }

        false
    }

    fn mark_selected(&mut self, selected_num: u32) -> bool {
        return self.squares.iter_mut().any(|row| {
            for s in row.iter_mut() {
                if s.0 == selected_num {
                    s.1 = true;
                    return true;
                }
            }
            return false;
        });
    }

    /**
     * The score of the winning board can now be calculated. Start by finding
     * the sum of all unmarked numbers on that board; in this case, the sum is
     * 188. Then, multiply that sum by the number that was just called when the
     * board won, 24, to get the final score, 188 * 24 = 4512.
     */
    fn calculate_score(&self, last_called_number: u32) -> u32 {
        let unmarked_numbers_sum: u32 = self
            .squares
            .iter()
            .map(|r| -> u32 {
                r.iter()
                    .map(|s| match s.1 {
                        true => 0,
                        false => s.0,
                    })
                    .sum()
            })
            .sum();

        unmarked_numbers_sum * last_called_number
    }
}

#[cfg(test)]
mod tests {
    use super::BingoCard;

    #[test]
    fn bingo_create_test() {
        let input = "2 4 25
8 6 10
11 21 32"
            .lines()
            .map(|s| s.to_string())
            .collect();

        let bingo_card = BingoCard::new(input);
        assert!(!bingo_card.is_winner());
    }

    #[test]
    fn bingo_vertical_win_test() {
        let input = "2 4 25
8 6 10
11 21 32"
            .lines()
            .map(|s| s.to_string())
            .collect();

        let mut bingo_card = BingoCard::new(input);

        assert!(!bingo_card.is_winner());

        bingo_card.mark_selected(10);
        bingo_card.mark_selected(21);
        bingo_card.mark_selected(4);
        assert!(!bingo_card.is_winner());

        bingo_card.mark_selected(6);

        assert!(bingo_card.is_winner());

        println!("CARD: {}", bingo_card);
        assert_eq!(bingo_card.calculate_score(6), 78 * 6)
    }

    #[test]
    fn bingo_horizontal_win_test() {
        let input = "2 4 25
8 6 10
11 21 32"
            .lines()
            .map(|s| s.to_string())
            .collect();

        let mut bingo_card = BingoCard::new(input);

        assert!(!bingo_card.is_winner());

        bingo_card.mark_selected(10);
        bingo_card.mark_selected(8);
        assert!(!bingo_card.is_winner());

        bingo_card.mark_selected(6);
        assert!(bingo_card.is_winner());
    }
}
