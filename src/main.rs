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
    println!("Root = {:?}", root(&programs.values().collect::<Vec<_>>()));

    let mut cache = HashMap::new();
    let wrong = programs.keys()
        .filter_map(|n| unbalanced(&programs, n, &mut cache))
        .nth(0).unwrap();

    println!("Wrong node = {:?}", wrong);

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

fn parents<'a, V: AsRef<[&'a Program]>>(programs: V) -> HashMap<&'a str, &'a Program> {
    programs.as_ref().iter().flat_map(|p|
        p.above.iter().map(|s| s.as_ref())
            .zip(std::iter::once(*p).cycle())
    ).collect()
}

fn programs<S: AsRef<str>>(data: S) -> HashMap<String, Program> {
    data.as_ref()
        .lines()
        .map(|l| {
            let p: Program = l.into();
            (p.name.to_string(), p)
        })
        .collect()
}

fn root<'a, V: AsRef<[&'a Program]>>(programs: V) -> &'a Program {
    let map = parents(&programs.as_ref());

    let mut program = map.values().nth(0).unwrap();
    while let Some(parent) = map.get(program.name.as_str()) {
        program = parent;
    }
    program
}

fn compute_weight<'a>(programs: &'a HashMap<String, Program>, pname: &'a str, cache: &mut HashMap<&'a str, u32>) -> Option<u32> {
    let p = programs.get(pname)?;

    if let Some(&w) = cache.get(pname) {
        return Some(w)
    };
    let v = p.weight + p.above.iter().map(|n| compute_weight(programs, n.as_str(), cache).unwrap()).sum::<u32>();
    cache.insert(pname, v);
    Some(v)
}

fn unbalanced<'a>(programs: &'a HashMap<String, Program>, pname: &'a str, cache: &mut HashMap<&'a str, u32>)
    -> Option<(String, u32)> {
    let p = programs.get(pname).unwrap();
    let mut m = HashMap::new();

    for (name, w) in p.above
        .iter()
        .map(|n|
            (n, compute_weight(programs, n, cache))
        ) {
        m.entry(w).or_insert(vec![]).push(name);
    }
    let expected = m.keys().filter(|w| m[w].len() > 1).nth(0);
    m.values().filter(|names| names.len() == 1).nth(0).map(|names|
                                                               (names[0].clone(), expected.unwrap().unwrap())
    )
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
        let pp = programs(DATA);
        let programs = pp.values().collect::<Vec<_>>();

        let map = parents(&programs);

        assert_eq!(map["ktlj"].name, "fwft");
        assert_eq!(map["padx"].name, "tknk");
    }

    #[test]
    fn find_root() {
        let pp = programs(DATA);
        let programs = pp.values().collect::<Vec<_>>();

        assert_eq!(root(&programs).name, "tknk");
    }

    #[test]
    fn compute_weight_should_sum_childs() {
        let programs = programs(DATA);

        let mut cache = HashMap::new();

        assert_eq!(Some(251), compute_weight(&programs, "ugml", &mut cache));
        assert_eq!(None, compute_weight(&programs, "invalid", &mut cache));
        assert_eq!(Some(243), compute_weight(&programs, "padx", &mut cache));
    }


    #[test]
    fn unbalanced_should_return_the_unbalanced_disk_if_any() {
        let programs = programs(DATA);

        let mut cache = HashMap::new();

        assert_eq!(None, unbalanced(&programs, "ugml", &mut cache));
        assert_eq!(None, unbalanced(&programs, "padx", &mut cache));
        assert_eq!(None, unbalanced(&programs, "cntj", &mut cache));
        assert_eq!(Some(("ugml".to_string(), 243)), unbalanced(&programs, "tknk", &mut cache));
    }

}
