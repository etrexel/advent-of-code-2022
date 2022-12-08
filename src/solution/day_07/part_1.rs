// TODO: docs

use crate::solution::day_07::{parse_input, Directory};
use std::cell::RefCell;
use std::rc::Rc;

pub(crate) fn solve(input: &str) -> Result<String, anyhow::Error> {
    let root = parse_input(input)?;
    Ok(sum_sizes(&root).to_string())
}

fn sum_sizes(directory: &Rc<RefCell<Directory>>) -> u32 {
    let dir = directory.borrow();
    let mut total = 0;
    for child in &dir.directories {
        total += sum_sizes(child.1)
    }
    if dir.size < 100000 {
        total += dir.size;
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
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
        assert_eq!("95437", solve(input).expect("should return result"));
    }
}
