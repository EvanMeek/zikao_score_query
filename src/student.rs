#[derive(Debug)]
pub struct Student {
    pub index: String,
    pub name: String,
    pub id: String,
    pub stu_id: String,
    pub score: Vec<SubjectScore>,
}
#[derive(Debug)]
pub enum SubjectScore {
    name(String),
    score(f32),
}

impl Student {
    pub fn new(index: String, name: String, id: String, stu_id: String) -> Student {
        Student {
            index,
            name,
            id: id.as_str()[7..15].into(),
            stu_id,
        }
    }
}
