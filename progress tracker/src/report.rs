use crate::solutions;
use crate::problems::Problem;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn make_solution_map(tree: &git2::Tree) -> HashMap<String, Vec<solutions::Solution>> {
    let mut result = HashMap::new();

    solutions::list(tree, |solution| {
        result.entry(solution.problem_id.clone())
            .or_insert_with(Vec::new)
            .push(solution);
    });

    result
}

fn convert_problem_id(problem_id: &str) -> String {
    let force_upper = ["II", "III", "IV", "VI", "VII", "VIII", "IX", "X"];

    let parts: Vec<&str> = problem_id.splitn(2, '-').collect();
    let number = parts[0].trim_start_matches('0');
    let title = parts.get(1).unwrap_or(&"");
    let title = title
        .split('-')
        .map(|s| {
            if force_upper.contains(&s.to_ascii_uppercase().as_str()) {
                s.to_ascii_uppercase()
            } else {
                let mut c = s.chars();
                match c.next() {
                    None => String::new(),
                    Some(f) => f.to_uppercase().chain(c).collect(),
                }
            }
        })
        .collect::<Vec<_>>()
        .join("-");

    format!("{}.{}", number, title)
}


fn get_problem_id(problem_id: &str) -> String {
    let parts: Vec<&str> = problem_id.split('-').collect();
    if parts.len() >= 2 {
        let id = parts[0].trim_start_matches('0');
        format!("{}", id)
    } else {
        problem_id.to_string()
    }
}

fn write_html(writer: &mut impl Write, problems: &[Problem], tree: &git2::Tree, progress_chart: &str) {
    let solution_map = make_solution_map(tree);

    writeln!(writer, r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <link rel="icon" href="https://github.com/DiWangShePi/LeetCode/blob/main/docs/icon/favicon.ico" type="image/x-icon">
    <title>LeetCode Progress Report</title>
    <style>
        body {{
            font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif;
            line-height: 1.6;
            color: #333;
            background-color: #fff;
            margin: 0;
            padding: 0;
        }}
        .container {{
            max-width: 850px;
            margin: 0 auto;
            padding: 2rem;
        }}
        h1 {{
            font-size: 2rem;
            font-weight: normal;
            margin-bottom: 1.5rem;
            color: #222;
            display: flex;
            align-items: center;
            justify-content: center;
            gap: 0.5rem;
        }}
        h1 .red {{
            color: #b30000;
        }}
        h2 {{
            font-size: 1.5rem;
            font-weight: normal;
            margin: 2rem 0 1rem;
            color: #555;
            display: flex;
            align-items: center;
            gap: 10px;
        }}
        .progress-chart {{
            margin: 2rem 0;
            text-align: center;
        }}
        .progress-chart img {{
            max-width: 100%;
            height: auto;
            width: 800px;
        }}
        table {{
            width: 100%;
            border-collapse: collapse;
            margin: 2rem 0;
            font-size: 0.85rem;
        }}
        th, td {{
            padding: 0.4rem 0.6rem;
            border-bottom: none;
        }}
        td:first-child, th:first-child {{
            border-right: 2px solid #ddd;
            padding-right: 0.8rem;
        }}
        th {{
            color: #555;
            font-weight: normal;
            padding-bottom: 0.5rem;
            text-align: left;
        }}
        td:nth-child(3), th:nth-child(3),
        td:nth-child(4), th:nth-child(4) {{
            text-align: center;
        }}
        .difficulty {{
            display: inline-block;
            padding: 0.1rem 0;
            font-size: 0.85rem;
            color: #666;
        }}
        a {{
            color: #b30000;
            text-decoration: none;
        }}
        a:hover {{
            text-decoration: underline;
            color: #8c0000;
        }}
        hr {{
            border: 0;
            height: 1px;
            background: #eee;
            margin: 1rem 0;
        }}
        .title-separator {{
            border: 0;
            height: 1px;
            background: #ddd;
            margin: 0.5rem 0 1.5rem;
        }}
        tr.no-solution {{
            opacity: 0.6;
            filter: grayscale(50%);
        }}
        tr.no-solution a {{
            color: #999 !important;
        }}
        tr.no-solution .difficulty {{
            opacity: 0.7;
        }}
        .random-btn {{
            background: none;
            border: none;
            cursor: pointer;
            font-size: 1.2rem;
            color: #b30000;
            padding: 0;
            margin: 0;
            line-height: 1;
        }}
        .random-btn:hover {{
            color: #8c0000;
        }}
    </style>
</head>
<body>
    <div class="container">
        <h1>
            <span class="red">LeetCode</span>
            <span>Progress Report</span>
        </h1>

        <hr class="title-separator">

        <div class="progress-chart">
            <img src="{}" alt="Progress Chart">
        </div>

        <hr>

        <h2 style="display: flex; justify-content: space-between; align-items: center;">
            Detail
            <div style="display: flex; align-items: center;">
                <button class="random-btn" title="Jump to a random solved problem" onclick="jumpToRandomSolvedProblem()">
                    <img src="./icon/math-random.svg" alt="Random Pick Icon" style="width: 20px; height: 20px;">
                </button>
                <span style="font-size: 12px; margin-left: 5px;">Random pick</span>
            </div>
        </h2>

        <table>
            <thead>
                <tr>
                    <th>ID</th>
                    <th>Title</th>
                    <th>Difficulty</th>
                    <th>C++ Solutions</th>
                </tr>
            </thead>
        <tbody>"#, progress_chart).unwrap();

    for problem in problems {
        let problem_id = get_problem_id(&problem.get_id());
        let has_solution = solution_map.contains_key(&problem_id);

        writeln!(writer, r#"<tr class="{}">"#, if has_solution { "" } else { "no-solution" }).unwrap();

        writeln!(writer, r#"
                <td>{}</td>
                <td><a href="https://leetcode.com/problems/{}/">{}</a></td>
                <td><span class="difficulty">{}</span></td>
                <td>"#,
            problem.stat.frontend_question_id,
            problem.stat.title_slug,
            problem.stat.title,
            "⚠".repeat(problem.difficulty.level as usize)
        ).unwrap();

        if has_solution {
            let github_link = format!(
                "https://github.com/DiWangShePi/LeetCode/tree/main/solutions/{}/README.md",
                convert_problem_id(&problem.get_id())
            );
            writeln!(writer, r#"<a href="{}" target="_blank">Solution</a>"#, github_link).unwrap();
        } else {
            writeln!(writer, "-").unwrap();
        }

        writeln!(writer, "</td></tr>").unwrap();
    }

    writeln!(writer, r#"</tbody>
            </table>
        </div>

        <script>
            function jumpToRandomSolvedProblem() {{
                const solvedRows = document.querySelectorAll('tbody tr:not(.no-solution)');
                
                if (solvedRows.length === 0) {{
                    alert('No solved problems found!');
                    return;
                }}

                const randomIndex = Math.floor(Math.random() * solvedRows.length);
                const randomRow = solvedRows[randomIndex];
                const problemLink = randomRow.querySelector('td:nth-child(4) a');

                if (problemLink) {{
                    window.open(problemLink.href, '_blank');
                }}
            }}
        </script>
        </body>
        </html>"#).unwrap();

}


pub fn generate(problems: &[Problem], tree: &git2::Tree, progress_chart: &str, output: &Path) {
    let mut file = File::create(output).unwrap();
    write_html(&mut file, problems, tree, progress_chart);
}