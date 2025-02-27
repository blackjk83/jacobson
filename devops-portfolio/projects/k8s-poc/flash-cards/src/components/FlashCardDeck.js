import React, { useState } from 'react';
import FlashCard from './FlashCard';
import './FlashCardDeck.css';

const FlashCardDeck = ({ cards }) => {
  const [currentIndex, setCurrentIndex] = useState(0);

  const nextCard = () => {
    setCurrentIndex((prevIndex) => (prevIndex + 1) % cards.length);
  };

  const prevCard = () => {
    setCurrentIndex((prevIndex) => (prevIndex - 1 + cards.length) % cards.length);
  };

  return (
    <div className="flash-card-deck">
      <div className="deck-controls">
        <button onClick={prevCard}>Previous</button>
        <span className="card-counter">{currentIndex + 1} / {cards.length}</span>
        <button onClick={nextCard}>Next</button>
      </div>
      <FlashCard {...cards[currentIndex]} />
    </div>
  );
};

export default FlashCardDeck; 