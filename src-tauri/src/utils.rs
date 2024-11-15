use crate::models;

use std::fs;
use std::io::Read;
use std::hash::{BuildHasher, Hasher, RandomState};

pub fn get_data() -> Result<models::Quiz, Box<dyn std::error::Error>> {

    let mut file = fs::File::open("../public/data.json")?; // Aqui, o ? irá propagar o erro se ocorrer
    let mut data_json = String::new();
    
    file.read_to_string(&mut data_json)?; // Propaga o erro de leitura

    let quiz: models::Quiz = serde_json::from_str(&data_json)?; // Propaga o erro de deserialização

    Ok(quiz) // Retorna o quiz se tudo correr bem

}

pub fn rand() -> u64 {
    RandomState::new().build_hasher().finish()
}

pub fn shuffle<T>(seed: u64, vec: &mut [T]) {
    let n: usize = vec.len();
    for i in (0..n).rev() {
        let j: usize = (seed as usize) % (i + 1);
        vec.swap(i, j);
    }
}

pub fn get_int_array(seed: u64) -> Vec<u32> {
    let mut integers: Vec<u32> = (0..=3).collect();

    shuffle(seed, &mut integers);

    return integers;
}