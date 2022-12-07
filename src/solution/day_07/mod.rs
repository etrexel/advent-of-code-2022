use anyhow::{anyhow, Context};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::{Rc, Weak};

pub(crate) mod part_1;
pub(crate) mod part_2;

struct Directory {
    parent: Weak<RefCell<Directory>>,
    directories: HashMap<String, Rc<RefCell<Directory>>>,
    files: HashMap<String, u32>,
    size: u32,
}

fn new_directory() -> Directory {
    Directory {
        parent: Weak::new(),
        directories: Default::default(),
        files: Default::default(),
        size: 0,
    }
}

fn add_directory(directory: &Rc<RefCell<Directory>>, name: &str) -> Result<(), anyhow::Error> {
    if directory.borrow().directories.contains_key(name) {
        return Err(anyhow!(
            "current directory already contains directory: {}",
            name
        ));
    }
    let new_dir = Rc::new(RefCell::new(Directory {
        parent: Rc::downgrade(directory),
        directories: Default::default(),
        files: Default::default(),
        size: 0,
    }));
    directory
        .borrow_mut()
        .directories
        .insert(name.to_string(), new_dir);
    Ok(())
}

fn add_file(
    directory: &Rc<RefCell<Directory>>,
    name: &str,
    size: u32,
) -> Result<(), anyhow::Error> {
    if directory.borrow().files.contains_key(name) {
        return Err(anyhow!("current directory already contains file: {}", name));
    }
    directory.borrow_mut().files.insert(name.to_string(), size);
    Ok(())
}

fn change_directory(
    directory: &Rc<RefCell<Directory>>,
    name: &str,
) -> Result<Rc<RefCell<Directory>>, anyhow::Error> {
    if name == ".." {
        return match directory.borrow().parent.upgrade() {
            Some(parent) => Ok(parent),
            None => Ok(directory.clone()),
        };
    }
    match directory.borrow().directories.get(name) {
        Some(dir) => Ok(dir.clone()),
        None => Err(anyhow!(
            "current directory does not contain directory: {}",
            name
        )),
    }
}

fn calculate_size(directory: &Rc<RefCell<Directory>>) -> u32 {
    let mut total = 0;
    for dir in &directory.borrow().directories {
        total += calculate_size(dir.1)
    }
    for file in &directory.borrow().files {
        total += file.1;
    }
    directory.borrow_mut().size = total;
    total
}

fn parse_input(input: &str) -> Result<Rc<RefCell<Directory>>, anyhow::Error> {
    let root = Rc::new(RefCell::new(new_directory()));
    let mut current_directory = root.clone();
    for line in input.split('\n') {
        // line is a command
        if line.starts_with('$') {
            let parts = line.split(' ').collect::<Vec<&str>>();
            if parts[1] == "cd" {
                match parts[2] {
                    "/" => current_directory = root.clone(),
                    _ => current_directory = change_directory(&current_directory, parts[2])?,
                }
            }
            continue;
        }
        // line is from ls
        let parts = line.split(' ').collect::<Vec<&str>>();
        if parts[0] == "dir" {
            add_directory(&current_directory, parts[1])?
        } else {
            let size = parts[0]
                .parse::<u32>()
                .with_context(|| format!("could not parse token to u32: {}", parts[0]))?;
            add_file(&current_directory, parts[1], size)?;
        }
    }
    calculate_size(&root);
    Ok(root)
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
        let root = parse_input(input).expect("should return result");
        assert_eq!(48381165, root.borrow().size);
    }
}
