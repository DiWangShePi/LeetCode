use git2::Repository;
use http::header;
use problems::Problems;
use reqwest::blocking::Client;
use std::path::Path;
use std::{env, fs};

mod html;
mod problems;
mod progress_chart;
mod report;
mod solutions;

fn get_all_problems() -> Problems {
    Client::new()
        .get("https://leetcode.com/api/problems/algorithms/")
        .header(header::COOKIE, "LEETCODE_SESSION=")
        .send()
        .unwrap()
        .json()
        .unwrap()
}

fn main() {
    let repository = Path::new("..");  // 上级目录作为 Git 仓库
    let target = Path::new("../docs");  // 上级目录的 docs 文件夹作为输出

    let target_path = Path::new(&target);
    fs::create_dir_all(target_path).unwrap();

    let repository_path = Path::new(&repository);
    let repository = Repository::open(repository_path).unwrap();

    let mut problems = get_all_problems();
    problems.retain_free();
    problems.problems.sort_by_key(|p| p.stat.frontend_question_id);

    // Generate progress chart
    progress_chart::draw(&repository, &problems, &target_path.join("progress.svg"));

    // Generate HTML report
    let tree = repository.head().unwrap().peel_to_tree().unwrap();
    report::generate(
        &problems.problems,
        &tree,
        "progress.svg",
        &target_path.join("index.html")
    );
}