advent_of_code::solution!(1);


pub fn part_one(input: &str) -> Option<u32> {
    let mut sum = 0;

    let lines = input.split('\n');

    for line in lines {
        let first_digit = line.chars().find(|&char| char.is_ascii_digit());
        let last_digit = line.chars().rev().find(|&char| char.is_ascii_digit());

        if first_digit.is_some() && last_digit.is_some() {
            let number: u32 = format!("{}{}", first_digit.unwrap(), last_digit.unwrap()).parse().unwrap();
            sum += number;
        } else {
            println!("Can't parse: {}", line);
        }
    }

    println!("{}", sum);

    return Some(sum);
}

fn replace_string_numbers (line: String) -> String {
    // replace every string numbers into digit numbers
    // one, two, three, four, five, six, seven, eight, and nine
    line
        // special case
        .replace("nineight", "98")
        .replace("eighthree", "83")
        .replace("eightwo", "82")
        .replace("sevenine", "79")
        .replace("fiveight", "58")
        .replace("threeight", "38")
        .replace("twone", "21")
        .replace("oneight", "18")
        // normal case
        .replace("nine", "9")
        .replace("eight", "8")
        .replace("seven", "7")
        .replace("six", "6")
        .replace("five", "5")
        .replace("four", "4")
        .replace("three", "3")
        .replace("two", "2")
        .replace("one", "1")
        // .replace("zero", "0")
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut sum = 0;

    let lines = input.split('\n');

    for l in lines {
        let line = replace_string_numbers(l.to_string());


        let first_digit = line.chars().find(|&char| char.is_ascii_digit());
        let last_digit = line.chars().rev().find(|&char| char.is_ascii_digit());

        if first_digit.is_some() && last_digit.is_some() {
            let number: u32 = format!("{}{}", first_digit.unwrap(), last_digit.unwrap()).parse().unwrap();
            sum += number;

            println!("{} -> {} -> {}", l, line, number);
        } else {
            // println!("Can't parse: {}", line);
        }
    }

    println!("{}", sum);

    return Some(sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(209));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(281));
    }
}

