import { useState } from "react";
import { getRandomQuestion, checkAnswer } from "../services/commands";
import { QuestionResponse, Answer } from "../types/quiz";

export function useQuiz() {
  const [question, setQuestion] = useState<QuestionResponse | null>(null);
  const [score, setScore] = useState<number>(0);
  const [gameOver, setGameOver] = useState<boolean>(false);

  async function loadQuestion() {
    const question: QuestionResponse = await getRandomQuestion();
    setQuestion(question);
  }

  async function handleAnswer(value: number) {
    if (question) {
      const isCorrect = await checkAnswer({ seed: question.seed, value } as Answer);
      if (isCorrect) {
        setScore(prev => prev + 1);
        loadQuestion();
      } else {
        setGameOver(true);
      }
    }
  }

  function resetGame() {
    setScore(0);
    setGameOver(false);
    loadQuestion();
  }

  return { question, score, gameOver, loadQuestion, handleAnswer, resetGame };
}
