use regex::Regex;

pub fn puzzle_03() -> usize {
    let data = include_str!("../data/03.txt");
    part_2(data)
}

fn part_1(input: &str) -> usize {
    let regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    regex
        .captures_iter(input)
        .map(|captures| {
            captures[1].parse::<usize>().unwrap() * captures[2].parse::<usize>().unwrap()
        })
        .sum()
}

fn part_2(input: &str) -> usize {
    let dont_do = Regex::new(r"don't\(\)[\s\S]+?(do\(\)|$)").unwrap();
    let mul = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let result = dont_do.replace_all(input, "");
    println!("{:?}", result.to_string());
    mul.captures_iter(&result)
        .map(|captures| {
            captures[1].parse::<usize>().unwrap() * captures[2].parse::<usize>().unwrap()
        })
        .sum()
}
