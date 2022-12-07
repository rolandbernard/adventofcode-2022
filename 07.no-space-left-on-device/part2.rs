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

fn size(root: &Entry) -> usize {
    match root {
        Entry::Dir { files, .. } => files.iter().map(size).sum(),
        Entry::File { size, .. } => *size,
    }
}

fn best_match(root: &Entry, to_free: usize) -> usize {
    match root {
        Entry::Dir { files, .. } => {
            let child = files.iter().map(|x| best_match(x, to_free)).min().unwrap();
            let this = size(root);
            if this > to_free && this < child {
                return this;
            } else {
                return child;
            }
        },
        Entry::File { .. } => {
            return usize::MAX;
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
    println!("Result: {}", best_match(&root, size(&root) - 40_000_000));
}
