use std::str::FromStr;
use array2d::Array2D;
// use std::collections::HashMap
use std::collections::HashSet;

advent_of_code::solution!(3);

/* example data
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
*/

#[derive(Debug)]
struct Number {
    start_y: usize,
    start_x: usize,
    size: usize,
    value: String
}

impl Number {
    pub fn is_part_number (&self, board: &Schematic) -> bool {
        // println!("check number {:?}", self);

        let mut rows = vec![];

        if self.start_y > 0 {
            rows.push(self.start_y - 1);
        }
        rows.push(self.start_y);
        if self.start_y < (board.rows() - 1) {
            rows.push(self.start_y + 1);
        }

        for y in rows {
            let start = if self.start_x > 0 {
                self.start_x - 1
            } else {
                0
            };

            let mut end = self.start_x + self.size;

            if end > (board.cols() - 1) {
                end = board.cols() - 1;
            }

            // println!("y {}, start {}, end {}", y, start, end);
            for x in start..=end {
                let char = board.get(x, y);

                // println!("  {}, {} => {}", x, y, char);

                if !char.is_ascii_digit() && char != '.' {
                    return true
                }
            }
        }

        return false
    }
}

// a Schematic contains a visual representation of the engine
#[derive(Debug)]
struct Schematic {
    board: Vec<Vec<char>>
}

impl FromStr for Schematic {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let board: Vec<Vec<char>> = s.lines().map(
            |line| line.chars().collect()
        ).collect();

        Ok(Schematic { board })
    }
}

impl Schematic {
    pub fn rows (&self) -> usize {
        self.board.len()
    }

    pub fn cols (&self) -> usize {
        self.board[0].len()
    }

    pub fn get (&self, x: usize, y: usize) -> char {
        self.board[y][x]
    }

    pub fn get_numbers (&self) -> Vec<Number> {
        let mut numbers: Vec<Number> = vec![];

        for (y, line) in self.board.iter().enumerate() {
            let mut current_num: Option<Number> = None;

            for (x, char) in line.iter().enumerate() {
                if char.is_ascii_digit() {
                    if current_num.is_none() {
                        current_num = Some(Number {
                            start_y: y,
                            start_x: x,
                            size: 1,
                            value: char.to_string()
                        })
                    } else {
                        // move out
                        let mut n = current_num.unwrap();

                        n.size += 1;
                        n.value.push(char.clone());

                        // move back
                        current_num = Some(n);
                    }
                } else {
                    if current_num.is_some() {
                        numbers.push(current_num.unwrap());
                        current_num = None
                    }
                }
            }

            if current_num.is_some() {
                numbers.push(current_num.unwrap());
            }
        }

        // println!("numbers, {:?}", numbers);

        return numbers
    }

    pub fn get_part_numbers_sum (&self) -> u32 {
        self.get_numbers()
            .iter()
            .filter(|n| n.is_part_number(self))
            .map(|n| {
                // println!("{}", n.value);
                n.value.parse::<u32>().unwrap_or(0)
            })
            .sum()
    }
}


pub fn part_one(input: &str) -> Option<u32> {
    let schematic = Schematic::from_str(input).unwrap();
    Some(schematic.get_part_numbers_sum())
}


fn neighbours(board: &Array2D<char>, x: usize, y: usize) -> HashSet<(usize, usize)>  {
    let mut points: HashSet<(usize, usize)> = HashSet::new();

    let mut rows = vec![];

    if y > 0 {
        rows.push(y - 1);
    }
    rows.push(y);
    if y < (board.column_len() - 1) {
        rows.push(y + 1);
    }

    for dy in rows {
        if x > 0 {
            points.insert((x - 1, dy));
        }

        if dy != y {
            points.insert((x, dy));
        }

        if x < (board.row_len() - 1) {
            points.insert((x + 1, dy));
        }
    }

    return points
}


fn get_gear_ratio(board: &Array2D<char>, x: usize, y: usize) -> u32 {
    // find all neighbours
    // let mut s = HashSet::new();
    let mut points = neighbours(board, x, y);
    let mut results = vec![];

    loop {
        // dbg!(&points);
        // stupid way of Set::pop()
        let mut iter = points.iter();
        let value = iter
            .next()
            // clone the value will clear the borrow!?
            .cloned();

        if value.is_none() {
            break
        }

        let point = value.unwrap();
        let (x, y) = point;

        // get the numbers
        if board.get(y, x).unwrap().is_ascii_digit() {
            // 1. find the beginning of the digit
            let mut start_x = x;

            loop {
                // dbg!("loop1", (start_x, y));

                if start_x > 0 {
                    let prev_point = board.get(y, start_x - 1);

                    match prev_point {
                        Some(value) => {
                            if value.is_ascii_digit() {
                                start_x = start_x - 1;
                                continue;
                            } else {
                                break
                            }
                        },
                        _ => break
                    }
                } else {
                    break
                }
            }

            // 2. find the beginning, find the whole number
            let mut number_string = String::from("");

            loop {
                // dbg!("loop2", (start_x, y));
                let point = board.get(y, start_x);

                match point {
                    Some(value) => {
                        if value.is_ascii_digit() {
                            number_string.push(value.clone());

                            // remove the point from list
                            points.remove(&(start_x, y));

                            start_x += 1;
                        } else {
                            break
                        }
                    },
                    _ => break
                }
            }

            let num: u32 = number_string.parse().unwrap();

            results.push(num);

            dbg!(number_string);
        }

        points.remove(&point);
    }

    dbg!(&results);

    if results.len() == 2 {
        return results[0] * results[1]
    }

    return 0
}


pub fn part_two(input: &str) -> Option<u32> {
    let board: Vec<Vec<char>> = input.lines().map(
        |line| line.chars().map(|c| c).collect()
    ).collect();

    let board = Array2D::from_rows(&board).unwrap();

    Some(board.enumerate_row_major().filter(
        |(_, c)| { **c == '*' }
    ).map(
        |((y, x), _c)| { get_gear_ratio(&board, x, y) }
    ).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
