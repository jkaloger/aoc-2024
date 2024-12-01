use std::collections::HashMap;

pub fn puzzle_01() -> u64 {
    let data = include_str!("../data/01.txt");
    let (left, right): (Vec<u64>, Vec<u64>) = data
        .lines()
        .map(|line| {
            let mut split = line.split_whitespace();
            (
                split.next().unwrap().parse::<u64>().unwrap(),
                split.next().unwrap().parse::<u64>().unwrap(),
            )
        })
        .unzip();

    // part_1(left, right)
    part_2(left, right)
}

fn part_1(mut left: Vec<u64>, mut right: Vec<u64>) -> u64 {
    left.sort();
    right.sort();

    left.iter().zip(right).map(|(l, r)| l.abs_diff(r)).sum()
}

fn part_2(left: Vec<u64>, right: Vec<u64>) -> u64 {
    let frequency_map = right.iter().fold(HashMap::<_, u64>::new(), |mut map, num| {
        *map.entry(num).or_insert(0) += 1;
        map
    });

    left.iter()
        .map(|n| {
            if let Some(freq) = frequency_map.get(n) {
                return n * freq;
            }
            0
        })
        .sum()
}
