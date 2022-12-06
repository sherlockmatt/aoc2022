use itertools::Itertools;
use std::collections::HashSet;

pub fn run(input: &str) -> Vec<String> {
    let mut answers = Vec::new();

    answers.push(format!("{}", first_unique_window::<4>(input)));

    answers.push(format!("{}", first_unique_window::<14>(input)));

    return answers;
}

fn first_unique_window<const LENGTH: usize>(signal: &str) -> usize {
    signal
        .chars()
        .collect_vec()
        .windows(LENGTH)
        .position(|arr| arr.iter().cloned().collect::<HashSet<char>>().len() == LENGTH)
        .expect("No window found")
        + LENGTH // The puzzle wants the index of the end of the window, so add the width of the window
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn example1() {
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        assert_eq!(run(input), vec!["7".to_string(), "19".to_string()]);
    }

    #[test]
    fn example2() {
        let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        assert_eq!(run(input), vec!["5".to_string(), "23".to_string()]);
    }

    #[test]
    fn example3() {
        let input = "nppdvjthqldpwncqszvftbrmjlhg";
        assert_eq!(run(input), vec!["6".to_string(), "23".to_string()]);
    }

    #[test]
    fn example4() {
        let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        assert_eq!(run(input), vec!["10".to_string(), "29".to_string()]);
    }

    #[test]
    fn example5() {
        let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        assert_eq!(run(input), vec!["11".to_string(), "26".to_string()]);
    }
}
