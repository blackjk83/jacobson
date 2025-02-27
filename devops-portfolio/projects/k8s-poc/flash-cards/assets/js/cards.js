function renderCards(flashcards, containerId) {
    const cardContainer = document.getElementById(containerId);
    
    flashcards.forEach((card, index) => {
        const cardElement = document.createElement('div');
        cardElement.className = 'card';
        cardElement.tabIndex = 0;
        cardElement.setAttribute('role', 'button');
        cardElement.setAttribute('aria-label', `Flashcard ${index + 1}`);
        
        cardElement.innerHTML = `
            <div class="card-inner">
                <div class="card-front">
                    <div>${card.question}</div>
                </div>
                <div class="card-back">
                    <div>${card.answer}</div>
                </div>
            </div>
        `;
        
        cardContainer.appendChild(cardElement);
    });
} 