import { QuestionResponse } from "../types/quiz";

interface Props {
    question: QuestionResponse;
    onAnswer: (value: number) => void;
}

export default function Question({ question, onAnswer }: Props) {
    return (
        <div className="question-container">
            <h1>{question.name}</h1>
            {question.alternatives.map((alternative, index) => (
                <button className="alternative-button" key={index} onClick={() => onAnswer(index)}>
                    {alternative}
                </button>
            ))}
        </div>
    );
}
