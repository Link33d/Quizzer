import { invoke } from "@tauri-apps/api/core";
import { Answer, QuestionResponse } from "../types/quiz";

export async function getRandomQuestion(): Promise<QuestionResponse> {
  return await invoke("get_random_question", {});
}

export async function checkAnswer(data: Answer): Promise<boolean> {
  return await invoke("check_question_answer", { data });
}
