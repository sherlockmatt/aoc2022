use crate::utils::parse_usize;
use std::collections::HashMap;

pub fn run(input: &str) -> Vec<String> {
    let mut answers = Vec::new();

    let mut cursor = "/".to_string();
    let mut directories = HashMap::new();
    directories.insert("/".to_string(), Vec::new());

    // Assuming inputs always start with `$ cd /`, just skip it because it causes issues
    for line in input.lines().skip(1) {
        if line.starts_with("$") {
            if !line.starts_with("$ cd ") {
                continue;
            } else {
                if line.starts_with("$ cd ..") {
                    cursor = cursor
                        .rsplit_once("/")
                        .expect("no dirs in cursor")
                        .0
                        .to_string();
                } else {
                    cursor = concat_path(&cursor, line.trim_start_matches("$ cd "));
                }
            }
        } else {
            if line.starts_with("dir ") {
                directories.insert(
                    concat_path(&cursor, line.trim_start_matches("dir ")),
                    Vec::new(),
                );
            } else {
                let (size, _file) = line.split_once(" ").expect("no space in file line");
                directories
                    .get_mut(&cursor)
                    .expect(&format!("missing dir at {cursor}"))
                    .push(parse_usize(size));
            }
        }
    }

    let dir_sizes: HashMap<String, usize> = directories
        .keys()
        .map(|d| (d.clone(), get_dir_size(&directories, d)))
        .collect();

    answers.push(format!(
        "{}",
        dir_sizes.values().filter(|n| **n <= 100_000).sum::<usize>()
    ));

    const MAX_SIZE: usize = 70_000_000;
    const REQ_SIZE: usize = MAX_SIZE - 30_000_000; // Need 30M free space
    let cur_size = dir_sizes.get("/").unwrap();
    let remove_target = cur_size - REQ_SIZE;

    answers.push(format!(
        "{}",
        dir_sizes
            .values()
            .filter(|s| **s >= remove_target)
            .min()
            .expect("no dir to remove")
    ));

    return answers;
}

fn concat_path(path: &str, dir: &str) -> String {
    format!("{}/{}", path.trim_end_matches("/"), dir)
}

fn get_dir_size(directories: &HashMap<String, Vec<usize>>, dir: &str) -> usize {
    directories
        .iter()
        .filter(|(d, _sizes)| d.starts_with(dir))
        .map(|(_d, sizes)| sizes.iter().sum::<usize>())
        .sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn example1() {
        let input = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";
        assert_eq!(
            run(input),
            vec!["95437".to_string(), "24933642".to_string()]
        );
    }
}
