use itertools::Itertools;
use std::collections::HashSet;

pub fn run(input: String) -> Vec<String> {
    let mut answers = Vec::new();

    answers.push(format!(
        "{}",
        input
            .chars()
            .tuple_windows()
            .position(|(a, b, c, d)| HashSet::from([a, b, c, d]).len() == 4)
            .expect("No marker")
            + 4 // The puzzle wants the index of the end of the marker, so add the width of the marker
    ));

    // .tuple_windows() has a max size of 12, so do some filth to hack in a 14-window
    answers.push(format!(
        "{}",
        input
            .chars()
            .tuple_windows()
            .tuple_windows()
            .position(
                |(
                    (a, b, c, d, e, f, g, h, i, j, k, l),
                    (_, _, _, _, _, _, _, _, _, _, _, m),
                    (_, _, _, _, _, _, _, _, _, _, _, n),
                )| HashSet::from([a, b, c, d, e, f, g, h, i, j, k, l, m, n]).len()
                    == 14
            )
            .expect("No message")
            + 14 // The puzzle wants the index of the end of the marker, so add the width of the marker
    ));

    return answers;
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn example1() {
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string();
        assert_eq!(run(input), vec!["7".to_string(), "19".to_string()]);
    }

    #[test]
    fn example2() {
        let input = "bvwbjplbgvbhsrlpgdmjqwftvncz".to_string();
        assert_eq!(run(input), vec!["5".to_string(), "23".to_string()]);
    }

    #[test]
    fn example3() {
        let input = "nppdvjthqldpwncqszvftbrmjlhg".to_string();
        assert_eq!(run(input), vec!["6".to_string(), "23".to_string()]);
    }

    #[test]
    fn example4() {
        let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string();
        assert_eq!(run(input), vec!["10".to_string(), "29".to_string()]);
    }

    #[test]
    fn example5() {
        let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string();
        assert_eq!(run(input), vec!["11".to_string(), "26".to_string()]);
    }
}
