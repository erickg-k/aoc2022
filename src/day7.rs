use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::io;
use std::rc::Rc;
use std::rc::Weak;
use std::vec::Vec;

#[derive(Debug)]
enum FileNode {
    Directory,
    File(u64),
}

impl fmt::Display for FileNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FileNode::Directory => write!(f, "(directory)"),
            FileNode::File(size) => write!(f, "{size}"),
        }
    }
}

#[derive(Debug)]
struct FileTreeNode {
    name: String,
    node_type: FileNode,
    parent: Weak<RefCell<FileTreeNode>>,
    children: Vec<Rc<RefCell<FileTreeNode>>>,
}

const ROOT: &str = "/";

fn build_fs_tree() -> Rc<RefCell<FileTreeNode>> {
    let root = Rc::new(RefCell::new(FileTreeNode {
        name: String::from(ROOT),
        node_type: FileNode::Directory,
        parent: Weak::new(),
        children: Vec::new(),
    }));
    let mut processing_ls_output = false;
    let mut ptr = Rc::clone(&root);

    for line in io::stdin().lines() {
        let line_str = line.expect("IO failed reading data");
        if line_str.starts_with("$ cd") {
            processing_ls_output = false;
            let cd_dir = line_str
                .as_str()
                .split_whitespace()
                .last()
                .expect("get a directory");
            if cd_dir == ROOT {
                ptr = Rc::clone(&root);
            } else if cd_dir == ".." {
                let copied_ptr = Rc::clone(&ptr);
                if let Some(p) = (*copied_ptr).borrow().parent.upgrade() {
                    ptr = Rc::clone(&p);
                };
            } else {
                let iter_ptr = Rc::clone(&ptr);
                for child_cell in &(*iter_ptr).borrow().children {
                    let child = child_cell.borrow();
                    if child.name == cd_dir {
                        ptr = Rc::clone(child_cell);
                        break;
                    }
                }
            }
        } else if line_str == "$ ls" {
            processing_ls_output = true;
        } else {
            if processing_ls_output {
                let mut splits = line_str.as_str().split_whitespace();
                let dir_or_size = splits.next().expect("Got dir or a size");
                let name = String::from(splits.next().expect("Got a name"));
                let is_dir = dir_or_size == "dir";
                let new_node = if is_dir {
                    FileTreeNode {
                        name,
                        node_type: FileNode::Directory,
                        parent: Rc::downgrade(&ptr),
                        children: Vec::new(),
                    }
                } else {
                    FileTreeNode {
                        name,
                        node_type: FileNode::File(dir_or_size.parse::<u64>().expect("Got a size")),
                        parent: Rc::downgrade(&ptr),
                        children: Vec::new(),
                    }
                };
                (*ptr)
                    .borrow_mut()
                    .children
                    .push(Rc::new(RefCell::new(new_node)));
            }
        }
    }
    return root;
}

fn print_fs_tree(ptr: Rc<RefCell<FileTreeNode>>, level: usize) {
    let node = (*ptr).borrow();
    let lead = if level == 0 { "" } else { "|" };
    let blanks = "--".repeat(level);
    let leading_chars = lead.to_string() + &blanks;
    println!("{}{} | {}", leading_chars, node.name, node.node_type);
    for child_cell in &node.children {
        print_fs_tree(Rc::clone(child_cell), level + 1);
    }
}

#[allow(dead_code, unused_imports)]
fn build_fs_tree_in_hash_map() -> HashMap<String, FileNode> {
    let mut fs = HashMap::new();
    fs.insert(String::from(ROOT), FileNode::Directory);
    let mut path_stack = Vec::new();
    path_stack.push(String::from(""));
    let mut processing_ls_output = false;

    for line in io::stdin().lines() {
        let line_str = line.expect("IO failed reading data");
        if line_str.starts_with("$ cd") {
            processing_ls_output = false;
            let cd_dir = line_str
                .as_str()
                .split_whitespace()
                .last()
                .expect("get a directory");
            if cd_dir == ROOT {
                path_stack = Vec::new();
                path_stack.push(String::from(""));
            } else if cd_dir == ".." {
                path_stack.pop();
            } else {
                path_stack.push(String::from(cd_dir));
            }
        } else if line_str == "$ ls" {
            processing_ls_output = true;
        } else {
            if processing_ls_output {
                let mut splits = line_str.as_str().split_whitespace();
                let dir_or_size = splits.next().expect("Got dir or a size");
                let name = splits.next().expect("Got a name");
                let is_dir = dir_or_size == "dir";
                path_stack.push(String::from(name));
                let path = path_stack.join("/");
                path_stack.pop();
                if is_dir {
                    fs.insert(path, FileNode::Directory);
                } else {
                    let size = dir_or_size.parse::<u64>().expect("Got a value");
                    fs.insert(path, FileNode::File(size));
                }
            }
        }
    }
    return fs;
}

pub fn sum_bound_dirs() {
    let root = build_fs_tree();

    println!("{:#?}", *root);
    print_fs_tree(root, 0);
}
