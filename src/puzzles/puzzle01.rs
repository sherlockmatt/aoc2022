use crate::utils::parse_usize;
use itertools::Itertools;

pub fn run(input: String) -> Vec<String> {
    let mut answers = Vec::new();

    let elves = input
        .split("\n\n")
        .map(|elf| elf.lines().map(parse_usize).sum())
        .sorted()
        .rev()
        .collect_vec();

    answers.push(format!("{}", elves.get(0).unwrap()));
    answers.push(format!("{}", elves.iter().take(3).sum::<usize>()));

    return answers;
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn example1() {
        let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"
            .to_string();
        assert_eq!(run(input), vec!["24000".to_string(), "45000".to_string()]);
    }
}
