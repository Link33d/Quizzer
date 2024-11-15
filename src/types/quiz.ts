export interface QuestionResponse {
    id: number,
    seed: String,
    name: String,
    alternatives: String[]
} 
  
export interface Answer {
    seed: String,
    value: number
}