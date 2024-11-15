use crate::models;
use crate::utils;

pub fn get_random_question() -> models::QuestionResponse {
    
    let quiz: models::Quiz = match utils::get_data() {
        Ok(q) => q,
        Err(_) => {
            return models::QuestionResponse {
                id: -1,
                seed: "0".to_string(),
                name: "Error reading json file!".to_string(), 
                alternatives: vec![],
            }; 
        }
    };

    let seed: u64 = utils::rand();

    let random_index: usize = (seed % quiz.questions.len() as u64) as usize;

    let selected_question: &models::Question = &quiz.questions[random_index];

    let integers: Vec<u32> = utils::get_int_array(seed);

    let mut alternatives: Vec<String> = Vec::with_capacity(4);

    for i in 0..=3 {
        alternatives.push(selected_question.alternatives[integers[i] as usize].clone());
    }

    return models::QuestionResponse {
        id: random_index as i32,
        seed: seed.to_string(),
        name: selected_question.question.clone(),
        alternatives: alternatives,
    };
}

pub fn check_question_answer(data: models::Answer) -> bool {
    
    let seed: u64 = data.seed.parse::<u64>().unwrap_or(0);

    let integers: Vec<u32> = utils::get_int_array(seed);

    let correct_answer: Option<usize> = integers.iter().position(|&x| x == 0);

    if Some(correct_answer).is_none() {
        return false;
    }

    return data.value == correct_answer.unwrap() as u64;
}