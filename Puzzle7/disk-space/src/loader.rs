use crate::fs::{DirectoryLink, File};

use super::fs::Directory;
use std::fs;
use std::rc::Rc;

#[derive(Debug, PartialEq)]
enum Command {
    Cd(String),
    Ls,
    File(usize, String),
    Dir(String),
}

fn parse_commands(filename: &str) -> Vec<Command> {
    let command_file = fs::read_to_string(filename).unwrap();
    let mut command_list = vec![];
    for line in command_file.lines() {
        if line.starts_with("$ cd") {
            let cd_dir = line.replace("$ cd", "").trim().to_string();
            command_list.push(Command::Cd(cd_dir));
        } else if line.starts_with("$ ls") {
            command_list.push(Command::Ls);
        } else if line.starts_with("dir ") {
            let dir = line.replace("dir ", "").trim().to_string();
            command_list.push(Command::Dir(dir));
        } else {
            // todo: add error check
            let file = line.split_whitespace().collect::<Vec<_>>();
            command_list.push(Command::File(
                file[0].parse::<usize>().unwrap(),
                file[1].to_string(),
            ));
        }
    }
    command_list
}

pub fn load_directory_tree(filename: &str) -> Vec<DirectoryLink> {
    let mut dirs = vec![];
    let base_dir = Directory::new("/".to_string());
    dirs.push(Rc::clone(&base_dir));
    // Check if first command is cd /
    let mut current_dir = Rc::clone(&base_dir);
    let commands = parse_commands(filename);

    for command in commands.iter() {
        match command {
            Command::Cd(dirname) => {
                if dirname == "/" {
                    continue;
                } else if dirname == ".." {
                    let mut current_parent = None;
                    if let Some(parent) = current_dir.borrow().parent() {
                        current_parent = Some(Rc::clone(&parent));
                    }
                    if let Some(parent) = current_parent {
                        current_dir = parent;
                    }
                } else {
                    // Todo: What if directory is not a subdirectory? We are not wroking with complex paths either
                    let new_current_dir = current_dir.borrow_mut().find(dirname).unwrap();
                    current_dir = new_current_dir;
                    // Todo: Directory not found
                }
            }
            Command::Ls => {}
            Command::Dir(dirname) => {
                let new_dir = current_dir
                    .borrow_mut()
                    .add_subdirectory(&dirname, &current_dir);
                dirs.push(new_dir);
            }
            Command::File(size, name) => current_dir
                .borrow_mut()
                .add_file(File::new(*size, name.clone())),
        }
    }
    dirs
}
