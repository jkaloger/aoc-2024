use std::collections::HashMap;

pub fn puzzle_04() -> usize {
    let data = include_str!("../data/04.txt");

    part_2(data)
}

fn part_1(data: &str) -> usize {
    let matrix = data.lines().collect::<Vec<&str>>();

    let mut sum = 0;
    println!("{:?}", matrix[0][0..4].to_string());

    for y in 0..matrix.len() {
        for x in 0..matrix[y].len() {
            sum += find(&matrix, x, y);
        }
    }

    sum
}

fn part_2(data: &str) -> usize {
    let mas_match: [char; 2] = ['M', 'S'];
    let diagonals: [[(isize, isize); 2]; 4] = [
        [(-1, -1), (1, 1)],
        [(1, 1), (-1, -1)],
        [(1, -1), (-1, 1)],
        [(-1, 1), (1, -1)],
    ];
    let grid = data
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, value)| ((x as isize, y as isize), value))
        })
        .collect::<HashMap<(isize, isize), char>>();

    grid.iter()
        .filter(|(_, val)| **val == 'A')
        .filter(|((x, y), _)| {
            diagonals
                .iter()
                .map(|diagonal| {
                    diagonal
                        .iter()
                        // grab the diagonal values starting from x,y
                        .map(|&coord| grid.get(&(*x + coord.0, *y + coord.1)))
                        .enumerate()
                        // make sure all the diagonals match 'M' or 'S'
                        .all(|(dir, maybe_mas)| mas_match.get(dir) == maybe_mas)
                })
                .filter(|b| *b)
                // both diagonals must be satisfied
                .count()
                == 2
        })
        .count()
}

fn find(matrix: &[&str], x: usize, y: usize) -> usize {
    println!("{:?}, {:?}", x, y);
    let mut count = 0;
    if x < matrix[y].len() - 4 && (matrix[y][x..x + 4].eq("XMAS") || matrix[y][x..x + 4].eq("SAMX"))
    {
        count += 1;
    }

    if y < matrix.len() - 4 {
        let word = matrix[y..y + 4]
            .iter()
            .map(|&c| c.chars().nth(x).unwrap().to_string())
            .collect::<Vec<_>>()
            .join("");
        if word == "XMAS" || word == "SAMX" {
            count += 1;
        }
    }

    if y >= 3 && x < matrix[y].len() - 4 {
        let word = matrix[y - 3..=y]
            .iter()
            .enumerate()
            .map(|(i, &c)| c.chars().nth(x + i).unwrap().to_string())
            .collect::<Vec<_>>()
            .join("");
        println!("{:?}", word);
        if word == "XMAS" || word == "SAMX" {
            count += 1;
        }
    }

    if y < matrix.len() - 4 && x < matrix[y].len() - 4 {
        let word = matrix[y..y + 4]
            .iter()
            .enumerate()
            .map(|(i, &c)| c.chars().nth(x + i).unwrap().to_string())
            .collect::<Vec<_>>()
            .join("");
        if word == "XMAS" || word == "SAMX" {
            count += 1;
        }
    }

    count
}
