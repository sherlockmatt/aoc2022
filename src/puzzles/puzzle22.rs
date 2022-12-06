pub fn run(_input: &str) -> Vec<String> {
    let mut answers = Vec::new();

    answers.push(format!("Not implemented yet."));

    return answers;
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn example1() {
        let input = "";
        assert_eq!(run(input), vec!["Not implemented yet.".to_string()]);
    }
}
