use itertools::Itertools;
use crate::utils::parse_usize;

pub fn run(input: &str) -> Vec<String> {
    let mut answers = Vec::new();

    let ranges = input
        .lines()
        .map(|l| {
            l.split(",")
                .map(|r| {
                    r.split("-")
                        .map(parse_usize)
                        .collect_tuple()
                        .expect("tuple 1 failed")
                })
                .collect_tuple()
                .expect("tuple 2 failed")
        })
        .collect_vec();

    answers.push(format!(
        "{}",
        ranges
            .iter()
            .filter(
                |((e11, e12), (e21, e22))| (e11 <= e21 && e22 <= e12) || (e21 <= e11 && e12 <= e22)
            )
            .count()
    ));
    answers.push(format!(
        "{}",
        ranges
            .iter()
            .filter(
                |((e11, e12), (e21, e22))| (e21 <= e11 && e11 <= e22) || (e11 <= e21 && e21 <= e12)
            )
            .count()
    ));

    return answers;
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn example1() {
        let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
        assert_eq!(run(input), vec!["2".to_string(), "4".to_string()]);
    }
}
