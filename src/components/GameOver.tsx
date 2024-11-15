interface Props {
    score: number;
    onRetry: () => void;
}
  
export default function GameOver({ score, onRetry }: Props) {
    return (
        <div>
            <h1 className="game-over">Game Over!</h1>
            <p className="score">Your score: {score}</p>
            <button className="retry-button" onClick={onRetry}>Try Again</button>
        </div>
    );
}
  