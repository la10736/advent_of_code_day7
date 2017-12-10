use std::io::prelude::*;

fn read_all<S: AsRef<std::path::Path>>(path: S) -> String {
    let mut content = String::new();
    let mut f = std::fs::File::open(path).unwrap();
    f.read_to_string(&mut content).unwrap();
    content
}

fn main() {
    let fname = std::env::args().nth(1).unwrap_or(String::from("example"));
    let content = read_all(fname);

    let programs = programs(&content);
    println!("Root = {:?}", root(&programs));
}

#[derive(PartialEq, Eq, Debug)]
struct Program {
    name: String,
    weight: u32,
    above: Vec<String>
}

fn sub_slice(s: &str, begin: isize, end: isize) -> &str {
    let begin = if begin >= 0 { begin as usize } else { (s.len() as isize + begin) as usize };
    let end = if end >= 0 { end as usize } else { (s.len() as isize + end) as usize };
    unsafe { s.slice_unchecked(begin, end) }
}

impl<'a> From<&'a str> for Program {
    fn from(s: &'a str) -> Self {
        let mut tokens = s.split(' ');
        let above_str = s.split(" -> ").nth(1).unwrap_or_default();
        Program {
            name: tokens.next().unwrap().into(),
            weight: sub_slice(tokens.next().unwrap(), 1, -1).parse().unwrap(),
            above: above_str.split(", ").filter_map(
                |t| if t.is_empty() { None } else { Some(t.to_string()) }
            ).collect()
        }
    }
}

use std::collections::HashMap;

fn parents<'a>(programs: &'a Vec<Program>) -> HashMap<&'a str, &'a Program> {
    programs.iter().flat_map(|p|
        p.above.iter().map(|s| s.as_ref())
            .zip(std::iter::once(p).cycle())
    ).collect()
}

fn programs<S: AsRef<str>>(data: S) -> Vec<Program> {
    data.as_ref()
        .lines()
        .map(|l| l.into())
        .collect()
}

fn root(programs: &Vec<Program>) -> &Program {
    let map = parents(&programs);

    let mut program = map.values().nth(0).unwrap();
    while let Some(parent) = map.get(program.name.as_str()) {
        program = parent;
    }
    program
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn program_from_string() {
        assert_eq!(Program { name: String::from("pbga"), weight: 66, above: vec![] },
                   Program::from("pbga (66)"));
        assert_eq!(Program {
            name: String::from("fwft"),
            weight: 72,
            above: vec!["ktlj".to_string(), "cntj".to_string(), "xhth".to_string()]
        },
                   Program::from("fwft (72) -> ktlj, cntj, xhth")
        );
    }

    static DATA: &'static str = "pbga (66)\n\
                                xhth (57)\n\
                                ebii (61)\n\
                                havc (66)\n\
                                ktlj (57)\n\
                                fwft (72) -> ktlj, cntj, xhth\n\
                                qoyq (66)\n\
                                padx (45) -> pbga, havc, qoyq\n\
                                tknk (41) -> ugml, padx, fwft\n\
                                jptl (61)\n\
                                ugml (68) -> gyxo, ebii, jptl\n\
                                gyxo (61)\n\
                                cntj (57)";

    #[test]
    fn parents_map() {
        let programs = programs(DATA);

        let map = parents(&programs);

        assert_eq!(map["ktlj"].name, "fwft");
        assert_eq!(map["padx"].name, "tknk");
    }

    #[test]
    fn find_root() {
        let programs = programs(DATA);

        assert_eq!(root(&programs).name, "tknk");
    }
}
