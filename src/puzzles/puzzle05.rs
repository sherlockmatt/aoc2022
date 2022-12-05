use crate::utils::parse_usize;
use itertools::Itertools;

pub fn run(input: String) -> Vec<String> {
    let mut answers = Vec::new();

    let (input_crates, input_moves): (&str, &str) =
        input.split_once("\n\n").expect("No double newline");

    let mut crates = Vec::new();

    for l in input_crates.lines().rev().skip(1) {
        for (i, c) in l.chars().skip(1).step_by(4).enumerate() {
            if crates.len() <= i {
                crates.push(Vec::new());
            }
            if c.is_alphabetic() {
                crates[i].push(c);
            }
        }
    }

    let mut crates2 = crates.clone();

    let moves = input_moves
        .lines()
        .map(|l| {
            l.split(" ")
                .tuples()
                .map(|(_, q, _, f, _, t)| (parse_usize(q), parse_usize(f) - 1, parse_usize(t) - 1))
                .next()
                .expect("too many moves")
        })
        .collect_vec();

    // Do part 1
    for (quant, from, to) in moves.clone() {
        for _ in 0..quant {
            let c = crates[from].pop().expect("missing crate on from");
            crates[to].push(c);
        }
    }

    // Do part 2
    for (quant, from, to) in moves {
        let pos = crates2[from].len() - quant;
        let substack = crates2[from][pos..].to_vec();
        crates2[to].extend(substack);
        crates2[from].truncate(pos);
    }

    answers.push(format!("{}", get_top_crates(crates)));
    answers.push(format!("{}", get_top_crates(crates2)));

    return answers;
}

fn get_top_crates(crates: Vec<Vec<char>>) -> String {
    crates
        .iter()
        .map(|c| c.last().expect("no crate on stack at end"))
        .join("")
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn example1() {
        let input = "    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"
            .to_string();
        assert_eq!(run(input), vec!["CMZ".to_string(), "MCD".to_string()]);
    }
}
