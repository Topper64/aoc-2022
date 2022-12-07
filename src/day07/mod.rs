use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug)]
struct ParseError;

#[derive(Debug, Clone, PartialEq)]
enum Cmd {
    CdRoot,
    CdUp,
    Cd(String),
    Ls,
}

impl FromStr for Cmd {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let command = s.strip_prefix("$ ").ok_or(ParseError)?;
        match command.split_once(' ').unwrap_or_else(|| (command, "")) {
            ("cd", "/") => Ok(Cmd::CdRoot),
            ("cd", "..") => Ok(Cmd::CdUp),
            ("cd", args) if args.len() > 0 => Ok(Cmd::Cd(String::from(args))),
            ("ls", "") => Ok(Cmd::Ls),
            _ => Err(ParseError),
        }
    }
}

#[derive(Debug)]
struct File {
    name: String,
    size: usize,
    contents: Option<Listing>,
}

#[derive(Debug, Default)]
struct Listing(HashMap<String, File>);

impl File {
    pub fn calc_size(&mut self) -> usize {
        if let Some(listing) = &mut self.contents {
            self.size = listing.0.values_mut().fold(0, |s, f| s + f.calc_size());
        }
        self.size
    }
}

impl FromStr for File {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (ft, name) = s.split_once(' ').ok_or(ParseError)?;

        let (size, contents) = match ft.parse() {
            Ok(n) => (n, None),
            _ if ft == "dir" => (0, Some(Default::default())),
            _ => return Err(ParseError),
        };

        Ok(File {
            name: String::from(name),
            size,
            contents,
        })
    }
}

fn parse_input(input: &str) -> File {
    let mut prev_cmd = None;
    let mut root = File {
        name: String::from(""),
        size: 0,
        contents: Some(Default::default()),
    };

    // Store a list of all directory names so far, and the current directory.
    // Ideally, this would just be a list of directories, but they all need
    // to be &mut, but then we can't do anything else with them...  Could get
    // around it with Rc<RefCell<File>> but that seems overkill.
    let mut dirs: Vec<String> = vec![];
    let mut cwd = &mut root;

    for line in input.lines() {
        if let Ok(cmd) = Cmd::from_str(line) {
            match cmd {
                Cmd::CdRoot => {
                    dirs.clear();
                    cwd = &mut root;
                }
                Cmd::CdUp => {
                    dirs.pop().unwrap();
                    // Cd back into the right directory starting from root
                    cwd = &mut root;
                    for name in dirs.iter() {
                        cwd = cwd.contents.as_mut().unwrap().0.get_mut(name).unwrap();
                    }
                }
                Cmd::Cd(ref name) => {
                    dirs.push(name.clone());
                    let listing = match cwd.contents {
                        Some(ref mut listing) => listing,
                        None => panic!("cannot cd"),
                    };
                    cwd = listing.0.get_mut(name).unwrap();
                }
                _ => (),
            };
            prev_cmd = Some(cmd);
        } else if prev_cmd == Some(Cmd::Ls) {
            let file = File::from_str(line).expect("expected file listing");
            cwd.contents
                .as_mut()
                .unwrap()
                .0
                .insert(file.name.clone(), file);
        }
    }

    root.calc_size();

    root
}

pub fn main() {}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = include_str!("test.txt");

    #[test]
    fn test_size() {
        assert_eq!(parse_input(INPUT).size, 48381165);
    }
}
