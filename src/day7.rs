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

    // fn lookup_directory<P>(&self, path: &[P]) -> Option<&Directory>
    // where
    //     String: Borrow<P>,
    //     P: Hash + Eq,
    // {
    //     let mut dir = Some(&self.root);
    //     for p in path {
    //         dir = dir.and_then(|f| f.subdirectories.get(p))
    //     }
    //     dir
    // }

    // fn lookup_directory_mut<P>(&mut self, path: &[P]) -> Option<&mut Directory>
    // where
    //     String: Borrow<P>,
    //     P: Hash + Eq,
    // {
    //     let mut dir = Some(&mut self.root);
    //     for p in path {
    //         dir = dir.and_then(|f| f.subdirectories.get_mut(p))
    //     }
    //     dir
    // }

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

pub fn part1() {
    // let input = EXAMPLE;
    let input = read_input();

    let mut current_path = vec![];
    let mut lines = input.lines().peekable();

    let mut fs = FileSystem::new();

    while let Some(line) = lines.next() {
        if line.as_bytes()[0] == b'$' {
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
    }

    let mut counter = 0;
    let mut cb = |f| counter += f;
    let t = part1_walk(&fs.root, &mut cb, "/");
    println!("{counter}");
}

fn part1_walk<F>(dir: &Directory, callback: &mut F, dir_name: &str) -> u64
where
    F: FnMut(u64),
{
    let subdir_subtotal: u64 = dir
        .subdirectories
        .iter()
        .map(|(k, d)| part1_walk(d, callback, k))
        .sum();

    let total: u64 = subdir_subtotal + dir.files.iter().map(|(_, s)| s).sum::<u64>();

    if total <= 100000 {
        // println!("calling CB on {}", dir_name);
        callback(total);
    }
    total
}
