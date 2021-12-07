use std::{
    collections::BTreeSet,
    ops::{Index, IndexMut},
};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Board {
    board: Vec<usize>,
    size: usize,
    found: BTreeSet<usize>,
    won: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Bingo {
    input: Vec<usize>,
    boards: Vec<Board>,
}

impl Index<(usize, usize)> for Board {
    type Output = usize;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.board[index.0 + index.1 * self.size]
    }
}

impl IndexMut<(usize, usize)> for Board {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.board[index.0 + index.1 * self.size]
    }
}

impl Board {
    fn new(size: usize, values: Vec<usize>) -> Board {
        Board {
            board: values,
            size,
            found: BTreeSet::new(),
            won: false,
        }
    }

    fn is_bingo(&self) -> bool {
        // columns
        let mut columns: Vec<_> = self.found.iter().map(|idx| idx % self.size).collect();
        columns.sort();
        if columns
            .group_by(|x, y| x == y)
            .any(|group| group.len() == self.size)
        {
            return true;
        }
        // rows
        let rows: Vec<_> = self.found.iter().map(|idx| idx / self.size).collect();
        if rows
            .group_by(|x, y| x == y)
            .any(|group| group.len() == self.size)
        {
            return true;
        }

        false
    }
}

pub fn generate(input: &str) -> Bingo {
    let mut iter = input.lines();
    let rnginput = iter
        .next()
        .unwrap()
        .split(',')
        .map(str::parse::<usize>)
        .filter_map(Result::ok)
        .collect();

    let mut boards = vec![];

    // boards here empty line seperated
    while let Some("") = iter.next() {
        let myiter = iter.clone();
        let board = myiter
            .take(5)
            .map(|l| {
                l.split_whitespace()
                    .map(str::parse::<usize>)
                    .filter_map(Result::ok)
            })
            .flatten();

        boards.push(Board::new(5, board.collect()));
        let _ = iter.advance_by(5);
    }

    Bingo {
        input: rnginput,
        boards,
    }
}

fn find_winner(bingo: &mut Bingo, part2: bool) -> (usize, usize) {
    let mut last = (0, 0);
    for i in &bingo.input {
        for (bidx, b) in bingo.boards.iter_mut().enumerate().filter(|(_, b)| !b.won) {
            if let Some(idx) = b.board.iter().position(|n| n == i) {
                b.found.insert(idx);
                if b.is_bingo() {
                    if !part2 {
                        return (*i, bidx);
                    } else {
                        last = (*i, bidx);
                        b.won = true;
                    }
                }
            }
        }
    }
    last
}

fn solve(input: &Bingo, part2: bool) -> usize {
    let mut bingo = input.clone();

    let (winning_number, board_idx) = find_winner(&mut bingo, part2);

    let unusedsum: usize = bingo.boards[board_idx]
        .board
        .iter()
        .enumerate()
        .filter(|(i, _)| !bingo.boards[board_idx].found.contains(i))
        .map(|(_, n)| n)
        .sum();

    unusedsum * winning_number
}

pub fn part1(input: &Bingo) -> usize {
    solve(input, false)
}

pub fn part2(input: &Bingo) -> usize {
    solve(input, true)
}

#[cfg(test)]
mod test {
    use crate::day4::{Bingo, Board};

    const INPUT: &str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

    #[test]
    fn test_generate() {
        let rnginput = vec![
            7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8, 19,
            3, 26, 1,
        ];

        let board1 = vec![
            22, 13, 17, 11, 0, 8, 2, 23, 4, 24, 21, 9, 14, 16, 7, 6, 10, 3, 18, 5, 1, 12, 20, 15,
            19,
        ];
        let board2 = vec![
            3, 15, 0, 2, 22, 9, 18, 13, 17, 5, 19, 8, 7, 25, 23, 20, 11, 10, 24, 4, 14, 21, 16, 12,
            6,
        ];
        let board3 = vec![
            14, 21, 17, 24, 4, 10, 16, 15, 9, 19, 18, 8, 23, 26, 20, 22, 11, 13, 6, 5, 2, 0, 12, 3,
            7,
        ];

        let expected = Bingo {
            input: rnginput,
            boards: vec![
                Board::new(5, board1),
                Board::new(5, board2),
                Board::new(5, board3),
            ],
        };
        assert_eq!(super::generate(INPUT), expected);
    }

    #[test]
    fn part1() {
        let input = super::generate(INPUT);
        assert_eq!(super::part1(&input), 4512);
    }

    #[test]
    fn part2() {
        let input = super::generate(INPUT);
        assert_eq!(super::part2(&input), 1924);
    }
}
