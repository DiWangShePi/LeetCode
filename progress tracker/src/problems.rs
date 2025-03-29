use serde::Deserialize;

#[derive(Deserialize)]
pub struct Stat {
    #[serde(rename = "question__title")]
    pub title: String,
    #[serde(rename = "question__title_slug")]
    pub title_slug: String,
    pub frontend_question_id: u16,
}

#[derive(Deserialize)]
pub struct Difficulty {
    pub level: u8,
}

#[derive(Deserialize)]
pub struct Problem {
    pub stat: Stat,
    pub difficulty: Difficulty,
    pub paid_only: bool,
}

impl Problem {
    pub fn get_id(&self) -> String {
        format!("{:04}-{}", self.stat.frontend_question_id, self.stat.title_slug)
    }
}

#[derive(Deserialize)]
pub struct Problems {
    #[serde(rename = "stat_status_pairs")]
    pub problems: Vec<Problem>,
}

impl Problems {
    pub fn retain_free(&mut self) {
        self.problems.retain(|problem| !problem.paid_only);
    }
}
