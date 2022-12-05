pub fn run(input: String) -> Vec<String> {
    let mut answers = Vec::new();

    answers.push(format!(
        "{}",
        input
            .lines()
            .map(|l| match l {
                "A X" => 4,
                "A Y" => 8,
                "A Z" => 3,
                "B X" => 1,
                "B Y" => 5,
                "B Z" => 9,
                "C X" => 7,
                "C Y" => 2,
                "C Z" => 6,
                other => panic!("Unknown input: {other}"),
            })
            .sum::<usize>()
    ));

    answers.push(format!(
        "{}",
        input
            .lines()
            .map(|l| match l {
                "A X" => 3,
                "A Y" => 4,
                "A Z" => 8,
                "B X" => 1,
                "B Y" => 5,
                "B Z" => 9,
                "C X" => 2,
                "C Y" => 6,
                "C Z" => 7,
                other => panic!("{}", other),
            })
            .sum::<usize>()
    ));

    return answers;
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn example1() {
        let input = "A Y
B X
C Z"
        .to_string();
        assert_eq!(run(input), vec!["15".to_string(), "12".to_string()]);
    }
}
