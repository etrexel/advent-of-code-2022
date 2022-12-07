use crate::solution::day_07::{parse_input, Directory};
use std::cell::RefCell;
use std::rc::Rc;

pub(crate) fn solve(input: &str) -> Result<String, anyhow::Error> {
    let root = parse_input(input)?;
    let free_space = 70000000 - root.borrow().size;
    let space_to_free = 30000000 - free_space;
    Ok(find_directory_to_delete(&root, space_to_free).to_string())
}

fn find_directory_to_delete(directory: &Rc<RefCell<Directory>>, space_to_free: u32) -> u32 {
    let dir = directory.borrow();
    if dir.size < space_to_free {
        return u32::MAX;
    }
    let mut smallest_dir = dir.size;
    for child in &dir.directories {
        let smallest_child_size = find_directory_to_delete(child.1, space_to_free);
        if smallest_child_size < smallest_dir {
            smallest_dir = smallest_child_size;
        }
    }
    smallest_dir
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
        assert_eq!("24933642", solve(input).expect("should return result"));
    }
}
