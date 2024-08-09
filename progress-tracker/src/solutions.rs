use git2::{Tree, TreeWalkMode, TreeWalkResult};
use std::fmt::{self, Display, Formatter};
use regex::Regex;

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Language {
    Rust = 0,
    Cpp = 1,
}

impl Language {
    pub fn list() -> [Self; 2] {
        [Self::Rust, Self::Cpp]
    }
}

impl Display for Language {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str(match self {
            Self::Rust => "Rust",
            Self::Cpp => "C++",
        })
    }
}

pub struct Solution {
    pub problem_id: String,
    pub id: String,
    pub root: String,
    pub file: String,
}


pub fn list(tree: &Tree, mut f: impl FnMut(Solution)) {
    let re = Regex::new(r"(\d+)").unwrap();

    tree.walk(TreeWalkMode::PreOrder, |root, entry| {
        let name = entry.name().unwrap();
        let problem_id = if let Some(caps) = re.captures(root) {
            caps.get(1).map(|m| m.as_str().to_string())
        } else {
            None
        };


        if let Some(problem_id) = problem_id {
            f(Solution {
                problem_id: problem_id.clone(),
                id: problem_id.clone(),
                root: root.to_string(),
                file: name.to_string(),
            });
        }

        TreeWalkResult::Ok
    })
        .unwrap();
}