use std::collections::HashSet;

#[derive(Debug, Clone)]
struct Board {
    board: Vec<Option<u32>>,
}

impl Board {
    pub fn new(board: Vec<Option<u32>>) -> Self {
        Self {
            board
        }
    }

    fn sum(&self) -> u32 {
        self.board.iter().map(|num| num.unwrap_or(0)).sum()
    }

    fn horizontal_bingo(&self) -> bool {
        (0..5).any(|r| self.board[r * 5..][..5].iter().all(Option::is_none))
    }

    fn vertical_bingo(&self) -> bool {
        (0..5).any(|c| self.board.iter().skip(c).step_by(5).all(Option::is_none))
    }

    fn mark(&mut self, draw: u32) -> bool {
        for cell in &mut self.board {
            if let Some(inner_value) = cell {
                if *inner_value == draw {
                    *cell = None;
                }
            }
        }
        self.horizontal_bingo() || self.vertical_bingo()
    }
}

fn part_1(draws: &Vec<u32>, mut boards: Vec<Board>) -> u32 {
    for &draw in draws {
        for board in &mut boards {
            if board.mark(draw) {
                return board.sum() * draw;
            }
        }
    }
    return 0;
}

fn part_2(draws: &Vec<u32>, mut boards: Vec<Board>) -> u32 {
    let mut winners = HashSet::new();
    let board_count = boards.len();
    for &draw in draws {
        for (board_number, board) in &mut boards.iter_mut().enumerate() {
            if board.mark(draw) {
                winners.insert(board_number);
                if winners.len() == board_count {
                    return board.sum() * draw;
                }
            }
        }
    }
    return 0;
}

fn main() {
    let (draws, entries) = include_str!("input.txt").split_once("\n\n").unwrap();
    let draws: Vec<u32> = draws.split(",").map(|draw| draw.parse().unwrap()).collect();
    let mut boards: Vec<Board> = vec![];
    for line in entries.split("\n\n") {
        let mut board: Vec<Option<u32>> = vec![];
        for n in line.split_whitespace() {
            board.push(Some(n.parse().unwrap()));
        }
        boards.push(Board::new(board));
    }
    let part_1 = part_1(&draws, boards.clone());
    let part_2 = part_2(&draws, boards);
    println!("part 1 {}, part 2 {}", part_1, part_2);
}

// 31
