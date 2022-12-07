enum Entry {
    Dir { name: String, files: Vec<Entry> },
    File { name: String, size: usize },
}

fn index<'a>(root: &'a mut Entry, path: &[String]) -> &'a mut Entry {
    if path.len() == 0 {
        return root;
    } else if let Entry::Dir { files: sub, .. } = root {
        for dir in sub {
            match dir {
                Entry::Dir { name, .. } if name == &path[0] => {
                    return index(dir, &path[1..]);
                }
                Entry::File { name, .. } if name == &path[0] => {
                    return index(dir, &path[1..]);
                }
                _ => { /* continue */ }
            }
        }
        panic!();
    } else {
        panic!();
    }
}

fn count_sizes(root: &Entry) -> (usize, usize) {
    match root {
        Entry::Dir { files, .. } => {
            let mut size = 0;
            let mut res = 0;
            for file in files {
                let (s, r) = count_sizes(file);
                size += s;
                res += r;
            }
            if size < 100_000 {
                res += size;
            }
            return (size, res);
        }
        Entry::File { size, .. } => {
            return (*size, 0);
        }
    }
}

fn main() {
    let mut path = Vec::new();
    let mut root = Entry::Dir {
        files: Vec::new(),
        name: String::new(),
    };
    let lines = std::io::stdin()
        .lines()
        .map(Result::unwrap)
        .collect::<Vec<_>>();
    let mut i = 0;
    while i < lines.len() {
        if lines[i] == "$ ls" {
            i += 1;
            let mut entries = Vec::new();
            while i < lines.len() && !lines[i].starts_with('$') {
                let split = lines[i].split(' ').collect::<Vec<_>>();
                if split[0] == "dir" {
                    entries.push(Entry::Dir {
                        name: split[1].to_owned(),
                        files: Vec::new(),
                    });
                } else {
                    entries.push(Entry::File {
                        name: split[1].to_owned(),
                        size: split[0].parse().unwrap(),
                    });
                }
                i += 1;
            }
            if let Entry::Dir { files, .. } = index(&mut root, &path) {
                files.clear();
                files.extend(entries);
            }
        } else if lines[i] == "$ cd /" {
            path.clear();
            i += 1;
        } else if lines[i] == "$ cd .." {
            path.pop();
            i += 1;
        } else if lines[i].starts_with("$ cd ") {
            path.push(lines[i][5..].to_owned());
            i += 1;
        } else {
            panic!();
        }
    }
    println!("Result: {}", count_sizes(&root).1);
}
