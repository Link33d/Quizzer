import { useQuiz } from "../hooks/useQuiz";
import Question from "./Question";
import GameOver from "./GameOver";

export default function Game() {
    const { question, score, gameOver, loadQuestion, handleAnswer, resetGame } = useQuiz();

    return (
        <div>
            {!question ? (
                <button onClick={loadQuestion}>Start Game!</button>
            ) : !gameOver ? (
                <Question question={question} onAnswer={handleAnswer} />
            ) : (
                <GameOver score={score} onRetry={resetGame} />
            )}
        </div>
    );
}
