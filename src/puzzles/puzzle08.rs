use crate::utils::{parse_usize, Pos};
use itertools::Itertools;
use std::cmp::max;
use std::collections::HashMap;

pub fn run(input: &str) -> Vec<String> {
    let mut answers = Vec::new();

    let width = input.find('\n').expect("No newline");
    let height = input.trim().matches('\n').count() + 1;

    let trees: HashMap<Pos, usize> = input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars()
                .enumerate()
                .map(|(x, c)| (Pos::new(x, y), parse_usize(&c.to_string())))
                .collect_vec()
        })
        .collect();

    let mut visible: HashMap<Pos, bool> = (0..height)
        .flat_map(|y| {
            (0..height)
                .map(|x| {
                    (
                        Pos::new(x, y),
                        x == 0 || y == 0 || x == width - 1 || y == height - 1, // Outer trees are always visible
                    )
                })
                .collect_vec()
        })
        .collect();

    {
        let mut y_max_vec = (0..width)
            .map(|x| {
                trees
                    .get(&Pos::new(x, 0))
                    .expect(&format!("No tree at ({x}, 0)"))
                    .clone()
            })
            .collect_vec();
        for y in 1..(height - 1) {
            let mut x_max = trees
                .get(&Pos::new(0, y))
                .expect(&format!("No tree at (0, {y})"))
                .clone();
            for x in 1..(width - 1) {
                let h = trees
                    .get(&Pos::new(x, y))
                    .expect(&format!("No tree at ({x}, {y})"))
                    .clone();
                let y_max = y_max_vec.get_mut(x).unwrap();
                if h > x_max || h > *y_max {
                    *visible
                        .get_mut(&Pos::new(x, y))
                        .expect(&format!("No visible at ({x}, {y})")) = true;
                }
                x_max = max(x_max, h);
                *y_max = max(*y_max, h);
            }
        }
    }

    {
        let mut y_max_vec = (0..width)
            .map(|x| {
                trees
                    .get(&Pos::new(x, height - 1))
                    .expect(&format!("No tree at ({x}, {})", height - 1))
                    .clone()
            })
            .collect_vec();
        for y in (1..(height - 1)).rev() {
            let mut x_max = trees
                .get(&Pos::new(width - 1, y))
                .expect(&format!("No tree at ({}, {y})", width - 1))
                .clone();
            for x in (1..(width - 1)).rev() {
                let h = trees
                    .get(&Pos::new(x, y))
                    .expect(&format!("No tree at ({x}, {y})"))
                    .clone();
                let y_max = y_max_vec.get_mut(x).unwrap();
                if h > x_max || h > *y_max {
                    *visible
                        .get_mut(&Pos::new(x, y))
                        .expect(&format!("No visible at ({x}, {y})")) = true;
                }
                x_max = max(x_max, h);
                *y_max = max(*y_max, h);
            }
        }
    }

    answers.push(format!("{}", visible.values().filter(|&&v| v).count()));

    // Ignore the outer ring since one direction will be zero, and multiplying it makes its score zero too
    answers.push(format!(
        "{}",
        (1..(height - 1))
            .flat_map(|y| {
                (1..(width - 1))
                    .map(|x| {
                        let mut score = 1;
                        let this_height = trees
                            .get(&Pos::new(x, y))
                            .expect(&format!("No tree at ({x}, {y})"))
                            .clone();

                        // Look left
                        {
                            let mut x2 = x - 1;
                            while x2 > 0 {
                                let h = trees
                                    .get(&Pos::new(x2, y))
                                    .expect(&format!("No tree at ({x2}, {y})"))
                                    .clone();
                                if h >= this_height {
                                    break;
                                } else {
                                    x2 -= 1;
                                }
                            }
                            score *= x - x2;
                        }

                        // Look right
                        {
                            let mut x2 = x + 1;
                            while x2 < width - 1 {
                                let h = trees
                                    .get(&Pos::new(x2, y))
                                    .expect(&format!("No tree at ({x2}, {y})"))
                                    .clone();
                                if h >= this_height {
                                    break;
                                } else {
                                    x2 += 1;
                                }
                            }
                            score *= x2 - x;
                        }

                        // Look up
                        {
                            let mut y2 = y - 1;
                            while y2 > 0 {
                                let h = trees
                                    .get(&Pos::new(x, y2))
                                    .expect(&format!("No tree at ({x}, {y2})"))
                                    .clone();
                                if h >= this_height {
                                    break;
                                } else {
                                    y2 -= 1;
                                }
                            }
                            score *= y - y2;
                        }

                        // Look down
                        {
                            let mut y2 = y + 1;
                            while y2 < height - 1 {
                                let h = trees
                                    .get(&Pos::new(x, y2))
                                    .expect(&format!("No tree at ({x}, {y2})"))
                                    .clone();
                                if h >= this_height {
                                    break;
                                } else {
                                    y2 += 1;
                                }
                            }
                            score *= y2 - y;
                        }

                        score
                    })
                    .collect_vec()
            })
            .max()
            .unwrap()
    ));

    return answers;
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn example1() {
        let input = "30373
25512
65332
33549
35390";
        assert_eq!(run(input), vec!["21".to_string(), "8".to_string()]);
    }
}
