use std::str::FromStr;
use std::cmp;

advent_of_code::solution!(2);

#[derive(Debug)]
struct Cubes {
    red: u32,
    green: u32,
    blue: u32
}

impl FromStr for Cubes {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // 3 blue, 4 red
        let mut cubes = Cubes {
            red: 0,
            green: 0,
            blue: 0
        };


        // example parts: ['3 blue', '4 red', '6 green']
        let parts: Vec<&str> = s.split(',').collect();

        for part in parts {
            let n_color: Vec<&str> = part.trim().split_whitespace().collect();

            if n_color.len() != 2 {
                return Err(())
            }

            let number: u32 = n_color[0].parse().unwrap();

            if n_color[1] == "red" {
                cubes.red = number
            } else if n_color[1] == "green" {
                cubes.green = number
            } else if n_color[1] == "blue" {
                cubes.blue = number
            }
        }

        // println!("Cubes.from_str {}", s);

        Ok(cubes)
    }
}

impl Cubes {
    pub fn power (&self) -> u32 {
        self.red * self.green * self.blue
    }
}

#[derive(Debug)]
struct Game {
    id: u32,
    cubes: Vec<Cubes>
}

impl FromStr for Game {
    type Err = ();

    // Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(": ").collect();

        if parts.len() != 2 {
            return Err(());
        }

        let id = parts[0].trim().split_whitespace().nth(1).unwrap().parse().unwrap();

        let cubes: Vec<Cubes> = parts[1]
            .split("; ")
            .map(|s| s.parse().unwrap())
            .collect();

        Ok(Game { id, cubes })
    }
}

impl Game {
    /**
     * check if the game is possible with the given number of balls
     */
    pub fn is_possible(&self, red: u32, green: u32, blue: u32) -> bool {
        for cube in self.cubes.iter() {
            if cube.red <= red && cube.green <= green && cube.blue <= blue {
                continue
            } else {
                return false
            }
        }

        true
    }

    pub fn fewest_cubes (&self) -> Cubes {
        let result = Cubes {
            red: 0,
            green: 0,
            blue: 0
        };

        self.cubes.iter().fold(result, |mut acc, e| {
            acc.red = cmp::max(acc.red, e.red);
            acc.green = cmp::max(acc.green, e.green);
            acc.blue = cmp::max(acc.blue, e.blue);

            return acc
        })
    }
}

fn parse_input (input: &str) -> Vec<Game> {
    let games: Vec<Game> = input
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    // println!("{:?}", games);

    games
}

pub fn part_one(input: &str) -> Option<u32> {
    let sum:u32 = parse_input(input)
        .iter()
        .map(|game| {
            if game.is_possible(12, 13, 14) {
                game.id
            } else {
                0
            }
        })
        .sum();

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    // 56332 too high
    let result = parse_input(input)
        .iter()
        .map(|game| game.fewest_cubes().power())
        .sum();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
