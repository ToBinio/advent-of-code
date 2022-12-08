use crate::advent_of_code::day::{Day, Year};

pub struct Day7;

impl Day7 {
    pub fn parse(input: &str) -> Folder {
        let mut root = Folder::new("/".to_string());

        let mut path = vec![];

        for line in input.lines().skip(1) {
            let line: Vec<_> = line.split(' ').collect();

            if line.first().unwrap() == &"$" {
                Day7::parse_command(&line, &mut path);
            } else {
                let first_split = line.first().unwrap().parse::<i32>();

                match first_split {
                    Ok(value) => {
                        root.get_inner_folder(path.clone()).add_file(File::new(line.get(1).unwrap().to_string(), value));
                    }
                    Err(_) => {
                        root.get_inner_folder(path.clone()).add_folder(Folder::new(line.get(1).unwrap().to_string()));
                    }
                }
            }
        }

        root
    }

    pub fn parse_command(command: &[&str], path: &mut Vec<String>) {
        match *command.get(1).unwrap() {
            "cd" => {
                let location = *command.get(2).unwrap();
                match location {
                    "/" => {
                        path.clear();
                        path.push("/".to_string());
                    }
                    ".." => {
                        path.pop();
                    }
                    _ => {
                        path.push(location.to_string());
                    }
                }
            }
            &_ => {}
        }
    }
}

impl Day for Day7 {
    fn get_date() -> (Year, i32) {
        (Year::Year2022, 7)
    }

    fn part_1(input: &str) -> String {
        Day7::parse(input).get_small_size().to_string()
    }

    fn part_2(input: &str) -> String {
        let root = Day7::parse(input);
        let min_size = 30000000 - (70000000 - root.get_size());

        let mut folders = vec![];

        root.get_smaller_folders(min_size, &mut folders);

        folders.sort();

        folders.first().unwrap().to_string()
    }
}

pub struct File {
    name: String,
    size: i32,
}

impl File {
    pub fn new(name: String, size: i32) -> File {
        File {
            name,
            size,
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_size(&self) -> i32 {
        self.size
    }
}

pub struct Folder {
    name: String,

    inner_folders: Vec<Folder>,
    inner_files: Vec<File>,
}

impl Folder {
    pub fn new(name: String) -> Folder {
        Folder {
            name,
            inner_folders: vec![],
            inner_files: vec![],
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_inner_folders(&self) -> &'_ Vec<Folder> {
        &self.inner_folders
    }

    pub fn get_inner_files(&self) -> &'_ Vec<File> {
        &self.inner_files
    }


    pub fn get_inner_folder(&mut self, mut name: Vec<String>) -> &mut Folder {
        if name.is_empty() {
            return self;
        }

        for inner_folder in &mut self.inner_folders {
            if &inner_folder.name == name.first().unwrap() {
                name.remove(0);
                return inner_folder.get_inner_folder(name);
            }
        }

        panic!("No folder with the name {}", name.first().unwrap())
    }

    pub fn add_file(&mut self, file: File) {
        self.inner_files.push(file);
    }

    pub fn add_folder(&mut self, folder: Folder) {
        self.inner_folders.push(folder);
    }

    pub fn get_size(&self) -> i32 {
        let mut sum = 0;

        for file in &self.inner_files {
            sum += file.get_size();
        }

        for folder in &self.inner_folders {
            sum += folder.get_size();
        }

        sum
    }

    pub fn get_small_size(&self) -> i32 {
        let mut sum = 0;

        let size = self.get_size();
        if size <= 100_000 {
            sum += size;
        }

        for folder in &self.inner_folders {
            sum += folder.get_small_size();
        }

        sum
    }

    pub fn get_smaller_folders(&self, threshold: i32, folders: &mut Vec<i32>) {
        let size = self.get_size();
        if size >= threshold {
            folders.push(size);
        }

        for folder in &self.inner_folders {
            folder.get_smaller_folders(threshold, folders);
        }
    }
}