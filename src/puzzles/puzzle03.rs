use itertools::Itertools;
use std::collections::HashSet;

pub fn run(input: String) -> Vec<String> {
    let mut answers = Vec::new();

    answers.push(format!(
        "{}",
        input
            .lines()
            .map(|l| {
                let s1: HashSet<char> = l.chars().take(l.len() / 2).collect();
                let s2: HashSet<char> = l.chars().skip(l.len() / 2).collect();
                char_to_priority(s1.intersection(&s2).next().expect("No duplicate item"))
            })
            .sum::<u32>()
    ));

    answers.push(format!(
        "{}",
        input
            .lines()
            .tuples()
            .map(|(a, b, c)| {
                let s1: HashSet<char> = a.chars().collect();
                let s2: HashSet<char> = b.chars().collect();
                let s3: HashSet<char> = c.chars().collect();
                char_to_priority(
                    s1.intersection(&s2)
                        .map(|&c| c)
                        .collect::<HashSet<char>>()
                        .intersection(&s3)
                        .next()
                        .expect("No common item"),
                )
            })
            .sum::<u32>()
    ));

    return answers;
}

fn char_to_priority(c: &char) -> u32 {
    match c {
        &c @ 'a'..='z' => c as u32 - 96,
        &c @ 'A'..='Z' => c as u32 - 38,
        other => panic!("Weird character {other:?}"),
    }
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn example1() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"
            .to_string();
        assert_eq!(run(input), vec!["157".to_string(), "70".to_string()]);
    }
}
