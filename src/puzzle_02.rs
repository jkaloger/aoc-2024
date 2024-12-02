pub fn puzzle_02() -> usize {
    let data = include_str!("../data/02.txt");
    let input = data
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|c| c.parse::<usize>().unwrap())
                .collect()
        })
        .collect();

    part_2(input)
}

fn is_safe(report: &[usize]) -> bool {
    if !report.windows(2).all(|w| w[0] >= w[1]) && !report.windows(2).all(|w| w[0] <= w[1]) {
        return false;
    }
    report.windows(2).all(|w| {
        let diff = w[0].abs_diff(w[1]);
        (1..=3).contains(&diff)
    })
}

fn part_1(input: Vec<Vec<usize>>) -> usize {
    input
        .iter()
        .filter(|report| is_safe(report))
        .collect::<Vec<_>>()
        .len()
}

fn part_2(input: Vec<Vec<usize>>) -> usize {
    input
        .iter()
        .filter(|report| {
            if is_safe(report) {
                return true;
            }

            for i in 0..report.len() {
                let mut dampened_report = report.to_vec().clone();
                dampened_report.remove(i);
                if is_safe(&dampened_report) {
                    return true;
                }
            }

            false
        })
        .collect::<Vec<_>>()
        .len()
}
