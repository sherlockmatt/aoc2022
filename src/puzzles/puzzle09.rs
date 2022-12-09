use crate::utils::parse_usize;
use std::collections::HashSet;

pub fn run(input: &str) -> Vec<String> {
    let mut answers = Vec::new();

    let moves = input
        .lines()
        .map(|l| l.split_once(" ").expect("No space in line"))
        .map(|(d, c)| (d, parse_usize(c)))
        .collect();

    answers.push(format!("{}", simulate_rope::<2>(&moves)));
    answers.push(format!("{}", simulate_rope::<10>(&moves)));

    return answers;
}

fn simulate_rope<const LENGTH: usize>(moves: &Vec<(&str, usize)>) -> usize {
    let mut pos = vec![(0, 0); LENGTH];
    let mut tail_visited = HashSet::new();
    tail_visited.insert(pos[LENGTH - 1]);

    for (dir, count) in moves.iter() {
        for _ in 0..*count {
            head_move(dir, &mut pos[0]);
            for i in 0..(LENGTH - 1) {
                tail_move(&pos[i].clone(), &mut pos[i + 1]);
            }
            tail_visited.insert(pos[LENGTH - 1]);
        }
    }

    tail_visited.len()
}

fn head_move(dir: &str, mut head_pos: &mut (isize, isize)) -> () {
    match dir {
        "U" => head_pos.1 += 1,
        "D" => head_pos.1 -= 1,
        "L" => head_pos.0 -= 1,
        "R" => head_pos.0 += 1,
        other => panic!("Invalid direction {}", other.clone()),
    };
}

fn tail_move(head_pos: &(isize, isize), mut tail_pos: &mut (isize, isize)) -> () {
    let xdiff = head_pos.0 - tail_pos.0;
    let ydiff = head_pos.1 - tail_pos.1;

    if xdiff.abs() == 2 || ydiff.abs() == 2 {
        tail_pos.0 += xdiff.signum();
        tail_pos.1 += ydiff.signum();
    }
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn example1() {
        let input = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
        assert_eq!(run(input), vec!["13".to_string(), "1".to_string()]);
    }

    #[test]
    fn example2() {
        let input = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";
        // This example only has a given answer for part two, so only test that part
        assert_eq!(*run(input).get(1).unwrap(), "36".to_string());
    }
}
