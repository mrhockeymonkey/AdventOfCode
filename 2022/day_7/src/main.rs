use std::cell::RefCell;
use std::cmp::min;
use std::collections::HashMap;
use std::fmt;
use std::fmt::Formatter;
use std::rc::Rc;
use camino::Utf8PathBuf;
use id_tree::{InsertBehavior, Node, Tree};
use indexmap::IndexMap;
use nom::branch::alt;
use nom::bytes::complete::take_while1;
use nom::combinator::{all_consuming, map};
use nom::bytes::complete::tag;
use nom::error::Error;
use nom::{Finish, IResult};
use nom::sequence::{preceded, separated_pair};



fn main() -> color_eyre::Result<()>  {
    color_eyre::install().unwrap();
    
    let lines = include_str!("sample-input.txt")
        .lines()
        .map(|l| all_consuming(parse_line)(l).finish().unwrap().1);
    
    let mut tree = Tree::<FsEntry>::new();
    let root = tree.insert(
        Node::new(FsEntry{path: "/".into(), size: 0}),
        InsertBehavior::AsRoot,
    )?;
    let mut curr = root;
    
    for line in lines {
        println!("{line:?}");
        match line {
            Line::Command(cmd) => match cmd {
                Command::Ls => {
                    // ignore
                },
                Command::Cd(path) => match path.as_str() {
                    "/" => {
                        // ignore
                    },
                    ".." => {
                        curr = tree.get(&curr)?.parent().unwrap().clone();
                    },
                    _ => {
                        let node = Node::new(FsEntry{path: path.clone(), size: 0});
                        curr = tree.insert(node, InsertBehavior::UnderNode(&curr))?;
                    }
                }
            },
            Line::Entry(entry) => match entry {
                Entry::Dir(dir) => {
                    // ignore, we do this when we cd into them
                },
                Entry::File(size, file) => {
                    let node = Node::new(FsEntry{path: file, size });
                    tree.insert(node, InsertBehavior::UnderNode(&curr))?;
                }
            }
        }
    }

    let mut s = String::new();
    tree.write_formatted(&mut s)?;
    println!("{s}");

    

    // part 1
    let sum = tree
        .traverse_pre_order(tree.root_node_id().unwrap())? // traverse tree
        .filter(|n| !n.children().is_empty())// only dirs
        .map(|n| total_size(&tree, n).unwrap())
        .filter(|&s| s <= 100_000)
        .inspect(|s| {
            dbg!(s);
        })
        .sum::<u64>();
    dbg!(sum);

    // part 2    
    let total_space = 70_000_000_u64;
    let used_space = total_size(&tree, tree.get(tree.root_node_id().unwrap())?)?;
    let free_space = total_space.checked_sub(dbg!(used_space)).unwrap();
    let needed_space = 30_000_000_u64;
    let min_space_to_clear = needed_space.checked_sub(free_space).unwrap();
    dbg!(min_space_to_clear);
    
    let removed_dir_size = tree
        .traverse_pre_order(tree.root_node_id().unwrap())?
        .filter(|n| !n.children().is_empty())
        .map(|n| total_size(&tree, n).unwrap())
        .filter(|&s| s >= min_space_to_clear)
        .inspect(|s| {
            dbg!(s);
        })
        .min();
    
    dbg!(removed_dir_size);

    Ok(())
}

// id_tree impl

#[derive(Debug)]
struct FsEntry {
    path: Utf8PathBuf,
    size: u64,
}

fn total_size(tree: &Tree<FsEntry>, node: &Node<FsEntry>) -> color_eyre::Result<u64> {
    let mut total = node.data().size;
    for child in node.children() {
        total += total_size(tree, tree.get(child)?)?;
    }
    Ok(total)
}

// homebrew tree impl

fn old_main() {
    let lines = include_str!("input.txt")
        .lines()
        .map(|l| all_consuming(parse_line)(l).finish().unwrap().1);

    let root =  Rc::new(RefCell::new(FsNode::default()));
    let mut node = root.clone();

    for line in lines {
        println!("{line:?}");
        match line {
            Line::Command(cmd) => match cmd {
                Command::Ls => {
                    // ignore
                },
                Command::Cd(path) => match path.as_str() {
                    "/" => {
                        // ignore
                    },
                    ".." => {
                        let parent = node.borrow().parent.clone().unwrap();
                        node = parent
                    },
                    _ => {
                        let child = node.borrow_mut().children.entry(path).or_default().clone();
                        node = child
                    }
                }
            },

            Line::Entry(entry) => match entry {
                Entry::Dir(dir) => {
                    let entry = node.borrow_mut().children.entry(dir).or_default().clone();
                    entry.borrow_mut().parent = Some(node.clone());
                },
                Entry::File(size, file) => {
                    let entry = node.borrow_mut().children.entry(file).or_default().clone();
                    entry.borrow_mut().size = size as usize;
                    entry.borrow_mut().parent = Some(node.clone());
                }
            }
        }
    }

    println!("{:?}", PrettyNode(&root));

    // part 1
    // let sum = all_dirs(root)
    //     .map(|d| d.borrow().total_size())
    //     .filter(|&s| s <= 100_000)
    //     .inspect(|s| {
    //         dbg!(s);
    //     })
    //     .sum::<u64>();
    // dbg!(sum);

    let total_space = 70_000_000_u64;
    let used_space = root.borrow().total_size();
    let free_space = total_space.checked_sub(dbg!(used_space)).unwrap();
    let needed_space = 30_000_000_u64;
    let min_space_to_clear = needed_space.checked_sub(free_space).unwrap();
    dbg!(min_space_to_clear);

    let removed_dir_size = all_dirs(root)
        .map(|dir| dir.borrow().total_size())
        .filter(|&size| size >= min_space_to_clear)
        .inspect(|&s| {
            dbg!(s);
        })
        .min();

    dbg!(removed_dir_size);

    //let x = parse_cd("cd /foo");
    //println!("Hello, world!");

}

type NodeHandle = Rc<RefCell<FsNode>>;

fn all_dirs(n: NodeHandle) -> Box<dyn Iterator<Item = NodeHandle>> {
    let children = n.borrow().children.values().cloned().collect::<Vec<_>>();
    
    Box::new(
    std::iter::once(n.clone()).chain(
        children
            .into_iter()
            .filter_map(|c| {
                if c.borrow().is_dir() {
                    Some(all_dirs(c))
                } else {
                    None
                }
            })
            .flatten(),
    ))
}

struct PrettyNode<'a>(&'a NodeHandle);

impl<'a> fmt::Debug for PrettyNode<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let this = self.0.borrow();
        if this.size == 0 {
            writeln!(f, "(dir)");
        } else {
            writeln!(f, "(file, size={})", this.size);
        }

        for (name, child) in &this.children {
            // not very efficient at all, but shrug
            for (index, line) in format!("{:?}", PrettyNode(child)).lines().enumerate() {
                if index == 0 {
                    writeln!(f, "{name} {line}")?;
                } else {
                    writeln!(f, "  {line}")?;
                }
            }
        }
        
        Ok(())
    }
}

#[derive(Default)]
struct FsNode {
    size: usize,
    children: IndexMap<Utf8PathBuf, NodeHandle>,
    parent: Option<NodeHandle>,
}

impl FsNode {
    fn is_dir(&self) -> bool {
        self.size == 0 && !self.children.is_empty()
    }
    
    fn total_size(&self) -> u64 {
        self.children.values().map(|x| x.borrow().total_size()).sum::<u64>() + self.size as u64
    }
}

impl std::fmt::Debug for FsNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Node")
            .field("size", &self.size)
            .field("children", &self.children)
            .finish()
    }
}


// parsing 
fn parse_path(i: &str) -> IResult<&str, Utf8PathBuf> {
    map(
        take_while1(|c: char| "abcdefghijklmnopqrstuvwxyz./".contains(c)),
        Into::into // implicit convert from IResult<&str,&str> -> IResult<&str, Uft8PathBuf> 
    )(i)
}

#[derive(Debug)]
struct Ls;

fn parse_ls(i: &str) -> IResult<&str, Ls> {
    map(tag("ls"), |_| Ls)(i)
}

#[derive(Debug)]
struct Cd(Utf8PathBuf);

fn parse_cd(i: &str) -> IResult<&str, Cd> {
    map(preceded(tag("cd "), parse_path), Cd)(i)
}

#[derive(Debug)]
enum Command {
    Ls,
    Cd(Utf8PathBuf),
}

impl From<Ls> for Command {
    fn from(value: Ls) -> Self {
        Command::Ls
    }
}

impl From<Cd> for Command {
    fn from(value: Cd) -> Self {
        Command::Cd(value.0)
    }
}

fn parse_command(i: &str) -> IResult<&str, Command> {
    let (i, ii) = tag("$ ")(i)?;
    let ls_command = map(parse_ls, Into::into);
    let cd_command = map(parse_cd, Into::into);
    alt((ls_command, cd_command))(i)
}

#[derive(Debug)]
enum Entry {
    Dir(Utf8PathBuf),
    File(u64, Utf8PathBuf)
}

fn parse_entry(i: &str) -> IResult<&str, Entry> {
    let parse_file = map(
        separated_pair(nom::character::complete::u64, tag(" "), parse_path),
        |(size, file)| Entry::File(size, file)
    );
    
    let parse_dir = map(
        preceded(tag("dir "), parse_path),
        |path| Entry::Dir(path)
    );
    
    alt((parse_file, parse_dir))(i)
}

#[derive(Debug)]
enum Line {
    Command(Command),
    Entry(Entry),
}

fn parse_line(i: &str) -> IResult<&str, Line> {
    alt((
        map(parse_command, Line::Command),
        map(parse_entry, Line::Entry))
    )(i)
}