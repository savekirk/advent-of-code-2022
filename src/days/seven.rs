use std::{
    cell::RefCell,
    collections::{HashMap, VecDeque},
    fmt::{self, format},
    rc::Rc,
};

type ParentDir = Rc<RefCell<Directory>>;

#[derive(Debug, Clone)]
enum Command {
    Dir(String),
    Home,
    MoveOut,
    Ls,
}

#[derive(Debug, Clone)]
struct Directory {
    name: String,
    id: String,
    parent: Option<ParentDir>,
    children: VecDeque<ParentDir>,
    files: Vec<(String, usize)>,
}

impl Directory {
    fn new(name: String, parent: Option<ParentDir>) -> Self {
        let id = parent
            .clone()
            .and_then(|d| Some(d.borrow().parent_id()))
            .or(Some(String::from("")))
            .unwrap();
        let new_id = format!("{}.{}", id, name.clone());
        Self {
            name,
            id: new_id,
            parent,
            files: vec![],
            children: VecDeque::new(),
        }
    }

    fn add_parent(&mut self, parent: Option<ParentDir>) {
        self.parent = parent;
    }

    fn add_files(&mut self, files: Vec<(String, usize)>) {
        self.files = files;
    }

    fn add_parent_and_files(&mut self, parent: Option<ParentDir>, files: Vec<(String, usize)>) {
        self.add_parent(parent);
        self.add_files(files);
    }

    fn add_child(&mut self, dir: ParentDir) {
        // if self
        //     .children
        //     .iter()
        //     .any(|d| d.borrow().name == dir.borrow().name)
        // {
        //     self.children.pop_back();
        // }
        self.children.push_back(dir)
    }

    fn parent_name(&self) -> String {
        if let Some(p) = &self.parent {
            return p.borrow().name.to_string();
        }
        String::from("")
    }

    fn parent_id(&self) -> String {
        if let Some(p) = &self.parent {
            return p.borrow().id.to_string();
        }
        String::from("")
    }

    fn dir_files_size(&self) -> usize {
        self.files.iter().map(|(_, s)| s).sum()
    }

    fn total_size(&self) -> usize {
        let mut q = VecDeque::new();
        q.push_back(Rc::new(RefCell::new(self.to_owned())));

        let mut total = self.dir_files_size();

        while let Some(d) = q.pop_front() {
            d.borrow().children.iter().for_each(|d| {
                let cd = Rc::clone(&d);
                q.push_back(cd);
                total = total + d.borrow().dir_files_size();
            });
        }
        total
    }

    fn dirs(&self) -> Vec<String> {
        self.children.iter().fold(Vec::new(), |mut acc, d| {
            acc.push(d.borrow().name.to_string());
            acc
        })
    }
}

impl PartialEq for Directory {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl fmt::Display for Directory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Name: {}, Id: {}, Size: {}, Parent: {:?}. Children: {:?}, Files: {:?}",
            self.name,
            self.id,
            self.total_size(),
            self.parent_name(),
            self.dirs(),
            self.files
        )
    }
}

pub fn part_1(lines: Vec<String>) -> usize {
    // let mut level = 0;
    let root = Rc::new(RefCell::new(Directory::new(String::from("/"), None)));
    let mut cd = Rc::clone(&root);
    let mut dc = 1;
    let mut level = 0;
    // let mut line_iter = lines.into_iter();
    // line_iter.next();
    let mut files = Vec::<(String, usize)>::new();
    for line in lines {
        let mut components = line.split_ascii_whitespace();
        let start = components.next().unwrap();
        match start {
            "$" => {
                if !&files.is_empty() {
                    let mut parent = None;
                    if let Some(p) = &cd.borrow().parent {
                        let cloned_parent = Rc::clone(p);
                        cloned_parent.borrow_mut().add_child(Rc::clone(&cd));
                        parent = Some(cloned_parent);
                    }
                    cd.borrow_mut().add_parent_and_files(parent, files.clone());
                    // cd.borrow_mut().add_files(files.clone());
                    files.clear();
                }
                components.next();
                let param = components.next();
                let command = parse_command(param);
                match command {
                    Command::Dir(name) => {
                        dc = dc + 1;
                        level = level + 1;
                        let dir = Rc::new(RefCell::new(Directory::new(
                            name.to_string(),
                            Some(Rc::clone(&cd)),
                        )));
                        cd = Rc::clone(&dir)
                    }
                    Command::Home => {
                        continue;
                    }
                    Command::MoveOut => {
                        level = level - 1;
                        let cd_clone = Rc::clone(&cd);
                        cd = Rc::clone(cd_clone.borrow().parent.as_ref().unwrap());
                    }
                    Command::Ls => {
                        files.clear();
                    }
                }
            }
            "dir" => {}
            _ => {
                let file = (
                    components.next().unwrap().to_string(),
                    start.parse::<usize>().unwrap(),
                );
                files.push(file);
            }
        }
    }

    let mut total = 0;
    let mut td = 0;

    let mut q = VecDeque::new();
    q.push_back(Rc::clone(&root));

    // total = home.borrow().dir_files_size();
    println!("home size => {}", root.borrow().children.len());

    while let Some(d) = q.pop_front() {
        d.borrow().children.iter().for_each(|d| {
            let cd = Rc::clone(&d);
            q.push_back(cd);
            td = td + 1;
            let size = d.borrow().total_size();
            //  println!("size => {}", size);
            if size <= 100000 {
                total = total + size;
            }
        });
    }

    println!("DC => {}, td => {}", dc, td);

    total
}

pub fn part_2(lines: Vec<String>) -> i32 {
    todo!()
}

fn parse_command<'a>(param: Option<&'a str>) -> Command {
    // let mut components = line.split_ascii_whitespace();
    // components.next_chunk::<2>();
    // components.next().unwrap();
    // let param = components.next();
    if let Some(param) = param {
        match param {
            "/" => Command::Home,
            ".." => Command::MoveOut,
            _ => Command::Dir(param.to_string()),
        }
    } else {
        Command::Ls
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_command() {
        let s = vec![Some("/"), None, Some("a"), Some("..")];
        s.iter().for_each(|c| {
            let p = parse_command(*c);
            println!("P => {:?}", p);
        });
    }
}
