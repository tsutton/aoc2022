use std::{collections::HashMap, fs::File, io::Read};

fn read_input() -> String {
    let mut r = String::new();
    File::open("inputs/day7.txt")
        .unwrap()
        .read_to_string(&mut r)
        .unwrap();
    r
}

#[allow(unused)]
const EXAMPLE: &str = r"$ cd /
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
7214296 k
";

struct FileSystem {
    root: Directory,
}

impl FileSystem {
    fn new() -> Self {
        Self {
            root: Directory::default(),
        }
    }

    fn get_or_create_directory(&mut self, path: &[String]) -> &mut Directory {
        let mut dir = &mut self.root;
        for p in path {
            dir = dir.subdirectories.entry(p.clone()).or_default()
        }
        dir
    }
}

struct Directory {
    subdirectories: HashMap<String, Directory>,
    files: Vec<(String, u64)>,
}

impl Directory {
    fn new() -> Self {
        Self {
            subdirectories: HashMap::new(),
            files: vec![],
        }
    }

    fn size(&self) -> u64 {
        self.files.iter().map(|(_, s)| s).sum::<u64>()
            + self.subdirectories.values().map(|d| d.size()).sum::<u64>()
    }
}

impl Default for Directory {
    fn default() -> Self {
        Self::new()
    }
}

fn parse_filesystem(s: &str) -> FileSystem {
    // track the current directory using a vec of strings
    // then we can push for relative `cd`, pop for `cd ..`, and reset for `cd /`
    let mut current_path = vec![];
    let mut lines = s.lines().peekable();

    let mut fs = FileSystem::new();

    while let Some(line) = lines.next() {
        if line.as_bytes()[0] != b'$' {
            panic!("Tried to read command but didn't begin with $: {line}");
        }

        if line.as_bytes()[2..4] == [b'c', b'd'] {
            let dir = &line.as_bytes()[5..];
            if dir == [b'.', b'.'] {
                current_path.pop();
            } else {
                current_path.push(String::from_utf8(dir.to_vec()).unwrap());
            }
        } else if line.as_bytes()[2..4] == [b'l', b's'] {
            let mut files = vec![];
            while lines
                .peek()
                .map(|line| line.as_bytes()[0] != b'$')
                .unwrap_or(false)
            {
                let file_line = lines.next().unwrap();
                let (word1, word2) = file_line.split_once(' ').unwrap();
                if word1 != "dir" {
                    let size: u64 = word1.parse().unwrap();
                    files.push((word2.to_string(), size))
                }
            }
            fs.get_or_create_directory(&current_path).files = files;
        }
    }
    fs
}

pub fn part1() {
    // let input = EXAMPLE;
    let input = read_input();

    let fs = parse_filesystem(&input);

    let mut counter = 0;
    let mut cb = |f| {
        if f < 100000 {
            counter += f
        }
    };
    walk_directory_tree(&fs.root, &mut cb, "/");
    println!("{counter}");
}

// We're going to do this recursively using a sorta visitor pattern to walk the directory tree
// for a given directory, we determine its size recursively.
// We *could* do something like: produce a tree of sizes, similar to our tree of directories.
// then walk that tree of sizes to produce the result.
// However, we can avoid defining that tree and walking it by using a callback: after every directory,
// take some action right away utilizing the callback.
fn walk_directory_tree<F>(dir: &Directory, callback: &mut F, _dir_name: &str) -> u64
where
    F: FnMut(u64),
{
    let subdir_subtotal: u64 = dir
        .subdirectories
        .iter()
        .map(|(k, d)| walk_directory_tree(d, callback, k))
        .sum();

    let total: u64 = subdir_subtotal + dir.files.iter().map(|(_, s)| s).sum::<u64>();

    callback(total);
    total
}

pub fn part2() {
    // let input = EXAMPLE;
    let input = read_input();

    let fs = parse_filesystem(&input);

    // Not sure if we can avoid walking the directory tree twice. So just call size() at the start
    let current_size = fs.root.size();

    let min_size_to_update = 30000000;
    let total_size = 70000000;
    let current_unused_space = total_size - current_size;
    let min_space_to_delete = min_size_to_update - current_unused_space;

    println!("looking for dirs whose size is at least {min_space_to_delete}");

    let mut min_found = total_size; // aka infinity: just some number bigger than any number we'll actuall get
    let mut cb = |f| {
        if f >= min_space_to_delete && f < min_found {
            min_found = f
        }
    };
    walk_directory_tree(&fs.root, &mut cb, "/");
    println!("{min_found}");
}
