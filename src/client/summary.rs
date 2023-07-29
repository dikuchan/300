use itertools::Itertools;

const BULLET_POINT: &str = "•";

#[derive(Debug)]
pub struct Summary {
    title: String,
    theses: Vec<String>,
}

impl Summary {
    pub fn new(title: String, theses: Vec<String>) -> Self {
        Self { title, theses }
    }

    pub fn format(&self) -> String {
        let theses = self
            .theses
            .iter()
            .map(|thesis| format!("{} {}", BULLET_POINT, thesis))
            .join("\n");
        format!("{}\n\n{}", self.title, theses)
    }
}
