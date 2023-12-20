advent_of_code::solution!(6);

/*
Time:      7  15   30
Distance:  9  40  200
*/

fn parse (input: &str) -> (Vec<u32>, Vec<u32>) {
    dbg!(input);

    let mut iter = input.lines().map(|line| {
        let arr: Vec<u32> = line
            .split_whitespace()
            .skip(1)
            .map(|n| n.parse().unwrap())
            .collect();
        return arr;
    });

    return (
        iter.next().unwrap(),
        iter.next().unwrap()
    )
}


pub fn part_one(input: &str) -> Option<u32> {
    let (time, distance) = parse(input);

    time.iter().zip(distance.iter()).map(|(time, distance)| {
        let mut ways: u32 = 0;

        for hold_time in 0..*time {
            let speed = hold_time;
            let travel_time = time - hold_time;

            if speed * travel_time > *distance {
                ways += 1;
            }
        }

        ways
    }).reduce(|acc, item| acc * item)
}

pub fn part_two(_input: &str) -> Option<u32> {
    let time: u64 = 38677673;
    let distance: u64 = 234_1027_1157_1236;

    let mut ways: u32 = 0;

    for hold_time in 0..time {
        let speed = hold_time;
        let travel_time = time - hold_time;

        if speed * travel_time > distance {
            ways += 1;
        }
    }

    Some(ways)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
