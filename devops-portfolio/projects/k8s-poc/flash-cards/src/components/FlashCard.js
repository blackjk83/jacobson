import React, { useState } from 'react';
import './FlashCard.css';

const FlashCard = ({ front, back, category }) => {
  const [isFlipped, setIsFlipped] = useState(false);

  const handleClick = () => {
    setIsFlipped(!isFlipped);
  };

  return (
    <div className={`flash-card ${isFlipped ? 'flipped' : ''}`} onClick={handleClick}>
      <div className="card-inner">
        <div className="card-front">
          <div className="category">{category}</div>
          <div className="content">{front}</div>
        </div>
        <div className="card-back">
          <div className="category">{category}</div>
          <div className="content">{back}</div>
        </div>
      </div>
    </div>
  );
};

export default FlashCard; 