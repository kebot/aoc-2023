use core::ops::Range;

advent_of_code::solution!(5);

// SeedSoil
// SoilFertilizer
// FertilizerWater
// WaterLight
// LightTemperature
// TemperatureHumidity
// HumidityLocation(end)

#[derive(Debug)]
enum Category {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location
}

impl Category {
    pub fn from_str(s: &str) -> Self {
        match s {
            "seed" => Self::Seed,
            "soil" => Self::Soil,
            "fertilizer" => Self::Fertilizer,
            "water" => Self::Water,
            "light" => Self::Light,
            "temperature" => Self::Temperature,
            "humidity" => Self::Humidity,
            "location" => Self::Location,
            _ => Self::Seed
        }
    }
}

#[derive(Debug)]
struct AbRange {
    dest_start: usize,
    src_start: usize,
    length: usize
}

impl AbRange {
    pub fn from_nums (nums: Vec<usize>) -> Self {
        return AbRange {
            dest_start: nums[0],
            src_start: nums[1],
            length: nums[2]
        }
    }
}

#[derive(Debug)]
struct AbMap {
    // try ... catch ...
    from: Category,
    to: Category,
    ranges: Vec<AbRange>
}

impl AbMap {
    fn find_dest (&self, s: usize) -> usize {
        for range in &self.ranges {
            if s >= range.src_start && s < (range.src_start + range.length) {
                // dbg!(range);
                // matched
                return s - range.src_start + range.dest_start
            }
        }

        return s
    }
}


pub fn part_one(input: &str) -> Option<u32> {
    let mut sections_iter = input.split("\n\n");

    // seeds:
    let seeds: Vec<usize> = sections_iter
        .next()
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|c| c.parse().unwrap())
        .collect();

    let maps: Vec<AbMap> = sections_iter.map(|section| {
        let mut abmap = AbMap {
            from: Category::Seed,
            to: Category::Seed,
            ranges: vec![]
        };

        for (i, line) in section.lines().enumerate() {
            if i == 0 {
                let a2b: Vec<&str> = line.split(' ').nth(0).unwrap().split('-').collect();

                abmap.from = Category::from_str(a2b[0]);
                abmap.to = Category::from_str(a2b[2]);

                continue
            }

            // split whitespace
            let nums: Vec<usize> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();

            abmap.ranges.push(AbRange::from_nums(nums));
        };

        abmap
    }).collect();

    // loop over seeds and find the relative LOCATION
    let value: u32 = seeds
        .iter()
        .map(|seed| {
            let value = maps.iter()
                .fold(
                    seed.clone(),
                    |acc, x| x.find_dest(acc)
                );
            value.clone()
        })
        .min()
        .clone()
        .unwrap()
        .try_into()
        .unwrap();

    return Some(
        value
    )
}


pub fn part_two(input: &str) -> Option<u32> {
    let mut sections_iter = input.split("\n\n");

    // seeds:
    let ranges: Vec<Range<usize>> = sections_iter
        .next()
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|c| c.parse().unwrap())
        .collect::<Vec<usize>>()
        .chunks(2)
        .map(|i| {
            let start = i[0];
            let length = i[1];

            (start.clone()..(start+length).clone()).into_iter()
        })
        .collect();

    let maps: Vec<AbMap> = sections_iter.map(|section| {
        let mut abmap = AbMap {
            from: Category::Seed,
            to: Category::Seed,
            ranges: vec![]
        };

        for (i, line) in section.lines().enumerate() {
            if i == 0 {
                let a2b: Vec<&str> = line.split(' ').nth(0).unwrap().split('-').collect();

                abmap.from = Category::from_str(a2b[0]);
                abmap.to = Category::from_str(a2b[2]);

                continue
            }

            // split whitespace
            let nums: Vec<usize> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();

            abmap.ranges.push(AbRange::from_nums(nums));
        };

        abmap
    }).collect();

    let mut min_value = usize::max_value();

    for range in ranges {
        dbg!(&range);

        for seed in range {
            let value = maps.iter().fold(seed.clone(), |acc, x| x.find_dest(acc));
            min_value = min_value.min(value);
        }
    }

    return Some(
        min_value.try_into().unwrap()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
