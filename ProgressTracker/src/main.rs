/*
Credit to:
https://github.com/EFanZh/LeetCode/tree/master
https://efanzh.org/LeetCode/

I appreciate this guy's reports, which exude a clean, cool aesthetic and a distinctly geeky vibe.
*/


use git2::Repository;
use http::header;
use problems::Problems;
use reqwest::blocking::Client;
use std::path::Path;
use std::{env, fs};
use std::fs::File;
use std::io::Read;
use serde::Deserialize;
use serde_json;

mod problems;
mod progress_chart;
mod solutions;

fn get_all_problems() -> Problems {
    Client::new()
        .get("https://leetcode.com/api/problems/algorithms/")
        .header(header::COOKIE, "LEETCODE_SESSION=")
        .send()
        .unwrap()
        .json()
        .unwrap()

    // In case the above code can not work (due to network issue)
    // let mut file = File::open("./src/problems.json").expect("Unable to open local problem file");
    // let mut data = String::new();
    // file.read_to_string(&mut data).expect("Unable to read local problem file");
    // serde_json::from_str(&data).expect("Unable to parse local problem file into JSON format")
}

fn main() {
    // receive environment args
    let mut args = env::args();
    args.next().unwrap();
    let repository= args.next().unwrap();
    let target= args.next().unwrap();

    // create target folder
    let target_path = Path::new(&target);
    fs::create_dir_all(target_path).unwrap();

    // retrieve repository information
    let repository_path = Path::new(&repository);
    let repository = Repository::open(repository_path).unwrap();

    // get all leetcode problems
    let mut problems = get_all_problems();
    problems.retain_free();
    problems.problems.sort_by_key(|p| p.stat.frontend_question_id);

    // Generate progress chart.
    progress_chart::draw(&repository, &problems, &target_path.join("progress.svg"));
}
