use std::collections::HashSet;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    // Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    let scores: u32 = input.lines().map(|line| {
        let nums: Vec<HashSet<u32>> = line.split(':')
            .nth(1)
            .unwrap()
            .split('|').map(|num_string| {
                num_string
                    .split_whitespace()
                    .map(|num_str|
                         num_str.parse::<u32>().unwrap()
                    ).collect()
            })
            .collect();

        if nums.len() == 2 {
            let set1 = &nums[0];
            let set2 = &nums[1];

            // let is: HashSet<u32> = set1.intersection(set2).map(|i| i.clone()).collect();
            // dbg!(is);
            //
            let count = set1.intersection(set2).count();

            if count > 0 {
                return 2_u32.pow((count - 1).try_into().unwrap())
            }
        }

        return 0
    }).sum();


    return Some(scores)
}

pub fn part_two(input: &str) -> Option<u32> {
    // Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    let scores: Vec<usize> = input.lines().map(|line| {
        let nums: Vec<HashSet<u32>> = line.split(':')
            .nth(1)
            .unwrap()
            .split('|').map(|num_string| {
                num_string
                    .split_whitespace()
                    .map(|num_str|
                         num_str.parse::<u32>().unwrap()
                    ).collect()
            })
            .collect();

        if nums.len() == 2 {
            let set1 = &nums[0];
            let set2 = &nums[1];

            return set1.intersection(set2).count();
        }

        return 0
    }).collect();

    // number of each cards
    let mut counts = vec![1; scores.len()];

    for i in 0..scores.len() {
        let count = counts[i].clone();
        let score = scores[i].clone();

        if scores[i] > 0 {
            for a in 1..=score {
                let r = i + a;

                if r < scores.len() {
                    counts[r] += count;
                }
            }
        }
    }

    return Some(counts.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
