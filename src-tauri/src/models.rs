use serde::{Deserialize, Serialize};

#[derive(Serialize,  Deserialize)]
pub struct Quiz {
    pub quiz_name: String,
    pub questions: Vec<Question>,
}

#[derive(Serialize, Deserialize)]
pub struct Question {
    pub question: String,
    pub alternatives: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QuestionResponse {
    pub id: i32,
    pub seed: String,
    pub name: String,
    pub alternatives: Vec<String>,
} 

#[derive(Serialize, Deserialize)]
pub struct Answer {
    pub seed: String,
    pub value: u64
}